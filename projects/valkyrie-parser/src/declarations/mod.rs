mod function;
mod methods;

use crate::{
    expression::TypingExpression,
    helpers::ignore,
    utils::{parse_expression_node, parse_modifiers},
    ThisParser,
};
use lispify::Lisp;
use valkyrie_ast::{
    ApplyArgumentNode, ArgumentKeyNode, ArgumentTermNode, ExpressionBody, ExpressionContext, ExpressionNode, ExpressionType,
    FunctionBodyPart, FunctionCommonPart, FunctionDeclaration, FunctionType, GenericArgumentNode, IdentifierNode, ModifierPart,
    NamePathNode, PrettyPrint, StatementNode,
};
use valkyrie_types::third_party::pex::{BracketPattern, ParseResult, ParseState, StopBecause};
