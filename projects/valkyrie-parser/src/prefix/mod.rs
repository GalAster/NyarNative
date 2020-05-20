use crate::expression::{ValkyrieOperator, ValkyrieOperatorKind};
use pratt::Precedence;
use std::{
    fmt::{Debug, Formatter},
    ops::Range,
};

mod display;
mod parser;

#[derive(Clone)]
pub struct ValkyriePrefix {
    normalized: String,
    range: Range<usize>,
}

impl Debug for ValkyriePrefix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Infix({}, {:?})", self.normalized, self.range)
    }
}

impl ValkyriePrefix {
    pub fn new<S: ToString>(s: S, range: Range<usize>) -> ValkyriePrefix {
        ValkyriePrefix { normalized: s.to_string(), range }
    }

    pub fn precedence(&self) -> Precedence {
        match self.normalized.as_str() {
            "*" => Precedence(1000),
            "**" => Precedence(1000),
            "+" | "-" => Precedence(1000),
            "!" => Precedence(1000),
            _ => unreachable!("Unknown operator: {}", self.normalized),
        }
    }
    pub fn as_operator(&self) -> ValkyrieOperator {
        let kind = match self.normalized.as_str() {
            "+" => ValkyrieOperatorKind::Positive,
            "-" => ValkyrieOperatorKind::Negative,
            "*" => ValkyrieOperatorKind::Unbox,
            "**" => ValkyrieOperatorKind::Unpack,
            "!" => ValkyrieOperatorKind::Not,
            _ => unreachable!("Unknown operator: {}", self.normalized),
        };
        ValkyrieOperator::new(kind, self.range.clone())
    }
}
