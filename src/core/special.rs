use std::str::FromStr;
use anyhow::{Result, anyhow, bail};
use edn_rs::{Edn, Edn::*};
use crate::{
    value::{Value, Value::*},
    functions::{Lambda, FnTypes::*},
    environment::Env, eval,
};

pub fn closure(input: &[Value], _env: &mut Env) -> Result<Value> {
    dbg!(&input);
    if let (Some(Expr(args)), Some(Expr(body))) = (input.get(0), input.get(1)) {
        if let Vector(args) = args.clone() {
            return Ok(Fn(Lambda(Lambda{
                capture: true,
                params: args.to_vec(),
                body: body.clone(),
            })));
        }
    }
    Err(anyhow!("Invalid args"))
}

pub fn function(input: &[Value], _env: &mut Env) -> Result<Value> {
    dbg!(&input);
    if let (Some(Expr(args)), Some(Expr(body))) = (input.get(0), input.get(1)) {
        if let Vector(args) = args.clone() {
            return Ok(Fn(Lambda(Lambda{
                capture: false,
                params: args.to_vec(),
                body: body.clone(),
            })));
        }
    }
    Err(anyhow!("Invalid args"))
}

// pub fn recur(input: &[Value], env: &mut Env) -> Result<Value> {
//     let args = input.iter()
//         .filter_map(|edn| eval(edn).ok())
//         .collect::<Vec<Value>>();

//     if let Some(frame) = env.env.pop() {
//         env.env.lambda_assign(&args, &frame.0);
//         let res = env.eval(&frame.0.body);
//         return res;
//     }
//     bail!("Bad input {:?}", input);
// }

pub fn def(input: &[Value], env: &mut Env) -> Result<Value> {
   if input.len() % 2 != 0 {
       bail!("Must provide even args");
   }

   input.iter()
       .enumerate()
       .filter(|x| x.0 % 2 == 0)
       .filter_map(|(i, key)|if let (Some(val), key) = (input.get(i + 1), key) {
           Some((key, val))} else {None})
       .for_each(|(key, val)| if let Ok(val) = eval(val, env) {
           env.define(&key.to_string(), val);
       });
   Ok(Value::Expr(Edn::Nil))
}

pub fn is_truthy(val: &Edn) -> bool {
    match val {
        Edn::Nil         => false,
        Edn::Empty       => false,
        Edn::Bool(bl)    => *bl,
        Edn::Int(num)    => *num != 0isize,
        Edn::UInt(num)   => *num != 0usize,
        Edn::Double(_)   => val.to_float() != Some(0.0),
        Edn::Rational(_) => val.to_float() != Some(0.0),
        Edn::Str(s)      => s.as_str() != "",
        _                => true,
    }
}

pub fn if_statement(input: &[Value], env: &mut Env) -> Result<Value> {
    if let (Some(pred), Some(then), Some(otherwise)) = (input.get(0), input.get(1), input.get(2)) {
        if let Expr(edn) = eval(pred, env)? {
            if is_truthy(&edn) {
                return eval(then, env);
            } else {
                return eval(otherwise, env);
            }
        }
    }
    bail!("Bad input {:?}", input);
}

pub fn quote(input: &[Value], _env: &mut Env) -> Result<Value> {
    if let Some(first) = input.get(0) {
        return Ok(first.clone());
    }
    bail!("Bad input {:?}", input);
}

pub fn progn(input: &[Value], env: &mut Env) -> Result<Value> {
    let mut res: Value = Value::Expr(Edn::Nil);
    for expr in input {
        res = eval(expr, env)?;
    }
    Ok(res)
}

pub fn eval_lisp(input: &[Value], env: &mut Env) -> Result<Value> {
    if let Some(lisp) = input.first() {
        return eval(lisp, env);
    }
    bail!("Bad input {:?}", input);
}

pub fn load_file(input: &[Value], env: &mut Env) -> Result<Value> {
    if let Some(Expr(Str(filename))) = input.first() {
        let lisp = format!("(eval (read (str \"(do \" (slurp \"{}\") \" )\")))", filename);
        return eval(&Expr(Edn::from_str(&lisp)?), env);
    }
    bail!("bad input")
}
