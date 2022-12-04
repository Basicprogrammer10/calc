use super::{Error, Result, Token};

pub fn create_tree(mut tokens: Vec<Token>) -> Result<Token> {
    if tokens.len() == 1 {
        match tokens.pop().unwrap() {
            Token::Number(i) => return Ok(Token::Number(i)),
            Token::Group(i) => return create_tree(i),
            Token::Var(i) => return Ok(Token::Var(i)),
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

// can be optmised
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
