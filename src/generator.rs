use std::thread::{self, JoinHandle};

use crossbeam_channel::Sender;
use itertools::Itertools;

use crate::utils::PERMITTED_CHARS;

pub struct Generator {
    join_handle: JoinHandle<()>,
    length: u128,
}

impl Generator {
    pub fn spawn(suffix_length: u8, sender: Sender<String>) -> Self {
        let join_handle = thread::Builder::new()
            .name("suffix-generator".to_string())
            .spawn(move || {
                for suffix in generate_suffixes(suffix_length) {
                    match sender.send(suffix) {
                        Ok(_) => {}
                        Err(_) => break, // Stop thread on disconnect
                    }
                }
            })
            .unwrap();
        Self {
            join_handle,
            length: number_of_combinations_with_replacement(
                PERMITTED_CHARS.len() as u128,
                suffix_length as u128,
            ),
        }
    }

    pub fn len(&self) -> u128 {
        self.length
    }

    pub fn join(self) {
        let _ = self.join_handle.join();
    }
}

fn generate_suffixes(suffix_length: u8) -> impl Iterator<Item = String> {
    PERMITTED_CHARS
        .into_iter()
        .combinations_with_replacement(suffix_length.into())
        .map(|x| x.concat())
}

// n and r are only u8 as char set (n) is of length 64 and suffix length (r) is maximum 20
fn number_of_combinations_with_replacement(n: u128, r: u128) -> u128 {
    (n..=(n + r - 1)).product::<u128>() / (1..=r).product::<u128>()
}
