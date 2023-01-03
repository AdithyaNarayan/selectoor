use std::{
    sync::Arc,
    thread::{self, JoinHandle},
};

use crossbeam_channel::{Receiver, Sender};
use indicatif::ProgressBar;

use crate::types::{FunctionSignature, Selector, SuffixWithSelector};

pub struct Searcher {
    join_handle: JoinHandle<()>,
}

impl Searcher {
    pub fn spawn(
        signature: Arc<FunctionSignature>,
        suffix_length: u8,
        zero_bytes: u8,
        suffix_receiver: Receiver<String>,
        success_sender: Sender<SuffixWithSelector>,
        progress_bar: ProgressBar,
    ) -> Self {
        let join_handle = thread::Builder::new()
            .name("searcher".to_string())
            .spawn(move || {
                // string buffer to swap suffixes instead of cloning each time
                let mut buffer = signature.name.clone()
                    + "_"
                    + &" ".repeat(suffix_length.into())
                    + &signature.inputs;
                let name_length = signature.name.len();

                let range_to_replace = (name_length + 1)..=(name_length + suffix_length as usize);

                loop {
                    match suffix_receiver.recv() {
                        Ok(suffix) => {
                            buffer.replace_range(range_to_replace.clone(), &suffix);

                            let selector = Selector::new(&buffer);

                            progress_bar.inc(1);

                            if selector.zero_count() < zero_bytes {
                                continue;
                            }

                            success_sender
                                .send(SuffixWithSelector::new(suffix, selector))
                                .unwrap();
                        }
                        Err(_) => break, // Stop thread on disconnect
                    }
                }
            })
            .unwrap();
        Self { join_handle }
    }

    pub fn join(self) {
        let _ = self.join_handle.join();
    }
}
