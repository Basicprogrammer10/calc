use std::{collections::HashMap, f64};

use convert_case::{Case, Casing};

use super::{tree::create_tree, Error, Num, Ops, Result, Token};
use crate::funcs::{Function, FUNCTIONS};

const CONSTANTS: &[(&str, Token)] = &[
    ("pi", Token::Number(f64::consts::PI)),
    ("e", Token::Number(f64::consts::E)),
    ("tau", Token::Number(f64::consts::TAU)),
];

pub struct Context {
    pub vars: HashMap<String, Token>,
    pub funcs: HashMap<String, &'static dyn Function>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            vars: CONSTANTS
                .iter()
                .map(|(n, v)| (n.to_string(), v.clone()))
                .collect(),
            funcs: FUNCTIONS
                .iter()
                .map(|x| (x.name().to_case(Case::Snake), *x))
                .collect(),
        }
    }

    pub fn set_var(&mut self, name: &str, value: Token) {
        self.vars.insert(name.to_string(), value);
    }

    pub fn evaluate(&mut self, tree: Token) -> Result<Num> {
        match tree {
            Token::Tree(op, left, right) => {
                let left = self.evaluate(*left)?;
                let right = self.evaluate(*right)?;

                Ok(match op {
                    Ops::Add => left + right,
                    Ops::Sub => left - right,
                    Ops::Mul => left * right,
                    Ops::Div => left / right,
                    Ops::Pow => left.powf(right),
                    Ops::Mod => left % right,
                })
            }
            Token::Number(n) => Ok(n),
            Token::Var(n) => self.evaluate(
                self.vars
                    .get(&n.to_lowercase())
                    .cloned()
                    .ok_or(Error::UnknownIdentifier(n))?,
            ),
            Token::Func(n, args) => {
                let func = self
                    .funcs
                    .get(&n.to_lowercase())
                    .ok_or(Error::UnknownIdentifier(n))?;

                Ok(func.call(
                    args.iter()
                        .flat_map(|x| create_tree(x.to_owned()))
                        .collect(),
                    self,
                )?)
            }
            _ => panic!("Invalid token {:?}", tree),
        }
    }
}
