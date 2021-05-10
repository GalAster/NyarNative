use super::*;

#[cfg(feature = "pretty-print")]
mod display;

mod iters;

/// `union Name(Super): Trait {}`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UnionDeclaration {
    /// Documents of this union type
    pub document: DocumentationNode,
    /// The modifiers for the union
    pub modifiers: ModifierList,
    /// The range of the number.
    pub name: IdentifierNode,
    pub layout: Option<String>,
    pub derive_traits: Vec<String>,
    pub body: Vec<UnionTerm>,
    /// The text range of the statement
    pub span: Range<u32>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum UnionTerm {
    Field(UnionFieldDeclaration),
    Method(UnionMethodDeclaration),
}

/// `field: Type = default`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UnionFieldDeclaration {
    /// The documentation of the node.
    pub document: DocumentationNode,
    pub modifiers: ModifierList,
    pub field_name: IdentifierNode,
    pub r#type: ExpressionNode,
    /// The range of the node
    pub span: Range<u32>,
}

/// `union()`
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UnionMethodDeclaration {}
