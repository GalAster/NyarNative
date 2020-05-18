mod display;
mod parser;
use crate::{expression::ValkyrieExpression, helpers::ignore};
use lispify::{Lisp, Lispify};
use pex::{helpers::make_from_str, ParseResult, ParseState, StopBecause};

pub use self::parser::parse_repl;
use std::fmt::{Display, Formatter};
/// A number literal.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ValkyrieREPL {
    Expression(Box<ValkyrieExpression>),
}
