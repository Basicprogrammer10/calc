use std::process;

use super::{reqire_args, Function};
use crate::calc::{solver::Context, Num, Result, Token};

pub struct Exit;
impl Function for Exit {
    fn name(&self) -> &'static str {
        "exit"
    }

    fn call(&self, args: Vec<Token>, context: &mut Context) -> Result<Num> {
        let code = match args.get(0) {
            Some(arg) => context.evaluate(arg.to_owned())? as i32,
            None => 0,
        };
        process::exit(code as i32);
    }
}

pub struct Dbg;
impl Function for Dbg {
    fn name(&self) -> &'static str {
        "dbg"
    }

    fn call(&self, args: Vec<Token>, context: &mut Context) -> Result<Num> {
        reqire_args(self.name(), &args, 1)?;
        let val = context.evaluate(args[0].to_owned())?;
        println!(" [DEBUG] {} = {}", args[0], val);
        Ok(val)
    }
}
