use crate::binary::ValkyrieOperator;
use pratt::{Associativity, Precedence};
use std::{
    fmt::{Debug, Formatter},
    ops::Range,
};

mod display;
mod parser;

pub struct ValkyrieInfix {
    normalized: String,
    range: Range<usize>,
}

impl Debug for ValkyrieInfix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Infix({}, {:?})", self.normalized, self.range)
    }
}

impl ValkyrieInfix {
    pub fn new<S: ToString>(s: S, range: Range<usize>) -> ValkyrieInfix {
        ValkyrieInfix { normalized: s.to_string(), range }
    }

    pub fn precedence(&self) -> Precedence {
        match self.normalized.as_str() {
            "+" | "-" => Precedence(2),
            "*" | "/" => Precedence(3),
            "^" => Precedence(4),
            _ => unreachable!("Unknown operator: {}", self.normalized),
        }
    }
    pub fn associativity(&self) -> Associativity {
        match self.normalized.as_str() {
            "+" | "-" => Associativity::Left,
            "*" | "/" => Associativity::Left,
            "^" => Associativity::Right,
            _ => unreachable!("Unknown operator: {}", self.normalized),
        }
    }
    pub fn as_operator(&self) -> ValkyrieOperator {
        match self.normalized.as_str() {
            "+" => ValkyrieOperator::Add,
            "-" => ValkyrieOperator::Sub,
            "*" => ValkyrieOperator::Mul,
            "/" => ValkyrieOperator::Div,
            "^" => ValkyrieOperator::Pow,
            _ => unreachable!("Unknown operator: {}", self.normalized),
        }
    }
}
