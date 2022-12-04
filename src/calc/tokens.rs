use super::{Error, Num, Ops, Result, Token};

pub fn tokenize(inp: &str) -> Result<Vec<Token>> {
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

fn is_digit(chr: char) -> bool {
    matches!(chr, '0'..='9' | '.')
}

fn add_num(out: &mut Vec<Token>, working: &str, sign: bool) -> Result<()> {
    out.push(Token::Number(match working.parse::<Num>() {
        Ok(i) => i.copysign(if sign { -1. } else { 0. }),
        Err(_) => return Err(Error::InvalidNumber(working.to_string())),
    }));
    Ok(())
}
