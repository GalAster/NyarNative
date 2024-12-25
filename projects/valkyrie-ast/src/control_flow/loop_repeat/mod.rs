use crate::{IdentifierNode, StatementKind};
use core::ops::Range;

/// The pure `loop` statement
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LoopRepeat {
    pub keyword: Range<u32>,
    /// The named label of the loop
    pub label: Option<IdentifierNode>,
    pub terms: Vec<StatementKind>,
}

/// The `while` or `until` loop statement
#[derive(Debug)]
pub struct LoopContinuation {
    r#continue: Vec<StatementKind>,
    r#break: Vec<StatementKind>,
}

impl LoopContinuation {
    /// create a new loop
    pub fn new(v: Vec<StatementKind>) -> Self {
        Self { r#continue: vec![], r#break: vec![] }
    }
}
