use crate::value::Value::*;
use crate::value::functions::FnTypes::*;
use crate::{environment::*, value::*};
use anyhow::{Result, bail, anyhow};
use edn_rs::{Edn, Edn::*, List};

pub fn eval(input: &Value, env: &mut Env) -> Result<Value> {
    match input {
        Value::Expr(edn) => match edn {
            Edn::List(list) => eval_list(list, env),
            Edn::Symbol(sym) => Ok(env.find(sym).unwrap_or(&NIL).clone()),
            _ => Ok(Value::Expr(edn.clone()))
        }
        _ => Err(anyhow!(" is not an expression")),
    }
}

fn eval_list(list: &List, env: &mut Env) -> Result<Value> {
    if let Some((first, rest)) = list.clone().to_vec().split_first() {
        let first = eval(&Expr(first.clone()), env)?;
        let args: Vec<Value> = rest.iter().map(|e| Expr(e.clone())).collect();
        let mut eval_args = || -> Vec<Value> {
            args.iter().filter_map(|x| eval(x, env).ok()).collect()
        };
        match first {
            Fn(x) => return match x {
                Special(proc) => proc(&args, env),
                Native(fun) => fun(&eval_args()),
                Lambda(lambda) => lambda.apply(&eval_args(), env),
            },
            Expr(Key(key)) => {
                if let Some(map) = rest.first() {
                    if let Expr(Map(map)) = eval(&Expr(map.clone()), env)? {
                        let btree = map.to_map();
                        let val = btree.get(&key).unwrap_or(&Nil);
                        let res = eval(&Expr(val.clone()), env)?;
                        return Ok(res);
                    }
                }
            }
            _ => bail!("{:?} is not an operator", first)
        };
    }
    bail!("Empty list");
}
