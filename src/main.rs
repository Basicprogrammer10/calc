use std::{
    env,
    io::{stdin, stdout, Write},
};

fn main() {
    let args = env::args().collect::<Vec<_>>();

    if let Some(i) = args.get(1) {
        let quiet = args.contains(&"--quiet".to_string());
        let result = evaluate(create_tree(tokenize(&i)));
        println!("{}{result}", if quiet { "" } else { " ⮩ " });
    }

    loop {
        let mut input = String::new();
        print!(" ▷ ");
        stdout().flush().unwrap();
        stdin().read_line(&mut input).unwrap();
        let result = evaluate(create_tree(tokenize(&input)));
        println!("  ⮩ {result}");
    }
}

type Num = f64;

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

fn evaluate(tree: Token) -> Num {
    match tree {
        Token::Tree(op, left, right) => {
            let left = evaluate(*left);
            let right = evaluate(*right);

            match op {
                Ops::Add => left + right,
                Ops::Sub => left - right,
                Ops::Mul => left * right,
                Ops::Div => left / right,
                Ops::Pow => left.powf(right),
            }
        }
        Token::Number(n) => n,
        _ => panic!("Invalid token"),
    }
}

fn create_tree(mut tokens: Vec<Token>) -> Token {
    fn get_max_prio(tokens: &[Token]) -> usize {
        tokens
            .iter()
            .filter_map(|x| match x {
                Token::Op(i) => Some(i.prio()),
                _ => None,
            })
            .max()
            .unwrap()
    }

    fn contains_non_tree(tokens: &[Token]) -> bool {
        tokens
            .iter()
            .filter(|x| !matches!(x, Token::Tree(_, _, _)))
            .count()
            > 0
    }

    while contains_non_tree(&tokens) {
        let max_prio = get_max_prio(&tokens);

        let mut i = 0;
        while i < tokens.len() {
            if let Token::Op(op) = tokens[i] {
                if op.prio() < max_prio {
                    i += 1;
                    continue;
                }

                let left = tokens.remove(i - 1).make_tree();
                let right = tokens.remove(i).make_tree();

                tokens[i - 1] = Token::Tree(op, Box::new(left), Box::new(right));
                break;
            }

            i += 1;
        }
    }

    debug_assert!(tokens.len() == 1);
    tokens[0].clone()
}

fn tokenize(inp: &str) -> Vec<Token> {
    fn is_digit(chr: char) -> bool {
        chr.is_ascii_digit() || chr == '.'
    }

    fn add_num(out: &mut Vec<Token>, working: &str, sign: bool) {
        out.push(Token::Number(
            working
                .parse::<Num>()
                .unwrap()
                .copysign(if sign { -1. } else { 0. }),
        ));
    }

    let mut out = Vec::new();
    let mut working = String::new();
    let mut next_neg = false;
    let mut in_group = false;

    for i in inp.chars() {
        if !is_digit(i) && !working.is_empty() && !in_group {
            add_num(&mut out, &working, next_neg);
            next_neg = false;
            working.clear();
        }

        match i {
            i if i.is_whitespace() => continue,

            // Groups
            '(' => in_group = true,
            ')' => {
                in_group = false;
                out.push(Token::Group(tokenize(&working)));
                working.clear();
            }
            i if in_group => working.push(i),

            // Operations
            '-' => {
                if matches!(out.last(), Some(Token::Op(_))) {
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

            _ => panic!("INVALID CHAR: {}", i),
        }
    }

    if !working.is_empty() {
        add_num(&mut out, &working, next_neg);
    }

    out
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
    fn make_tree(self) -> Token {
        match self {
            Token::Group(tokens) => create_tree(tokens),
            _ => self,
        }
    }
}
