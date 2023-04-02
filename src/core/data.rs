use std::str::FromStr;
use crate::{value::*, value::Value::*};
use anyhow::{Result, bail};
use edn_rs::Edn::{*, self};

pub fn read_lisp(input: &[Value]) -> Result<Value> {
    if let Some(Expr(Str(lisp))) = input.first() {
        let edn = Edn::from_str(&lisp)?;
        return Ok(Expr(edn))
    }
    bail!("Bad input {:?}", input);
}

pub fn conj(input: &[Value]) -> Result<Value> {
    if let (Some(Expr(Map(a))), Some(Expr(Map(b)))) = (input.get(0), input.get(1)) {
        let mut first  = a.clone().to_map();
        let second = b.clone().to_map();
        for (key, val) in second {
            first.insert(key, val);
        }
        let new = edn_rs::Map::new(first);
        return Ok(Expr(Map(new)));
    }
    bail!("Bad input {:?}", input);
}
pub fn cons(input: &[Value]) -> Result<Value> {
    if let (Some(car), Some(cdr)) = (input.get(0), input.get(1)) {
        if let (Expr(car), Expr(cdr)) = (car, cdr) {
            if let List(list) = cdr {
                let mut v = list.clone().to_vec();
                v.insert(0, car.clone());
                let res = edn_rs::List::new(v);
                return Ok(Expr(List(res)));
            }
        }
    }
    bail!("Bad input {:?}", input);
}

pub fn car(input: &[Value]) -> Result<Value> {
    if let Some(Expr(List(list))) = input.first() {
        if let Some(res) = list.clone().to_vec().first() {
            return Ok(Expr(res.clone()));
        }
    }
    bail!("Bad input {:?}", input);
}

pub fn cdr(input: &[Value]) -> Result<Value> {
    if let Some(Expr(List(list))) = input.first() {
        if let Some((_, rest)) = list.clone().to_vec().split_first() {
            let res = edn_rs::List::new(rest.to_vec());
            return Ok(Expr(List(res)));
        }
    }
    bail!("Bad input {:?}", input);
}

pub fn str_append(input: &[Value]) -> Result<Value> {
    let mut str = String::new();
    for val in input {
        if let Expr(edn) = val {
            str = match edn {
                Str(s) => format!("{}{}", str, s),
                other => format!("{}{}", str, other.to_string()),
            }
        } else {
            str = format!("{}{}", str, val);
        }
    }
    Ok(Expr(Str(str)))
}
