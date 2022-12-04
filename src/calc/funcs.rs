use super::{solver::Context, Error, Num, Result, Token};

pub const FUNCTIONS: &[&dyn Function] = &[&Sqrt];

pub trait Function {
    fn name(&self) -> &'static str;
    fn call(&self, args: Vec<Token>, context: &mut Context) -> Result<Num>;
}

struct Sqrt;

impl Function for Sqrt {
    fn name(&self) -> &'static str {
        "sqrt"
    }

    fn call(&self, args: Vec<Token>, context: &mut Context) -> Result<Num> {
        if args.len() != 1 {
            return Err(Error::InvalidArgumentCount(
                self.name().to_owned(),
                1,
                args.len(),
            ));
        }

        return Ok(context.evaluate(args[0].to_owned())?.sqrt());
    }
}
