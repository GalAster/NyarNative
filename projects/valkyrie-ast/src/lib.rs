#![no_std]
#![allow(unused_imports)]
#![deny(missing_debug_implementations, missing_copy_implementations)]
#![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://avatars.githubusercontent.com/u/91894079")]
#![doc(html_favicon_url = "https://avatars.githubusercontent.com/u/91894079")]

extern crate alloc;

mod control_flow;
mod expression_level;
pub mod helper;
mod package_level;
mod patterns;
mod string_like;

pub use crate::{
    control_flow::{
        control::{ControlNode, ControlType, RaiseNode, TailCallNode},
        do_catch::{MatchCallNode, MatchKind, MatchStatement},
        do_try::TryStatement,
        jmp_guard::{GuardPattern, GuardStatement},
        jmp_if::{BreakStatement, ElseStatement, IfBranchNode, IfStatement, JumpStatement},
        jmp_switch::SwitchStatement,
        loop_for::{ForBarePattern, ForLoop},
        loop_while::{WhileConditionNode, WhileLoop, WhileLoopKind},
    },
    expression_level::{
        annotations::{AnnotationKind, AnnotationList, AnnotationNode, AnnotationPathNode, AnnotationTerm, ModifiersNode},
        argument::{ArgumentKey, ArgumentTerm, ArgumentsList},
        call_apply::ApplyCallNode,
        call_dot::{DotCallNode, DotCallTerm},
        call_generic::GenericCallNode,
        call_subscript::SubscriptCallNode,
        ctor::{CollectorNode, CollectorTerm, ConstructNewNode},
        lambda::{ClosureCallNode, LambdaNode},
        number::NumberLiteralNode,
        operators::{BinaryNode, LogicMatrix, OperatorNode, UnaryNode, ValkyrieOperator},
        parameter::{ParameterKind, ParameterTerm, ParametersList},
        range::{RangeKind, RangeNode, RangeTermNode},
        symbol::{BooleanNode, IdentifierNode, LambdaSlotItem, LambdaSlotNode, NamePathNode, NullNode, OutputNode},
        tuple::{TupleKind, TupleNode, TupleTermNode},
        ExpressionContext, ExpressionNode, ExpressionType, PostfixCallPart, TypingExpression,
    },
    package_level::{
        classes::{ClassDeclaration, ClassKind, ClassTerm, ConstructObjectNode, FieldDeclaration, MethodDeclaration},
        documentation::DocumentationNode,
        flags::{EncodeDeclaration, FlagDeclaration, FlagKind, FlagTerm},
        function::{
            FunctionDeclaration, FunctionDeclarationInline, FunctionEffectNode, FunctionReturnNode, FunctionType,
            StatementBlock,
        },
        guarantee::{EffectTypeNode, GuaranteeNode},
        import::{ImportAliasNode, ImportGroupNode, ImportResolvedItem, ImportState, ImportStatement, ImportTermNode},
        labeled::{GotoStatement, LabelStatement},
        let_bind::VariableDeclaration,
        namespace::{NamespaceDeclaration, NamespaceKind},
        program::ProgramRoot,
        statements::{StatementContext, StatementNode},
        tagged::{TaggedDeclaration, VariantDeclaration},
        traits::{ExtendsStatement, TraitDeclaration},
        unions::{UnionDeclaration, UnionFieldDeclaration},
    },
    patterns::{
        ArrayPatternNode, ClassPatternNode, IdentifierPattern, ImplicitCaseNode, PatternBlock, PatternBranch, PatternCaseNode,
        PatternCondition, PatternElseNode, PatternGuard, PatternNode, PatternStatements, PatternTypeNode, PatternWhenNode,
        TuplePatternNode, UnionPatternNode,
    },
    string_like::{
        string_formatter::{ExpressionFormatted, StringFormatter},
        string_literal::{StringLiteralNode, StringTextNode},
        string_template::{
            StringTemplateNode, TemplateCloseNode, TemplateCommentNode, TemplateInlineNode, TemplateLineType, TemplateOpenNode,
        },
    },
};
