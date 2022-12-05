use super::{solver::Context, Error, Num, Result, Token};

pub const FUNCTIONS: &[&dyn Function] = &[&Sqrt];

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

struct Sqrt;

impl Function for Sqrt {
    fn name(&self) -> &'static str {
        "sqrt"
    }

    fn call(&self, args: Vec<Token>, context: &mut Context) -> Result<Num> {
        reqire_args(self.name(), &args, 1)?;
        Ok(context.evaluate(args[0].to_owned())?.sqrt())
    }
}
