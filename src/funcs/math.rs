use super::{reqire_args, Function};
use crate::calc::{solver::Context, Num, Result, Token};

macro_rules! multi_func {
    ($name:ident, $func:ident) => {
        pub struct $name;

        impl Function for $name {
            fn name(&self) -> &'static str {
                stringify!($name)
            }

            fn call(&self, args: Vec<Token>, context: &mut Context) -> Result<Num> {
                reqire_args(self.name(), &args, 2)?;
                let a = context.evaluate(args[0].to_owned())?;
                let b = context.evaluate(args[1].to_owned())?;
                Ok(a.$func(b))
            }
        }
    };
}

multi_func!(Min, min);
multi_func!(Max, max);
multi_func!(Log, log);
multi_func!(Hypot, hypot);
multi_func!(Atan2, atan2);

pub struct Clamp;
impl Function for Clamp {
    fn name(&self) -> &'static str {
        "clamp"
    }

    fn call(&self, args: Vec<Token>, context: &mut Context) -> Result<Num> {
        reqire_args(self.name(), &args, 3)?;
        let val = context.evaluate(args[0].to_owned())?;
        let min = context.evaluate(args[1].to_owned())?;
        let max = context.evaluate(args[2].to_owned())?;
        Ok(val.clamp(min, max))
    }
}

pub struct Lerp;
impl Function for Lerp {
    fn name(&self) -> &'static str {
        "lerp"
    }

    fn call(&self, args: Vec<Token>, context: &mut Context) -> Result<Num> {
        reqire_args(self.name(), &args, 3)?;
        let t = context.evaluate(args[0].to_owned())?;
        let a = context.evaluate(args[1].to_owned())?;
        let b = context.evaluate(args[2].to_owned())?;
        Ok(a + (b - a) * t)
    }
}

pub struct Factorial;
impl Function for Factorial {
    fn name(&self) -> &'static str {
        "factorial"
    }

    fn call(&self, args: Vec<Token>, context: &mut Context) -> Result<Num> {
        reqire_args(self.name(), &args, 1)?;
        let mut n = context.evaluate(args[0].to_owned())? as u64;
        let mut result = 1;
        while n > 1 {
            result *= n;
            n -= 1;
        }
        Ok(result as Num)
    }
}

pub struct Gcf;
impl Function for Gcf {
    fn name(&self) -> &'static str {
        "gcf"
    }

    fn call(&self, args: Vec<Token>, context: &mut Context) -> Result<Num> {
        reqire_args(self.name(), &args, 2)?;
        let mut a = context.evaluate(args[0].to_owned())? as u64;
        let mut b = context.evaluate(args[1].to_owned())? as u64;

        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }

        Ok(a as Num)
    }
}

pub struct Lcm;
impl Function for Lcm {
    fn name(&self) -> &'static str {
        "lcm"
    }

    fn call(&self, args: Vec<Token>, context: &mut Context) -> Result<Num> {
        reqire_args(self.name(), &args, 2)?;
        let a = context.evaluate(args[0].to_owned())?;
        let b = context.evaluate(args[1].to_owned())?;
        Ok((a * b / Gcf.call(args, context)?) as Num)
    }
}
