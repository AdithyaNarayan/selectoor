use ethers::abi::{AbiParser, Function, FunctionExt};
use eyre::Result;

use super::selector::Selector;

#[derive(Debug, Clone)]
pub struct FunctionSignature {
    pub name: String,
    pub inputs: String,
}

impl From<Function> for FunctionSignature {
    fn from(function: Function) -> Self {
        Self {
            name: function.name.clone(),
            inputs: function
                .abi_signature()
                .trim_start_matches(function.name.as_str())
                .to_string(),
        }
    }
}

impl TryFrom<&str> for FunctionSignature {
    type Error = eyre::Error;

    fn try_from(signature: &str) -> Result<Self> {
        let function = AbiParser::default().parse_function(signature)?;
        Ok(function.into())
    }
}

impl FunctionSignature {
    #[allow(dead_code)]
    pub fn selector(&self) -> Selector {
        Selector::new(&(self.name.clone() + &self.inputs))
    }
}

#[derive(Debug, Clone)]
pub struct SuffixWithSelector {
    pub suffix: String,
    pub selector: Selector,
}

impl SuffixWithSelector {
    pub fn new(suffix: String, selector: Selector) -> Self {
        Self { suffix, selector }
    }
}
