use std::collections::HashMap;

use super::{Error, Result, Token};

// assumes no Trees in the input
pub fn create_tree(mut tokens: Vec<Token>) -> Result<Token> {
    if tokens.len() == 1 {
        match tokens.pop().unwrap() {
            Token::Number(i) => return Ok(Token::Number(i)),
            Token::Group(i) => return create_tree(i),
            Token::Var(i) => return Ok(Token::Var(i)),
            Token::Func(i, j) => return Ok(Token::Func(i, j)),
            _ => {}
        }
    }

    let mut prios = get_max_prio(&tokens)?;
    while tokens.len() > 1 {
        let mut i = 0;
        while i < tokens.len() {
            if let Token::Op(op) = tokens[i] {
                let max_prio = *prios.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap().0;
                if op.prio() < max_prio {
                    i += 1;
                    continue;
                }

                let left = safe_remove(&mut tokens, i as isize - 1)?.make_tree()?;
                let right = safe_remove(&mut tokens, i as isize)?.make_tree()?;

                tokens[i - 1] = Token::Tree(op, Box::new(left), Box::new(right));
                prios.entry(op.prio()).and_modify(|x| *x -= 1);
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

// Maps (Prio, Count)
fn get_max_prio(tokens: &[Token]) -> Result<HashMap<usize, usize>> {
    let mut out = HashMap::new();

    for i in tokens {
        if let Token::Op(op) = i {
            *out.entry(op.prio()).or_insert(0) += 1;
        }
    }

    Ok(out)
}

fn safe_remove(tokens: &mut Vec<Token>, index: isize) -> Result<Token> {
    if index < 0 || index as usize >= tokens.len() {
        return Err(Error::InvalidExpression);
    }

    Ok(tokens.remove(index as usize))
}
