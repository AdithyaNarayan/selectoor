use std::fmt;
use tiny_keccak::{Hasher, Keccak};

#[derive(Debug, Clone)]
pub struct Selector {
    selector: [u8; 4],
}

impl Selector {
    pub fn new(signature: &str) -> Self {
        let mut selector = [0u8; 4];

        let mut hasher = Keccak::v256();
        hasher.update(signature.as_bytes());
        hasher.finalize(&mut selector);

        Self { selector }
    }

    pub fn zero_count(&self) -> u8 {
        let mut count = 0;
        for e in self.selector {
            if e == 0u8 {
                count += 1;
            }
        }

        count
    }
}

impl fmt::Display for Selector {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "0x")?;
        for byte in self.selector {
            write!(fmt, "{:02x}", byte)?;
        }
        Ok(())
    }
}
