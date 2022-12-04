use std::{fmt::Display, result};

pub mod tokens;
pub mod tree;

pub type Num = f64;
pub type Result<T> = result::Result<T, Error>;

#[derive(Debug, Clone)]
pub enum Token {
    Number(Num),
    Op(Ops),
    Group(Vec<Token>),
    Tree(Ops, Box<Token>, Box<Token>),
}

#[derive(Debug, Clone, Copy)]
pub enum Ops {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

pub enum Error {
    // Tokenizer
    InvalidNumber(String),
    UnexpectedChar(char),

    // Tree
    InvalidExpression,
}

impl Ops {
    fn prio(&self) -> usize {
        match self {
            Ops::Add | Ops::Sub => 1,
            Ops::Mul | Ops::Div => 2,
            Ops::Pow => 3,
        }
    }
}

impl Token {
    fn make_tree(self) -> Result<Token> {
        match self {
            Token::Group(tokens) => tree::create_tree(tokens),
            _ => Ok(self),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Error::InvalidNumber(n) => format!("Invalid number: `{}`", n),
                Error::UnexpectedChar(c) => format!("Unexpected character: `{}`", c),
                Error::InvalidExpression => "Invalid expression".to_string(),
            }
        )
    }
}
