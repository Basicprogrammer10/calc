use crate::calc::{solver::Context, Num, Result, Token};

use super::{reqire_args, Function};

macro_rules! basic_func {
    ($name:ident, $func:ident) => {
        pub struct $name;

        impl Function for $name {
            fn name(&self) -> &'static str {
                stringify!($name)
            }

            fn call(&self, args: Vec<Token>, context: &mut Context) -> Result<Num> {
                reqire_args(self.name(), &args, 1)?;
                Ok(context.evaluate(args[0].to_owned())?.$func())
            }
        }
    };
}

basic_func!(Floor, floor);
basic_func!(Ceil, ceil);
basic_func!(Round, round);
basic_func!(Trunc, trunc);
basic_func!(Fract, fract);
basic_func!(Abs, abs);
basic_func!(Exp, exp);
basic_func!(Exp2, exp2);
basic_func!(Ln, ln);
basic_func!(Log2, log2);
basic_func!(Log10, log10);
basic_func!(Cbrt, cbrt);
basic_func!(ExpM1, exp_m1);
basic_func!(Ln1p, ln_1p);

basic_func!(Sqrt, sqrt);
basic_func!(Sin, sin);
basic_func!(Cos, cos);
basic_func!(Tan, tan);
basic_func!(Asin, asin);
basic_func!(Acos, acos);
basic_func!(Atan, atan);
basic_func!(Sinh, sinh);
basic_func!(Cosh, cosh);
basic_func!(Tanh, tanh);
basic_func!(Asinh, asinh);
basic_func!(Acosh, acosh);
basic_func!(Atanh, atanh);
