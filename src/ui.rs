use std::{borrow::Cow, fmt, io};

use ptree::{Style, TreeItem};

use crate::types::{FunctionSignature, Selector, SuffixWithSelector};

#[derive(Debug, Clone)]
pub struct GroupedSelector<'a> {
    signature: &'a FunctionSignature,
    selectors_by_zero_bytes: Vec<SuffixesWithSelector<'a>>,
}

impl<'a> GroupedSelector<'a> {
    pub fn new(signature: &'a FunctionSignature) -> Self {
        Self {
            signature,
            selectors_by_zero_bytes: vec![
                SuffixesWithSelector::new(0, signature),
                SuffixesWithSelector::new(1, signature),
                SuffixesWithSelector::new(2, signature),
                SuffixesWithSelector::new(3, signature),
                SuffixesWithSelector::new(4, signature),
            ],
        }
    }

    pub fn add(&mut self, suffix_with_selector: SuffixWithSelector) {
        self.selectors_by_zero_bytes[suffix_with_selector.selector.zero_count() as usize]
            .suffixes
            .push(suffix_with_selector);
    }
}

impl<'a> TreeItem for GroupedSelector<'a> {
    type Child = SuffixesWithSelector<'a>;

    fn write_self<W: io::Write>(&self, f: &mut W, _: &Style) -> io::Result<()> {
        let current_signature = SignatureWithSelector {
            signature: self.signature.name.clone() + &self.signature.inputs,
            selector: self.signature.selector(),
        };
        write!(f, "Results for {}", current_signature)
    }

    fn children(&self) -> Cow<[Self::Child]> {
        Cow::from(
            self.selectors_by_zero_bytes
                .iter()
                .filter(|x| !x.suffixes.is_empty())
                .map(|x| x.clone())
                .collect::<Vec<_>>(),
        )
    }
}

/// Internal type to impl `TreeItem` for `Vec<SuffixWithSelector>`
#[derive(Debug, Clone)]
pub struct SuffixesWithSelector<'a> {
    pub suffixes: Vec<SuffixWithSelector>,
    zero_count: u8,
    signature: &'a FunctionSignature,
}

impl<'a> SuffixesWithSelector<'a> {
    pub fn new(zero_count: u8, signature: &'a FunctionSignature) -> Self {
        Self {
            suffixes: vec![],
            zero_count,
            signature,
        }
    }
}

impl<'a> TreeItem for SuffixesWithSelector<'a> {
    type Child = SignatureWithSelector;

    fn write_self<W: io::Write>(&self, f: &mut W, _: &Style) -> io::Result<()> {
        write!(f, "With {} zero bytes", self.zero_count)
    }

    fn children(&self) -> Cow<[Self::Child]> {
        Cow::from(
            self.suffixes
                .iter()
                .map(|x| SignatureWithSelector::new(self.signature, x))
                .collect::<Vec<_>>(),
        )
    }
}

#[derive(Clone)]
pub struct SignatureWithSelector {
    pub signature: String,
    pub selector: Selector,
}

impl SignatureWithSelector {
    pub fn new(signature: &FunctionSignature, suffix_with_selector: &SuffixWithSelector) -> Self {
        let signature =
            signature.name.clone() + "_" + &suffix_with_selector.suffix + &signature.inputs;

        Self {
            signature,
            selector: suffix_with_selector.selector.clone(),
        }
    }
}

impl fmt::Display for SignatureWithSelector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} → {}", self.signature, self.selector)
    }
}

impl TreeItem for SignatureWithSelector {
    type Child = Self;

    fn write_self<W: io::Write>(&self, f: &mut W, _: &Style) -> io::Result<()> {
        write!(f, "{}", self)
    }

    fn children(&self) -> Cow<[Self::Child]> {
        Cow::from(vec![])
    }
}
