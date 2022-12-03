use std::{
    env,
    fmt::Display,
    io::{stdin, stdout, Write},
    result,
};

use colored::Colorize;

fn main() {
    let args = env::args().collect::<Vec<_>>();

    if let Some(i) = args.get(1) {
        let quiet = args.contains(&"--quiet".to_string());

        let result = tokenize(i).and_then(create_tree).and_then(evaluate);
        match result {
            Ok(i) => println!("{}{i}", if quiet { "" } else { " ⮩ " }),
            Err(e) if !quiet => println!("{}", format!("[ERROR] {}", e).red()),
            _ => {}
        }
    }

    loop {
        let mut input = String::new();
        print!(" ▷ ");
        stdout().flush().unwrap();
        stdin().read_line(&mut input).unwrap();
        let result = tokenize(&input).and_then(create_tree).and_then(evaluate);
        match result {
            Ok(i) => println!(" ⮩ {i}"),
            Err(e) => println!("{}", format!(" ⮩ {}", e).red()),
        }
    }
}

type Num = f64;
type Result<T> = result::Result<T, Error>;

#[derive(Debug, Clone)]
enum Token {
    Number(Num),
    Op(Ops),
    Group(Vec<Token>),
    Tree(Ops, Box<Token>, Box<Token>),
}

#[derive(Debug, Clone, Copy)]
enum Ops {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

enum Error {
    // Tokenizer
    InvalidNumber(String),
    UnexpectedChar(char),

    // Tree
    InvalidExpression,
}

fn evaluate(tree: Token) -> Result<Num> {
    match tree {
        Token::Tree(op, left, right) => {
            let left = evaluate(*left)?;
            let right = evaluate(*right)?;

            Ok(match op {
                Ops::Add => left + right,
                Ops::Sub => left - right,
                Ops::Mul => left * right,
                Ops::Div => left / right,
                Ops::Pow => left.powf(right),
            })
        }
        Token::Number(n) => Ok(n),
        _ => panic!("Invalid token {:?}", tree),
    }
}

fn create_tree(mut tokens: Vec<Token>) -> Result<Token> {
    // can be optmised
    fn get_max_prio(tokens: &[Token]) -> Result<usize> {
        match tokens
            .iter()
            .filter_map(|x| match x {
                Token::Op(i) => Some(i.prio()),
                _ => None,
            })
            .max()
        {
            Some(i) => Ok(i),
            None => Err(Error::InvalidExpression),
        }
    }

    fn contains_non_tree(tokens: &[Token]) -> bool {
        tokens
            .iter()
            .filter(|x| !matches!(x, Token::Tree(_, _, _)))
            .count()
            > 0
    }

    fn safe_remove(tokens: &mut Vec<Token>, index: isize) -> Result<Token> {
        if index < 0 || index as usize >= tokens.len() {
            return Err(Error::InvalidExpression);
        }

        Ok(tokens.remove(index as usize))
    }

    if tokens.len() == 1 {
        match tokens.pop().unwrap() {
            Token::Number(i) => return Ok(Token::Number(i)),
            Token::Group(i) => return create_tree(i),
            _ => {}
        }
    }

    while contains_non_tree(&tokens) {
        let max_prio = get_max_prio(&tokens)?;

        let mut i = 0;
        while i < tokens.len() {
            if let Token::Op(op) = tokens[i] {
                if op.prio() < max_prio {
                    i += 1;
                    continue;
                }

                let left = safe_remove(&mut tokens, i as isize - 1)?.make_tree()?;
                let right = safe_remove(&mut tokens, i as isize)?.make_tree()?;

                tokens[i - 1] = Token::Tree(op, Box::new(left), Box::new(right));
                break;
            }

            i += 1;
        }
    }

    if tokens.len() != 1 {
        return Err(Error::InvalidExpression);
    }

    Ok(tokens[0].clone())
}

fn tokenize(inp: &str) -> Result<Vec<Token>> {
    fn is_digit(chr: char) -> bool {
        chr.is_ascii_digit() || chr == '.'
    }

    fn add_num(out: &mut Vec<Token>, working: &str, sign: bool) -> Result<()> {
        out.push(Token::Number(match working.parse::<Num>() {
            Ok(i) => i.copysign(if sign { -1. } else { 0. }),
            Err(_) => return Err(Error::InvalidNumber(working.to_string())),
        }));
        Ok(())
    }

    let mut out = Vec::new();
    let mut working = String::new();
    let mut next_neg = false;
    let mut in_group = false;

    for i in inp.chars() {
        if !is_digit(i) && !working.is_empty() && !in_group {
            add_num(&mut out, &working, next_neg)?;
            next_neg = false;
            working.clear();
        }

        match i {
            i if i.is_whitespace() => continue,

            // Groups
            '(' => in_group = true,
            ')' => {
                in_group = false;
                out.push(Token::Group(tokenize(&working)?));
                working.clear();
            }
            i if in_group => working.push(i),

            // Operations
            '-' => {
                if out.is_empty() || matches!(out.last(), Some(Token::Op(_))) {
                    next_neg ^= true;
                    continue;
                }
                out.push(Token::Op(Ops::Sub));
            }
            '+' => out.push(Token::Op(Ops::Add)),
            '*' => out.push(Token::Op(Ops::Mul)),
            '/' => out.push(Token::Op(Ops::Div)),
            '^' => out.push(Token::Op(Ops::Pow)),

            // Numbers
            i if is_digit(i) => working.push(i),

            _ => return Err(Error::UnexpectedChar(i)),
        }
    }

    if !working.is_empty() {
        add_num(&mut out, &working, next_neg)?;
    }

    Ok(out)
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
            Token::Group(tokens) => create_tree(tokens),
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
