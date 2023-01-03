mod generator;
mod searcher;
mod types;
mod ui;
mod utils;

use std::{
    fs::File,
    io::BufReader,
    ops::RangeInclusive,
    path::{Path, PathBuf},
    sync::Arc,
    thread,
};

use clap::{ArgGroup, Parser, Subcommand};
use ethers::abi::Abi;
use eyre::Result;
use generator::Generator;
use ptree::print_tree;
use searcher::Searcher;
use types::FunctionSignature;
use ui::GroupedSelector;
use utils::create_progress_bar;

const SELECTOR_LENGTH_RANGE: RangeInclusive<u8> = 1..=4;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[clap(group(
        ArgGroup::new("name")
            .required(true)
            .multiple(false)
            .args(&["abi","signature"])
    ))]
    /// generates function signatures with optimal zeroes in selector
    Generate {
        /// Loads function signatures from ABI file
        #[arg(short, long, value_name = "FILE", value_parser = abi_file_exists)]
        abi: Option<PathBuf>,

        /// Function signature to optimize
        #[arg(short, long, value_parser = parse_signature)]
        signature: Option<FunctionSignature>,

        /// Number of bytes to suffix the function name with
        #[arg(short = 'l', long, value_name = "NUMBER", default_value_t = 4)]
        suffix_length: u8,

        /// Minimum number of bytes that are to be zeroed out in the selector
        #[arg(short, long, value_name = "NUMBER", value_parser = zero_bytes_within_selector_length, default_value_t = 2)]
        zero_bytes: u8,
    },
    // TODO: rewrites function signature in solidity files
    // Fmt {},
}

// Handlers

fn generate(
    abi: Option<PathBuf>,
    signature: Option<FunctionSignature>,
    suffix_length: u8,
    zero_bytes: u8,
) -> Result<()> {
    let signatures = get_signatures_from(abi, signature)?;

    for signature in signatures {
        let signature_arc = Arc::new(signature.clone());

        let num_searchers = thread::available_parallelism()
            .map(|x| x.get() - 1) // one thread in use for generator
            .map(|x| if x == 0 { 1 } else { x })
            .unwrap_or(1);

        let (suffix_sender, suffix_receiver) = crossbeam_channel::bounded(5000 * num_searchers);
        let (result_sender, result_receiver) = crossbeam_channel::unbounded();

        let generator = Generator::spawn(suffix_length, suffix_sender);
        let mut searchers = Vec::with_capacity(num_searchers);

        let progress_bar = create_progress_bar(generator.len().try_into().expect("Too long"));

        for _ in 0..num_searchers {
            searchers.push(Searcher::spawn(
                signature_arc.clone(),
                suffix_length,
                zero_bytes,
                suffix_receiver.clone(),
                result_sender.clone(),
                progress_bar.clone(),
            ));
        }

        // to ensure that this thread stops after collecting results
        drop(result_sender);

        for searcher in searchers {
            searcher.join();
        }
        generator.join();

        progress_bar.finish_and_clear();

        let mut grouped_selector = GroupedSelector::new(&signature);
        while let Ok(prefix_with_selector) = result_receiver.recv() {
            grouped_selector.add(prefix_with_selector);
        }
        print_tree(&grouped_selector)?;
        println!();
    }

    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Generate {
            abi,
            signature,
            suffix_length,
            zero_bytes,
        } => {
            generate(abi, signature, suffix_length, zero_bytes)?;
        }
    };

    Ok(())
}

// CLI Constraints

fn zero_bytes_within_selector_length(s: &str) -> Result<u8, String> {
    let zero_bytes: u8 = s.parse().map_err(|_| format!("`{}` isn't a number", s))?;
    if SELECTOR_LENGTH_RANGE.contains(&zero_bytes) {
        Ok(zero_bytes)
    } else {
        Err(format!(
            "Number of bytes to be zeroed out not within length of function selector {}-{}",
            SELECTOR_LENGTH_RANGE.start(),
            SELECTOR_LENGTH_RANGE.end()
        ))
    }
}

fn parse_signature(s: &str) -> Result<FunctionSignature, String> {
    s.try_into().map_err(|x: eyre::Error| x.to_string())
}

fn abi_file_exists(s: &str) -> Result<PathBuf, String> {
    let p = Path::new(s);
    if !p.exists() {
        return Err("ABI file does not exist".to_string());
    }
    if p.is_dir() {
        return Err("ABI file cannot be a directory".to_string());
    }

    Ok(p.to_path_buf())
}

// Helpers

fn get_signatures_from(
    abi: Option<PathBuf>,
    signature: Option<FunctionSignature>,
) -> Result<Vec<FunctionSignature>> {
    Ok(match abi {
        Some(abi) => {
            let abi_file = File::open(abi)?;
            let reader = BufReader::new(abi_file);

            let abi: Abi = serde_json::from_reader(reader)?;

            abi.functions()
                .into_iter()
                .map(|x| x.clone().into())
                .collect()
        }
        _ => {
            vec![signature.expect("Either signature or abi must be provided")]
        }
    })
}
