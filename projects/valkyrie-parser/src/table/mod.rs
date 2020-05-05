mod display;
mod parser;

use crate::symbol::ValkyrieIdentifier;
use std::ops::Range;

/// A number literal.
#[derive(Debug, Clone, Eq, Hash)]
pub struct ValkyrieTable {
    /// The raw string of the number.
    pub value: String,
    /// The unit of the number, if any.
    pub unit: Option<ValkyrieIdentifier>,
    /// The range of the number.
    pub range: Range<usize>,
}

/// A number literal.
#[derive(Debug, Clone, Eq, Hash)]
pub struct ValkyrieBytes {
    /// The raw string of the number.
    pub bytes: Vec<u8>,
    /// The unit of the number, if any.
    pub unit: Option<ValkyrieIdentifier>,
    /// The range of the number.
    pub range: Range<usize>,
}

impl PartialEq for ValkyrieTable {
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value) && self.unit.eq(&other.unit)
    }
}

impl PartialEq for ValkyrieBytes {
    fn eq(&self, other: &Self) -> bool {
        self.bytes.eq(&other.bytes) && self.unit.eq(&other.unit)
    }
}
