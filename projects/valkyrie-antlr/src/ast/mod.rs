mod extractors;
mod visitor;
use crate::{
    antlr::{
        valkyrieantlrlexer::ValkyrieAntlrLexer,
        valkyrieantlrparser::{self, *},
        valkyrieantlrvisitor::ValkyrieAntlrVisitor,
    },
    traits::Extractor,
};
use antlr_rust::{
    common_token_stream::CommonTokenStream,
    errors::ANTLRError,
    parser::ParserNodeType,
    parser_rule_context::ParserRuleContext,
    token::Token,
    tree::{ParseTree, ParseTreeVisitorCompat, TerminalNode, Tree, VisitChildren, VisitableDyn},
    CoerceTo, InputStream, TidExt,
};
use std::{ops::Range, rc::Rc, str::FromStr};
use valkyrie_ast::{
    ApplyCallNode, ArrayKind, ArrayNode, ArrayTermNode, BooleanNode, CallTermNode, ClassDeclaration, ClassFieldDeclaration,
    ClassKind, ExpressionNode, ExpressionType, ExtendsStatement, FlagsDeclaration, ForLoop, FunctionDeclaration, FunctionType,
    GuardPattern, GuardStatement, IdentifierNode, IfStatement, InfixNode, LetPattern, LogicMatrix, ModifiersNode, NamePathNode,
    NamespaceDeclaration, NamespaceKind, NullNode, NumberLiteralNode, OperatorNode, OutputNode, PostfixNode, PrefixNode,
    ProgramRoot, StatementNode, StringLiteralNode, StringTextNode, TupleKind, TupleNode, TuplePatternNode, TupleTermNode,
    UnionDeclaration, ValkyrieOperator, WhileConditionNode, WhileLoop, WhileLoopKind,
};

#[derive(Clone, Debug, Default)]
pub struct ValkyrieProgramParser {
    statements: Vec<StatementNode>,
}

impl ValkyrieProgramParser {
    pub fn parse(input: &str) -> Result<ProgramRoot, ANTLRError> {
        let codepoints = input.chars().map(|x| x as u32).collect::<Vec<_>>();
        let input = InputStream::new(&*codepoints);
        let lexer = ValkyrieAntlrLexer::new(input);
        let token_source = CommonTokenStream::new(lexer);
        let mut parser = ValkyrieAntlrParser::new(token_source);
        let root = parser.program()?;
        let mut state = ValkyrieProgramParser::default();
        state.visit(&*root);
        Ok(ProgramRoot { statements: state.statements })
    }
}
