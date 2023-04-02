use crate::{value::Value, value::Value::*};
use anyhow::{Result, bail};
use edn_rs::Edn::*;

pub fn slurp(input: &[Value]) -> Result<Value> {
    if let Some(Expr(Str(filename))) = input.first() {
        let text = std::fs::read_to_string(filename)?;
        return Ok(Expr(Str(text)));
    }
    bail!("Bad input {:?}", input);
}


pub fn print_line(input: &[Value]) -> Result<Value> {
    println!("{}", input.first().unwrap_or(&Expr(Nil)));
    Ok(Expr(Nil))
}

pub fn dbg_line(input: &[Value]) -> Result<Value> {
    println!("{:?}", input.first().unwrap_or(&Expr(Nil)));
    Ok(Expr(Nil))
}
