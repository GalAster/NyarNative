use super::*;
use crate::ArgumentKeyNode;
#[cfg(feature = "pretty-print")]
mod display;

/// `while cond {...} else {...}`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WhileLoopNode {
    pub condition: ConditionType,
    pub body: Vec<StatementNode>,
    pub r#else: Vec<StatementNode>,
    pub span: Range<u32>,
}

/// ```vk
/// for i in 0..10 {
///     ...
/// }
/// else {
///     ...
/// }
/// ```
///
/// ```vk
/// let i = 1;
/// let j = 1;
/// let mut i, mut j;
/// let [a, b]
/// let (a, b)
/// ```
///
/// ```vk
/// for i in range;
/// for i, j in range;
/// for mut i, mut j in range
/// for [table] in
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ForLoopNode {
    pub pattern: PatternType,
    pub condition: ConditionType,
    pub body: Vec<StatementNode>,
    pub r#else: Vec<StatementNode>,
    pub span: Range<u32>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PatternType {
    Tuple(Vec<ArgumentKeyNode>),
    Case,
}
