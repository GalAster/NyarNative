use crate::utils::get_span;
use pex::{ParseResult, ParseState, Regex};
use pratt::{Associativity, Precedence};
use std::{
    fmt::{Debug, Formatter},
    ops::Range,
    sync::LazyLock,
};
use valkyrie_ast::{OperatorNode, ValkyrieOperator};

mod display;
mod parser;

#[derive(Clone)]
pub struct ValkyrieInfix {
    normalized: String,
    span: Range<u32>,
}

#[derive(Clone)]
pub struct ValkyriePrefix {
    normalized: String,
    span: Range<u32>,
}

#[derive(Clone)]
pub struct ValkyrieSuffix {
    normalized: String,
    span: Range<u32>,
}
