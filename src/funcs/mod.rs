use crate::calc::{solver::Context, Error, Num, Result, Token};

mod basic;

// todo - log, hypot, atan2
pub const FUNCTIONS: &[&dyn Function] = &[
    &basic::Sqrt,
    &basic::Sin,
    &basic::Cos,
    &basic::Tan,
    &basic::Asin,
    &basic::Acos,
    &basic::Atan,
    &basic::Sinh,
    &basic::Cosh,
    &basic::Tanh,
    &basic::Asinh,
    &basic::Acosh,
    &basic::Atanh,
    &basic::Floor,
    &basic::Ceil,
    &basic::Round,
    &basic::Trunc,
    &basic::Fract,
    &basic::Abs,
    &basic::Exp,
    &basic::Exp2,
    &basic::Ln,
    &basic::Log2,
    &basic::Log10,
    &basic::Cbrt,
    &basic::ExpM1,
    &basic::Ln1p,
];

pub trait Function {
    fn name(&self) -> &'static str;
    fn call(&self, args: Vec<Token>, context: &mut Context) -> Result<Num>;
}

pub fn reqire_args(name: &str, args: &[Token], count: usize) -> Result<()> {
    if args.len() != count {
        return Err(Error::InvalidArgumentCount(
            name.to_owned(),
            args.len(),
            count,
        ));
    }

    Ok(())
}
