use std::ops::{BitAnd, BitOr, BitXor};

use super::{reqire_args, Function};
use crate::calc::{solver::Context, Num, Result, Token};

macro_rules! bool_func {
    ($name:ident, $func:ident) => {
        pub struct $name;

        impl Function for $name {
            fn name(&self) -> &'static str {
                stringify!($name)
            }

            fn call(&self, args: Vec<Token>, context: &mut Context) -> Result<Num> {
                reqire_args(self.name(), &args, 1)?;
                Ok(context.evaluate(args[0].to_owned())?.$func() as u8 as Num)
            }
        }
    };
}

macro_rules! logic_func {
    ($name:ident, $func:ident) => {
        pub struct $name;

        impl Function for $name {
            fn name(&self) -> &'static str {
                stringify!($name)
            }

            fn call(&self, args: Vec<Token>, context: &mut Context) -> Result<Num> {
                reqire_args(self.name(), &args, 2)?;

                let a = context.evaluate(args[0].to_owned())? > 0 as Num;
                let b = context.evaluate(args[1].to_owned())? > 0 as Num;
                Ok(a.$func(&b) as u8 as Num)
            }
        }
    };
}

bool_func!(IsInfinite, is_infinite);
bool_func!(IsNan, is_nan);
bool_func!(IsFinite, is_finite);
bool_func!(IsSubnormal, is_subnormal);
bool_func!(IsNormal, is_normal);
logic_func!(And, bitand);
logic_func!(Or, bitor);
logic_func!(Xor, bitxor);
logic_func!(Lt, lt);
logic_func!(Le, le);
logic_func!(Gt, gt);
logic_func!(Ge, ge);
logic_func!(Eq, eq);
logic_func!(Ne, ne);

pub struct If;
impl Function for If {
    fn name(&self) -> &'static str {
        "if"
    }

    fn call(&self, args: Vec<Token>, context: &mut Context) -> Result<Num> {
        reqire_args(self.name(), &args, 3)?;
        let cond = context.evaluate(args[0].to_owned())?;
        if cond > 0 as Num {
            return context.evaluate(args[1].to_owned());
        }
        context.evaluate(args[2].to_owned())
    }
}

pub struct Not;
impl Function for Not {
    fn name(&self) -> &'static str {
        "not"
    }

    fn call(&self, args: Vec<Token>, context: &mut Context) -> Result<Num> {
        reqire_args(self.name(), &args, 1)?;
        Ok((context.evaluate(args[0].to_owned())? <= 0 as Num) as u8 as Num)
    }
}
