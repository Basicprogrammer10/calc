use super::{Error, Num, Ops, Result, Token};

struct TokenizeContext {
    out: Vec<Token>,
    working: String,

    next_neg: bool,
    in_group: bool,
    is_num: bool,
}

pub fn tokenize(inp: &str) -> Result<Vec<Token>> {
    let mut ctx = TokenizeContext::new();

    for i in inp.chars() {
        match i {
            i if i.is_whitespace() => continue,

            // Groups
            '(' => ctx.in_group = true,
            ')' => {
                ctx.in_group = false;
                ctx.out.push(Token::Group(tokenize(&ctx.working)?));
                ctx.working.clear();
            }
            i if ctx.in_group => ctx.working.push(i),

            // Operations
            '-' => {
                flush_working(&mut ctx)?;
                if ctx.out.is_empty() || matches!(ctx.out.last(), Some(Token::Op(_))) {
                    ctx.next_neg ^= true;
                    continue;
                }
                ctx.out.push(Token::Op(Ops::Sub));
            }
            '+' => add_op(Ops::Add, &mut ctx)?,
            '*' => add_op(Ops::Mul, &mut ctx)?,
            '/' => add_op(Ops::Div, &mut ctx)?,
            '^' => add_op(Ops::Pow, &mut ctx)?,

            // Numbers
            _ => {
                ctx.working.push(i);
                ctx.is_num &= is_digit(i);
            }
        }
    }

    if !ctx.working.is_empty() {
        add_num(&mut ctx)?;
    }

    Ok(ctx.out)
}

impl TokenizeContext {
    fn new() -> Self {
        Self {
            out: Vec::new(),
            working: String::new(),

            next_neg: false,
            in_group: false,
            is_num: true,
        }
    }
}

fn is_digit(chr: char) -> bool {
    matches!(chr, '0'..='9' | '.')
}

fn add_num(ctx: &mut TokenizeContext) -> Result<()> {
    if !ctx.is_num {
        ctx.out.push(Token::Var(ctx.working.to_string()));
        return Ok(());
    }

    ctx.out
        .push(Token::Number(match ctx.working.parse::<Num>() {
            Ok(i) => i.copysign(if ctx.next_neg { -1. } else { 0. }),
            Err(_) => return Err(Error::InvalidNumber(ctx.working.to_string())),
        }));
    Ok(())
}

fn flush_working(ctx: &mut TokenizeContext) -> Result<()> {
    if !ctx.working.is_empty() {
        add_num(ctx)?;
        ctx.next_neg = false;
        ctx.is_num = true;
        ctx.working.clear();
    }
    Ok(())
}

fn add_op(op: Ops, ctx: &mut TokenizeContext) -> Result<()> {
    flush_working(ctx)?;
    ctx.out.push(Token::Op(op));
    Ok(())
}
