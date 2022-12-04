use std::{
    env,
    io::{stdin, stdout, Write},
};

use calc::{
    tokens::tokenize,
    tree::{create_tree, evaluate},
};
use colored::Colorize;

mod calc;

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
