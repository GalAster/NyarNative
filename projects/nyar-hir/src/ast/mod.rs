mod atoms;
mod chain;
mod control;
mod import;
mod utils;

pub use self::{atoms::*, chain::*, control::*, import::ImportStatement};

#[cfg(feature = "pest")]
mod wrap_pest;

use lsp_types::Range;
use std::ops::AddAssign;
// use crate::ast::kind::chain::CallChain;
use std::fmt::{self, Debug, Display, Formatter};


pub type StringRange = (String, Range);

#[derive(Clone)]
pub struct ASTNode {
    pub kind: ASTKind,
    pub range: Range,
}

#[derive(Debug, Clone)]
pub enum ASTKind {
    /// Wrong node
    None,

    /// Root Node of the AST
    Program(Vec<ASTNode>),
    Suite(Vec<ASTNode>),
    /// - `EmptyStatement`: Skip
    EmptyStatement,
    /// - `ImportStatement`
    ImportStatement(Box<ImportStatement>),
    IfStatement(Box<IfStatement>),
    LetBinding,
    /// - `Expression`
    Expression {
        base: Box<ASTNode>,
        eos: bool,
    },
    /// - `Expression`
    TypeExpression {},
    ///
    ListExpression(Vec<ASTNode>),
    ///
    TupleExpression(Vec<ASTNode>),
    /// - `InfixOperators`
    Operator(Box<Operator>),
    CallChain(Box<CallChain>),
    /// - `SliceCall`
    ///
    /// ```v
    /// expr[index]
    /// ```
    CallSlice(Box<SliceCall>),
    ///
    /// ```v
    /// expr(index)
    /// ```
    CallApply(Box<ApplyCall>),
    ///
    /// ```v
    /// expr + rhs1 + rhs2
    /// ```
    CallInfix(Box<InfixCall>),
    /// - `UnaryOperators`
    ///     - `base`
    ///
    /// ```v
    /// ++expr!!
    /// ```
    CallUnary(Box<UnaryCall>),
    /// - `Comment`: raw comment with handler
    Comment(Box<CommentLiteral>),

    /// - `Symbol`
    Symbol(Box<Symbol>),
    /// - `Number`: raw number represent
    NumberLiteral(Box<NumberLiteral>),
    /// - `Number`: raw number represent
    ByteLiteral(Box<ByteLiteral>),
    ///
    StringLiteral(Box<StringLiteral>),
    ///
    Boolean(bool),
    /// - `Null`: It doesn't look like anything to me
    Null,
}

impl ASTNode {
    pub fn empty_statement(r: Range) -> Self {
        Self {
            kind: ASTKind::EmptyStatement,
            range: r,
        }
    }
    pub fn suite(v: Vec<ASTNode>, r: Range) -> Self {
        Self {
            kind: ASTKind::Suite(v),
            range: r,
        }
    }
    pub fn expression(base: ASTNode, eos: bool, r: Range) -> Self {
        Self {
            kind: ASTKind::Expression { base: box base, eos },
            range: r,
        }
    }


    pub fn infix(base: ASTNode, eos: bool, r: Range) -> Self {
        Self {
            kind: ASTKind::Expression { base: box base, eos },
            range: r,
        }
    }

    pub fn list(v: Vec<ASTNode>, r: Range) -> Self {
        Self {
            kind: ASTKind::ListExpression(v),
            range: r,
        }
    }

    pub fn tuple(v: Vec<ASTNode>, r: Range) -> Self {
        Self {
            kind: ASTKind::TupleExpression(v),
            range: r,
        }
    }

    pub fn boolean(v: bool, r: Range) -> Self {
        Self {
            kind: ASTKind::Boolean(v),
            range: r,
        }
    }



}