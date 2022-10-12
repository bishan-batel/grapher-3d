// Module Definitions ---------------------------------------------------------
pub mod lexer;
pub mod parser;
pub mod native;

use std::fmt::{Display, Write};

#[derive(Debug, Clone)]
#[allow(unused)]
pub enum ParserError {
    SyntaxError(String),
    RecursiveCall(String),
}

// Enumerations for use in Parsing -----------------------------------------

#[derive(Clone)]
#[allow(unused)]
pub enum ParseNode {
    Factor(f32),
    Identifier(String),
    Function(String, Vec<ParseNode>),
    FunctionDefine(String, Vec<String>, Box<ParseNode>),
    UnaryOp(Operator, Box<ParseNode>),
    BinOp(Box<ParseNode>, Operator, Box<ParseNode>),
}

impl Display for ParseNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseNode::FunctionDefine(name, args, body) => {
                let mut builder = String::new();

                for arg in args {
                    builder += format!("{},", arg).as_str();
                }

                // removes trailing comma
                builder = builder.trim_end_matches(&[',']).into();

                f.write_fmt(format_args!(
                    "float {}({}){{\n\treturn {}\n}}",
                    name, builder, body
                ))
            }
            ParseNode::Factor(val) => {
                let mut float_str = val.to_string();

                if !float_str.contains(".") {
                    float_str += ".0";
                }
                f.write_str(float_str.as_str())
            },
            ParseNode::Function(name, args) => {
                let mut builder = String::new();

                for arg in args {
                    builder += format!("{},", arg).as_str();
                }

                // removes trailing comma
                builder = builder.trim_end_matches(&[',']).into();

                f.write_fmt(format_args!("{}({})", name, builder))
            }
            ParseNode::Identifier(name) => f.write_fmt(format_args!("{}", name)),
            ParseNode::BinOp(lhs, op, rhs) => match op {
                Operator::Pow => f.write_fmt(format_args!("pow({}, {})", lhs, rhs)),
                _ => f.write_fmt(format_args!("({}{}{})", lhs, op, rhs)),
            },
            ParseNode::UnaryOp(op, node) => f.write_fmt(format_args!("({}{})", op, node)),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Token {
    RightParen,
    LeftParen,
    ArgumentSeperator,
    BinOp(Operator),
    Identifier(String),
    Equals,
    Literal(f32),
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LeftParen => f.write_str("("),
            Self::RightParen => f.write_str(")"),
            Self::Equals => f.write_str("="),
            Self::ArgumentSeperator => f.write_str(","),
            Self::BinOp(op) => op.fmt(f),
            Self::Literal(val) => val.fmt(f),
            Self::Identifier(val) => val.fmt(f),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

impl Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            Self::Add => '+',
            Self::Sub => '-',
            Self::Mul => '*',
            Self::Div => '/',
            Self::Pow => '^',
        })
    }
}

// Macros ---------------------------------------------------------------------
#[macro_export]
macro_rules! op_tok {
    ($v: ident) => {
        Token::BinOp(crate::parser::Operator::$v)
    };
}
