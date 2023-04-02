use crate::{value::*, value::Value::*};
use anyhow::{Result, bail};
use edn_rs::Edn::*;
use std::convert::TryFrom;

pub fn add(input: &[Value]) -> Result<Value> {
    let mut sum_int: i64   = 0;
    let mut sum_float: f64 = 0.0;
    for val in input.iter() {
        if let Expr(edn) = val {
            match &edn {
                UInt(int) => sum_int += i64::try_from(*int)?,
                Int(int)  => sum_int += i64::try_from(*int)?,
                Double(_)   => sum_float += edn.to_float().unwrap_or_default(),
                Rational(_) => sum_float += edn.to_float().unwrap_or_default(),
                _ => bail!("Bad value {:?}", edn)
            }
        }
    }
    if sum_float == 0.0 {
        Ok(Expr(Int(sum_int as isize)))
    } else {
        Ok(Expr(Double(edn_rs::Double::from(sum_float + sum_int as f64))))
    }
}

pub fn multiply(input: &[Value]) -> Result<Value> {
    let mut sum_int: i64   = 1;
    let mut sum_float: f64 = 1.0;
    for val in input.iter() {
        if let Expr(edn) = val {
            match &edn {
                UInt(int)   => sum_int   *= i64::try_from(*int)?,
                Int(int)    => sum_int   *= i64::try_from(*int)?,
                Double(_)   => sum_float *= edn.to_float().unwrap_or_default(),
                Rational(_) => sum_float *= edn.to_float().unwrap_or_default(),
                _ => bail!("Bad value {:?}", edn)
            }
        }
    }
    if sum_float == 1.0 {
        Ok(Expr(Int(sum_int as isize)))
    } else {
        Ok(Expr(Double(edn_rs::Double::from(sum_float * sum_int as f64))))
    }
}

pub fn divide(input: &[Value]) -> Result<Value> {
    if let (Some(Expr(numer)), Some(Expr(denom))) = (input.get(0), input.get(1)) {
        return Ok(Expr(Double(edn_rs::Double::from(
            numer.to_float().unwrap_or_default() /
                denom.to_float().unwrap_or_default()))));
    }
    bail!("Bad input {:?}", input);
}

pub fn modulo(input: &[Value]) -> Result<Value> {
    if let (Some(Expr(numer)), Some(Expr(denom))) = (input.get(0), input.get(1)) {
        return Ok(Expr(Double(edn_rs::Double::from(
            numer.to_float().unwrap_or_default() %
                denom.to_float().unwrap_or_default()))));
    }
    bail!("Bad input {:?}", input);
}

pub fn cast_int(input: &[Value]) -> Result<Value> {
    if let Some(Expr(arg)) = input.first() {
        return Ok(Expr(Int(arg.to_int().unwrap_or_default())));
    }
    bail!("Bad input {:?}", input);
}
