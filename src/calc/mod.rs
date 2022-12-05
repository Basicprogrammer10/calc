use std::{fmt::Display, result};

pub mod solver;
pub mod tokens;
pub mod tree;

pub type Num = f64;
pub type Result<T> = result::Result<T, Error>;

#[derive(Debug, Clone)]
pub enum Token {
    // == Basic tokens ==
    Number(Num),
    Op(Ops),
    Group(Vec<Token>),

    // == Dynamic ==
    Func(String, Vec<Vec<Token>>),
    Var(String),

    // == Misc ==
    Tree(Ops, Box<Token>, Box<Token>),
}

#[derive(Debug, Clone, Copy)]
pub enum Ops {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Mod,
}

#[derive(Debug)]
pub enum Error {
    // Tokenizer
    InvalidNumber(String),

    // Tree (more detailed?)
    InvalidExpression,

    // Solver
    UnknownIdentifier(String),

    // Function
    InvalidArgumentCount(String, usize, usize),
}

impl Ops {
    fn prio(&self) -> usize {
        match self {
            Ops::Add | Ops::Sub => 1,
            Ops::Mul | Ops::Div | Ops::Mod => 2,
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

impl Display for Ops {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Ops::Add => write!(f, "+"),
            Ops::Sub => write!(f, "-"),
            Ops::Mul => write!(f, "*"),
            Ops::Div => write!(f, "/"),
            Ops::Pow => write!(f, "^"),
            Ops::Mod => write!(f, "%"),
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&match self {
            Token::Number(n) => n.to_string(),
            Token::Op(op) => op.to_string(),
            Token::Group(tokens) => format!(
                "({})",
                tokens
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(" ")
            ),
            Token::Func(name, args) => format!(
                "{}({})",
                name,
                args.iter()
                    .map(|x| x
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<_>>()
                        .join(" "))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Token::Var(name) => name.to_string(),
            Token::Tree(op, left, right) => format!("({} {} {})", left, op, right),
        })
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&match self {
            Error::InvalidNumber(n) => format!("Invalid number: `{}`", n),
            Error::InvalidExpression => "Invalid expression".to_string(),
            Error::UnknownIdentifier(n) => format!("Unknown identifier: `{}`", n),
            Error::InvalidArgumentCount(n, a, e) => format!(
                "Invalid argument count for `{}`: expected {}, got {}",
                n, e, a
            ),
        })
    }
}
