// #![feature(result_option_inspect)]

use std::{
    env,
    io::{stdin, stdout, Write},
};

use calc::{solver::Context, tokens::tokenize, tree::create_tree};
use colored::Colorize;

use crate::calc::Token;

mod calc;
mod funcs;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let mut context = Context::new();

    if let Some(i) = args.get(1) {
        let quiet = args.contains(&"--quiet".to_string());

        let result = tokenize(i)
            .and_then(create_tree)
            .and_then(|x| context.evaluate(x));
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
        let result = tokenize(&input)
            // .inspect(|x| println!("TOKENIZE {:?}", x))
            .and_then(create_tree)
            // .inspect(|x| println!("TREE {:?}", x))
            .and_then(|x| context.evaluate(x));

        if let Ok(i) = result {
            // Maybe use prevous tree to get an exact result
            context.set_var("ans", Token::Number(i));
        }

        match result {
            Ok(i) => println!(" ⮩ {i}"),
            Err(e) => println!("{}", format!(" ⮩ {}", e).red()),
        }
    }
}
