use crate::{
    helpers::{ignore, parse_eos},
    traits::ThisParser,
    utils::{get_span, parse_expression_node, parse_modifiers},
};
use lispify::{Lisp, ListString};
use valkyrie_ast::{
    ClassDeclaration, ControlNode, DocumentationNode, ExpressionContext, ExpressionNode, ForLoopNode, FunctionDeclaration,
    FunctionType, ImportStatementNode, LambdaArgumentNode, LambdaCallNode, LambdaDotNode, LambdaNode, LetBindNode,
    NamePathNode, NamespaceDeclarationNode, StatementNode, StatementType, WhileLoopNode,
};
use valkyrie_types::third_party::pex::{helpers::comment_line, ParseResult, ParseState, StopBecause};

mod classes;
mod def_var;
mod documentation;
mod lambda;
mod new;

pub struct ReplRoot {
    pub statements: Vec<StatementNode>,
}

pub struct ScriptRoot {
    pub statements: Vec<StatementNode>,
}

impl ThisParser for ReplRoot {
    fn parse(input: ParseState) -> ParseResult<Self> {
        input.match_repeats(|s| parse_statement_node(s, false)).map_inner(|s| ReplRoot { statements: s })
    }

    fn as_lisp(&self) -> Lisp {
        unreachable!()
    }
}

impl ThisParser for ScriptRoot {
    fn parse(input: ParseState) -> ParseResult<Self> {
        input.match_repeats(|s| parse_statement_node(s, false)).map_inner(|s| ScriptRoot { statements: s })
    }

    fn as_lisp(&self) -> Lisp {
        unreachable!()
    }
}

impl ThisParser for StatementNode {
    /// - [term](ExpressionType::parse)
    fn parse(input: ParseState) -> ParseResult<Self> {
        parse_statement_node(input, false)
    }

    fn as_lisp(&self) -> Lisp {
        self.r#type.as_lisp()
    }
}

pub fn parse_statement_node(input: ParseState, repl: bool) -> ParseResult<StatementNode> {
    let parser = match repl {
        true => parse_repl_statements,
        false => StatementType::parse,
    };
    let (state, expr) = input.skip(ignore).match_fn(parser)?;
    let (state, eos) = parse_eos(state)?;
    state.finish(StatementNode { r#type: expr, end_semicolon: eos, span: get_span(input, state) })
}

impl ThisParser for StatementType {
    fn parse(input: ParseState) -> ParseResult<Self> {
        input
            .begin_choice()
            .or_else(|s| NamespaceDeclarationNode::parse(s).map_inner(Into::into))
            .or_else(|s| ImportStatementNode::parse(s).map_inner(Into::into))
            .or_else(|s| ClassDeclaration::parse(s).map_inner(Into::into))
            .or_else(function_with_head)
            .or_else(|s| WhileLoopNode::parse(s).map_inner(Into::into))
            .or_else(|s| ForLoopNode::parse(s).map_inner(Into::into))
            .or_else(|s| ControlNode::parse(s).map_inner(Into::into))
            .or_else(|s| parse_expression_node(s, ExpressionContext::in_free()).map_inner(Into::into))
            .end_choice()
    }

    fn as_lisp(&self) -> Lisp {
        match self {
            StatementType::Nothing => Lisp::Any(vec![]),
            StatementType::Namespace(v) => v.as_lisp(),
            StatementType::Import(v) => v.as_lisp(),
            StatementType::While(v) => v.as_lisp(),
            StatementType::For(v) => v.as_lisp(),
            StatementType::Class(v) => v.as_lisp(),
            StatementType::Expression(v) => v.as_lisp(),
            StatementType::Function(v) => v.as_lisp(),
            StatementType::Control(v) => v.as_lisp(),
            StatementType::Document(v) => v.as_lisp(),
            StatementType::LetBind(v) => v.as_lisp(),
        }
    }
}

pub fn parse_repl_statements(input: ParseState) -> ParseResult<StatementType> {
    input
        .begin_choice()
        .or_else(|s| NamespaceDeclarationNode::parse(s).map_inner(Into::into))
        .or_else(|s| ImportStatementNode::parse(s).map_inner(Into::into))
        .or_else(|s| ClassDeclaration::parse(s).map_inner(Into::into))
        .or_else(|s| FunctionDeclaration::parse(s).map_inner(Into::into))
        .or_else(|s| WhileLoopNode::parse(s).map_inner(Into::into))
        .or_else(|s| ForLoopNode::parse(s).map_inner(Into::into))
        .or_else(|s| parse_expression_node(s, ExpressionContext::in_free()).map_inner(Into::into))
        .end_choice()
}

fn function_with_head(input: ParseState) -> ParseResult<StatementType> {
    let (state, mods) = parse_modifiers(input, FunctionType::parse)?;
    let (state, mut func) = state.skip(ignore).match_fn(FunctionDeclaration::parse)?;
    func.modifiers = mods.modifiers;
    state.finish(StatementType::Function(Box::new(func)))
}
