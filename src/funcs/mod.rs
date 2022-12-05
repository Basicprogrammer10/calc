use crate::calc::{solver::Context, Error, Num, Result, Token};

mod basic;
mod logic;
mod math;
mod misc;

// == Misc ==
// cmp
// rand (seedable)

pub const FUNCTIONS: &[&dyn Function] = &[
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
    &basic::Recip,
    &basic::ToDegrees,
    &basic::ToRadians,
    &basic::SigNum,
    &logic::IsInfinite,
    &logic::IsNan,
    &logic::IsFinite,
    &logic::IsSubnormal,
    &logic::IsNormal,
    &logic::If,
    &logic::Not,
    &logic::And,
    &logic::Or,
    &logic::Xor,
    &logic::Lt,
    &logic::Le,
    &logic::Gt,
    &logic::Ge,
    &logic::Eq,
    &logic::Ne,
    &misc::Exit,
    &misc::Dbg,
    &math::Min,
    &math::Max,
    &math::Log,
    &math::Hypot,
    &math::Atan2,
    &math::Clamp,
    &math::Lerp,
    &math::Factorial,
    &math::Gcf,
    &math::Lcm,
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
