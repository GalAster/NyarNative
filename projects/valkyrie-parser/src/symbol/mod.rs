mod parser;
mod display;

use std::cell::LazyCell;
use std::sync::LazyLock;
use regex::{Captures, Regex};

use pex::helpers::{make_from_str, whitespace};
use pex::{ParseResult, ParseState, StopBecause};
use std::fmt::{Display, Formatter};
use std::ops::Range;
use std::str::FromStr;

/// These names cannot be used as function names
pub const KEYWORDS: [&str; 3] = ["true", "false", "null"];

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ValkyrieIdentifier {
    /// The name of the identifier.
    pub name: String,
    /// The range of the identifier.
    pub range: Range<usize>,
}

/// A namepath is a series of identifiers separated by dots.
#[derive(Clone, Debug)]
pub struct ValkyrieNamepath {
    /// The names of the identifier.
    pub names: Vec<ValkyrieIdentifier>,
    /// The range of the identifier.
    pub range: Range<usize>,
}

impl ValkyrieIdentifier {
    pub fn is_normal(&self) -> bool {
        self.name.starts_with(|c: char| c.is_ascii_lowercase())
    }
}
