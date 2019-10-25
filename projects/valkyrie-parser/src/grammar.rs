use crate::pest_parser::{Rule, Valkyrie};
use crate::utils::unescape;
use nyar_ast::ast::ImportStatement;
use nyar_ast::AST;
use pest::iterators::{Pair, Pairs};
use pest::Parser;

use pest::prec_climber::{Assoc, Operator, PrecClimber};

#[derive(Debug)]
struct Settings {
    pub refine: bool,
}

impl Settings {
    fn from_cli_args() -> Self {
        let args = std::env::args();
        Self { refine: true }
    }
}

#[rustfmt::skip]
lazy_static! {
    static ref SETTINGS: Settings = Settings::from_cli_args();
    static ref PREC_CLIMBER: PrecClimber<Rule> = {
        use Rule::*;
        use Assoc::*;
        //TODO: use macro
        PrecClimber::new(vec![
            Operator::new(Plus, Left) | Operator::new(Minus, Left),
            Operator::new(Multiply, Left) | Operator::new(CenterDot, Left),
            Operator::new(Power, Right)
        ])
    };
}

pub fn get_ast(text: &str) -> AST {
    let pairs = Valkyrie::parse(Rule::program, text).unwrap_or_else(|e| panic!("{}", e));
    let mut nodes: Vec<AST> = vec![];
    for pair in pairs {
        let rule = pair.as_rule();
        let node = match rule {
            Rule::EOI => continue,
            Rule::eos => continue,
            Rule::emptyStatement => AST::EmptyStatement,
            Rule::importStatement => parse_import(pair.into_inner()),
            Rule::if_statement => parse_if(pair.into_inner()),
            Rule::expression => parse_expression(pair.into_inner()),
            _ => {
                println!("Unimplemented Valkyrie Rule::{:?}=>AST::None,", rule);
                AST::None
            }
        };
        nodes.push(node)
    }
    return AST::Program(nodes);
}

fn parse_import(pairs: Pairs<Rule>) -> AST {
    let mut root = 0;
    for pair in pairs {
        match pair.as_rule() {
            Rule::Dot => {
                root += 1;
                continue;
            }
            Rule::use_alias => {
                let mut nodes: Vec<String> = vec![];
                for inner in pair.into_inner() {
                    let node = match inner.as_rule() {
                        Rule::SYMBOL => inner.as_str().to_string(),
                        _ => continue,
                    };
                    nodes.push(node)
                }
                let alias = nodes.pop().unwrap();
                return AST::ImportStatement { data: ImportStatement::LocalAlias { root, path: nodes, alias }, modifier: None };
            }
            Rule::use_module_select => {
                println!("Rule:    {:?}", pair.as_rule());
                println!("Span:    {:?}", pair.as_span());
                println!("Text:    {}\n", pair.as_str());
                AST::None
            }
            Rule::use_module_string => {
                println!("Rule:    {:?}", pair.as_rule());
                println!("Span:    {:?}", pair.as_span());
                println!("Text:    {}\n", pair.as_str());
                AST::None
            }
            _ => continue,
        };
    }
    return AST::None;
}

fn parse_if(pairs: Pairs<Rule>) -> AST {
    let mut conditions: Vec<AST> = vec![];
    let mut blocks: Vec<AST> = vec![];
    let mut default = None;
    for pair in pairs {
        match pair.as_rule() {
            Rule::If => (),
            Rule::Else => (),
            Rule::expr => conditions.push(parse_expr(pair.into_inner())),
            Rule::block => blocks.push(parse_block(pair.into_inner())),
            _ => unreachable!(),
        }
    }
    if conditions.len() != blocks.len() {
        default = Some(Box::new(blocks.pop().unwrap()))
    }
    let pairs = conditions.into_iter().zip(blocks.into_iter()).collect();
    return AST::IfStatement { pairs, default, modifier: None };
}

fn parse_dict(pairs: Pairs<Rule>) -> AST {
    let mut vec: Vec<AST> = vec![];
    for pair in pairs {
        match pair.as_rule() {
            Rule::expr => vec.push(parse_expr(pair.into_inner())),
            _ => {
                println!("parse_data: Rule::{:?}=>AST::None,", pair.as_rule());
                println!("Span:       {:?}", pair.as_span());
                println!("Text:       {}\n", pair.as_str());
            }
        };
    }
    return AST::None;
}

fn parse_block(pairs: Pairs<Rule>) -> AST {
    let mut pass: Vec<AST> = vec![];
    for pair in pairs {
        match pair.as_rule() {
            Rule::expr => {
                let node = parse_expr(pair.into_inner());
                pass.push(node);
            }
            _ => {
                println!("parse_block: Rule::{:?}=>AST::None,", pair.as_rule());
                println!("Span:        {:?}", pair.as_span());
                println!("Text:        {}\n", pair.as_str());
            }
        };
    }
    return AST::None;
}

fn parse_expression(pairs: Pairs<Rule>) -> AST {
    let mut base = AST::None;
    let mut eos = false;
    for pair in pairs {
        match pair.as_rule() {
            Rule::expr => base = parse_expr(pair.into_inner()),
            Rule::eos => eos = true,
            _ => unreachable!(),
        };
    }
    return AST::Expression { base: Box::new(base), eos, modifier: None };
}

#[rustfmt::skip]
fn parse_expr(pairs: Pairs<Rule>) -> AST {
    PREC_CLIMBER.climb(
        pairs,
        |pair: Pair<Rule>| match pair.as_rule() {
            Rule::expr => parse_expr(pair.into_inner()),
            Rule::term => parse_term(pair.into_inner()),
            Rule::trinocular => {
                println!("parse_expr: Rule::{:?}=>AST::None,", pair.as_rule());
                println!("Span:       {:?}", pair.as_span());
                println!("Text:       {}\n", pair.as_str());
                AST::None
            }
            Rule::bracket_call => {
                println!("parse_expr: Rule::{:?}=>AST::None,", pair.as_rule());
                println!("Span:       {:?}", pair.as_span());
                println!("Text:       {}\n", pair.as_str());
                AST::None
            }
            _ => unreachable!(),
        },
        |left: AST, op: Pair<Rule>, right: AST| match op.as_rule() {
            _ => AST::InfixOperators {
                lhs: Box::new(left),
                rhs: Box::new(right),
                o: op.as_str().to_string(),
            },
        },
    )
}

fn parse_term(pairs: Pairs<Rule>) -> AST {
    let mut base = AST::None;
    let mut prefix = vec![];
    let mut postfix = vec![];
    for pair in pairs {
        match pair.as_rule() {
            Rule::node => base = parse_node(pair.into_inner()),
            Rule::Prefix => prefix.push(pair.as_str().to_string()),
            Rule::Postfix => postfix.push(pair.as_str().to_string()),
            _ => unreachable!(),
        };
    }
    return if prefix.len() + postfix.len() == 0 { base } else { AST::UnaryOperators { base: Box::new(base), prefix, postfix } };
}

fn parse_node(pairs: Pairs<Rule>) -> AST {
    for pair in pairs {
        return match pair.as_rule() {
            Rule::expr => parse_expr(pair.into_inner()),
            Rule::data => parse_data(pair.into_inner()),
            Rule::bracket_call => parse_bracket_call(pair.into_inner()),
            _ => {
                println!("parse_node: Rule::{:?}=>AST::None,", pair.as_rule());
                println!("Span:       {:?}", pair.as_span());
                println!("Text:       {}\n", pair.as_str());
                AST::None
            }
        };
    }
    return AST::None;
}

fn parse_bracket_call(pairs: Pairs<Rule>) -> AST {
    let mut unboxed = AST::None;
    for pair in pairs {
        match pair.as_rule() {
            Rule::data => unboxed = parse_data(pair.into_inner()),
            Rule::apply => {
                let mut apply = parse_apply(pair.into_inner());
                // return AST::ApplyExpression { base: Box::new(base), ..apply };
                match apply {
                    AST::ApplyExpression { base, types, args, kv_pairs } => {
                        apply = AST::ApplyExpression { base: Box::new(unboxed), types, args, kv_pairs };
                        return apply;
                    }
                    _ => unreachable!(),
                }
            }
            Rule::slice => {
                let mut list = vec![];
                for inner in pair.into_inner() {
                    match inner.as_rule() {
                        Rule::Comma => (),
                        Rule::index => list.push(parse_index(inner.into_inner())),
                        _ => unreachable!(),
                    };
                }
                return AST::SliceExpression { base: Box::new(unboxed), list };
            }
            _ => {
                println!("parse_bracket_call: Rule::{:?}=>AST::None,", pair.as_rule());
                println!("Span:       {:?}", pair.as_span());
                println!("Text:       {}\n", pair.as_str());
            }
        };
    }
    return AST::None;
}

fn parse_apply(pairs: Pairs<Rule>) -> AST {
    let mut args = vec![];
    let mut kv_pairs = vec![];
    let mut types = vec![];
    for pair in pairs {
        match pair.as_rule() {
            Rule::Comma => (),
            Rule::apply_kv => {
                let (mut k, mut v) = (AST::None, AST::None);
                for inner in pair.into_inner() {
                    match inner.as_rule() {
                        Rule::Colon => (),
                        Rule::SYMBOL => k = parse_symbol(inner),
                        Rule::expr => v = parse_expr(inner.into_inner()),
                        _ => unreachable!(),
                    };
                }
                match k {
                    AST::None => args.push(k),
                    _ => kv_pairs.push((k, v)),
                }
            }
            Rule::apply_type => {
                for inner in pair.into_inner() {
                    match inner.as_rule() {
                        Rule::expr => types.push(parse_expr(inner.into_inner())),
                        _ => unreachable!(),
                    };
                }
            }
            _ => unreachable!(),
        };
    }
    return AST::ApplyExpression { base: Box::new(AST::None), types, args, kv_pairs };
}

fn parse_apply_kv(pairs: Pairs<Rule>) -> (AST, AST) {
    let (mut k, mut v) = (AST::None, AST::None);
    for pair in pairs {
        match pair.as_rule() {
            Rule::Colon => (),
            Rule::SYMBOL => k = parse_symbol(pair),
            Rule::expr => v = parse_expr(pair.into_inner()),
            _ => unreachable!(),
        };
    }
    return (k, v);
}

fn parse_index(pairs: Pairs<Rule>) -> AST {
    let mut base = AST::None;
    for pair in pairs {
        match pair.as_rule() {
            Rule::expr => return parse_expr(pair.into_inner()),
            _ => {
                println!("parse_slice: Rule::{:?}=>AST::None,", pair.as_rule());
                println!("Span:       {:?}", pair.as_span());
                println!("Text:       {}\n", pair.as_str());
            }
        };
    }
    return AST::None;
}

fn parse_data(pairs: Pairs<Rule>) -> AST {
    for pair in pairs {
        let node = match pair.as_rule() {
            Rule::String => parse_string(pair.into_inner()),
            Rule::Boolean => parse_boolean(pair.into_inner()),
            Rule::Number => parse_number(pair.into_inner()),
            Rule::Byte => parse_byte(pair.into_inner()),
            Rule::Symbol => parse_symbol(pair),
            Rule::list => parse_list(pair.into_inner()),
            _ => {
                println!("parse_data: Rule::{:?}=>AST::None,", pair.as_rule());
                println!("Span:       {:?}", pair.as_span());
                println!("Text:       {}\n", pair.as_str());
                AST::None
            }
        };
        return node;
    }
    return AST::None;
}

fn parse_list(pairs: Pairs<Rule>) -> AST {
    let mut vec: Vec<AST> = vec![];
    for pair in pairs {
        match pair.as_rule() {
            Rule::Comma => (),
            Rule::expr => vec.push(parse_expr(pair.into_inner())),
            _ => unreachable!(),
        };
    }
    return AST::ListExpression(vec);
}

fn parse_string(pairs: Pairs<Rule>) -> AST {
    let (mut h, mut t) = ("", String::new());
    for pair in pairs {
        match pair.as_rule() {
            Rule::SYMBOL => h = pair.as_str(),
            Rule::StringEmpty => continue,
            Rule::StringNormal => {
                for inner in pair.into_inner() {
                    match inner.as_rule() {
                        Rule::StringText => t = unescape(inner.as_str()),
                        _ => continue,
                    };
                }
            }
            Rule::StringLiteral => {
                for inner in pair.into_inner() {
                    match inner.as_rule() {
                        Rule::StringLiteralText => t = unescape(inner.as_str()),
                        _ => continue,
                    };
                }
            }
            _ => unreachable!(),
        };
    }
    return AST::StringLiteral { handler: h.to_string(), data: t };
}

fn parse_number(pairs: Pairs<Rule>) -> AST {
    let (mut h, mut t) = ("", String::new());
    for pair in pairs {
        match pair.as_rule() {
            Rule::Integer => {
                h = "int";
                t = pair.as_str().to_string();
            }
            Rule::Decimal => {
                h = "fp";
                t = pair.as_str().to_string();
            }
            Rule::DecimalBad => {
                h = "fp";
                let s = pair.as_str();
                if s.starts_with('.') {
                    t = "0".to_string() + s
                } else {
                    t = s.to_string() + "0"
                }
            }
            Rule::Complex => {
                for inner in pair.into_inner() {
                    match inner.as_rule() {
                        Rule::Integer => t = inner.as_str().to_string(),
                        Rule::Decimal => t = inner.as_str().to_string(),
                        Rule::SYMBOL => h = inner.as_str(),
                        _ => unreachable!(),
                    };
                }
            }
            _ => unreachable!(),
        };
    }
    let n = AST::NumberLiteral { handler: h.to_string(), data: t };
    return if SETTINGS.refine { n.parse_number() } else { n };
}

fn parse_byte(pairs: Pairs<Rule>) -> AST {
    let (mut h, mut t) = ("", "0");
    for pair in pairs {
        match pair.as_rule() {
            Rule::Byte_HEX => {
                let s = pair.as_str();
                h = "x";
                t = &s[2..s.len()];
            }
            Rule::Byte_OCT => {
                let s = pair.as_str();
                h = "o";
                t = &s[2..s.len()];
            }
            Rule::Byte_BIN => {
                let s = pair.as_str();
                h = "b";
                t = &s[2..s.len()];
            }
            _ => unreachable!(),
        };
    }
    let n = AST::NumberLiteral { handler: h.to_string(), data: t.to_string() };
    return if SETTINGS.refine { n.parse_number() } else { n };
}

fn parse_boolean(pairs: Pairs<Rule>) -> AST {
    for pair in pairs {
        let node = match pair.as_rule() {
            Rule::True => AST::Boolean(true),
            Rule::False => AST::Boolean(false),
            _ => unreachable!(),
        };
        return node;
    }
    return AST::None;
}

fn parse_symbol(pairs: Pair<Rule>) -> AST {
    let mut scope = vec![];
    match pairs.as_rule() {
        Rule::SYMBOL => scope.push(pairs.as_str().to_string()),
        _ => {
            for pair in pairs.into_inner() {
                match pair.as_rule() {
                    Rule::SYMBOL => scope.push(pair.as_str().to_string()),
                    Rule::namespace => {
                        for inner in pair.into_inner() {
                            match inner.as_rule() {
                                Rule::Proportion => (),
                                Rule::SYMBOL => scope.push(inner.as_str().to_string()),
                                _ => unreachable!(),
                            };
                        }
                    }
                    _ => unreachable!(),
                };
            }
        }
    }
    let name = scope.pop().unwrap();
    return AST::Symbol { name, scope };
}
