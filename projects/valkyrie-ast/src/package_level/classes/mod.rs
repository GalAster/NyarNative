use super::*;
use crate::helper::ValkyrieNode;

mod display;

mod iters;

/// `class A { }, structure V { }`
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ClassKind {
    /// A function that lazy evaluate the arguments
    Class,
    /// A function that eager evaluate the arguments
    Structure,
}

/// `class Name(Super): Trait {}`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ClassDeclaration {
    /// The name of the class.
    pub name: IdentifierNode,
    /// The kind of class
    pub kind: ClassKind,
    /// The document of the class
    pub document: DocumentationNode,
    /// The modifiers of the class.
    pub modifiers: ModifiersNode,
    /// The parameter arguments of the class.
    pub generic: Option<ParametersList>,
    /// The super class of the class.
    pub base_classes: Option<String>,
    /// The traits that the class implements.
    pub auto_traits: Vec<String>,
    /// All fields declared in this block, exclude extensions.
    pub terms: Vec<ClassTerm>,
    /// The range of the number.
    pub span: Range<u32>,
}

#[derive(Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ClassTerm {
    Field(FieldDeclaration),
    Method(MethodDeclaration),
    Domain(DomainDeclaration),
}

/// `object: Trait { ... }`
///
/// Construct an anonymous object
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ConstructObjectNode {
    /// The super class of the class.
    pub base_classes: Option<String>,
    /// The range of the node
    pub span: Range<u32>,
}

/// `field: Type = default`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FieldDeclaration {
    /// The name of this field
    pub name: IdentifierNode,
    /// The documentation of the declaration.
    pub document: DocumentationNode,
    /// The modifiers of the declaration.
    pub modifiers: ModifiersNode,
    /// The type hint of this field
    pub typing: Option<ExpressionNode>,
    /// The default value of this field
    pub default: Option<ExpressionNode>,
    /// The range of the declaration.
    pub span: Range<u32>,
}

/// `method()`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MethodDeclaration {
    /// The documentation of the node.
    pub document: DocumentationNode,
    /// The modifiers of the node.
    pub modifiers: ModifiersNode,
    /// `method_name()`
    pub method_name: NamePathNode,
    /// `method_name<T>()`
    pub generic: Option<ParametersList>,
    /// `method_name(arguments)`
    pub arguments: ArgumentsList,
    /// `: ReturnType`
    pub return_type: Option<FunctionReturnNode>,
    /// `/ EffectType`
    pub effect_type: Option<FunctionEffectNode>,
    /// `{ body }`
    pub body: Option<StatementBlock>,
    /// The range of the declaration.
    pub span: Range<u32>,
}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DomainDeclaration {
    /// The range of the declaration.
    pub body: Vec<ClassTerm>,
    /// The range of the declaration.
    pub span: Range<u32>,
}

impl ValkyrieNode for ConstructObjectNode {
    fn get_range(&self) -> Range<usize> {
        Range { start: self.span.start as usize, end: self.span.end as usize }
    }
}

// impl ClassDeclare {
//     pub fn get_namepath(&self) -> Iter<'_, ValkyrieIdentifier> {
//         self.namepath.iter()
//     }
//     pub fn mut_namepath(&mut self) -> &mut Vec<ValkyrieIdentifier> {
//         &mut self.namepath
//     }
//     pub fn get_modifiers(&self) -> Iter<'_, ValkyrieIdentifier> {
//         self.modifiers.iter()
//     }
//     pub fn mut_modifiers(&mut self) -> &mut Vec<ValkyrieIdentifier> {
//         &mut self.modifiers
//     }
//     pub fn get_statement(&self) -> Iter<'_, ValkyrieASTNode> {
//         self.statements.iter()
//     }
//     pub fn mut_statement(&mut self) -> &mut Vec<ValkyrieASTNode> {
//         &mut self.statements
//     }
//     pub fn to_node(self, file: FileID, range: &Range<usize>) -> ValkyrieASTNode {
//         ValkyrieASTKind::Class(box self).to_node(file, range)
//     }
// }
