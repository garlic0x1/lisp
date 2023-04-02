use crate::{
    value::*,
    value::Value::*,
    functions::FnTypes::*,
};
use anyhow::{Result, bail};
use edn_rs::Edn::*;
use crate::str;

pub fn equals_edn(input: &[Value]) -> Result<Value> {
    if let (Some(Expr(a)), Some(Expr(b))) = (input.get(0), input.get(1)) {
        if a.to_string() == b.to_string() {
            return Ok(Expr(Bool(true)));
        } else {
            return Ok(Expr(Bool(false)));
        }
    }
    bail!("Bad input {:?}", input);
}

pub fn type_of(input: &[Value]) -> Result<Value> {
    if let Some(val) = input.first() {
        return Ok(Expr(Str(
            match val {
                Fn(x) => match x {
                    Lambda(_)  => str!("Lambda"),
                    Native(_)  => str!("Native"),
                    Special(_) => str!("Special"),
                },
                Atom(_)   => str!("Atom"),
                Future(_) => str!("Future"),
                Expr(edn) => match edn {
                    Tagged(..)  => str!("Tagged"),
                    Vector(_)   => str!("Vector"),
                    Set(_)      => str!("Set"),
                    Map(_)      => str!("Map"),
                    List(_)     => str!("List"),
                    Key(_)      => str!("Key"),
                    Symbol(_)   => str!("Symbol"),
                    Str(_)      => str!("Str"),
                    Int(_)      => str!("Int"),
                    UInt(_)     => str!("Int"),
                    Double(_)   => str!("Float"),
                    Rational(_) => str!("Rational"),
                    Char(_)     => str!("Char"),
                    Bool(_)     => str!("Bool"),
                    Inst(_)     => str!("Inst"),
                    Uuid(_)     => str!("Uuid"),
                    Nil         => str!("Nil"),
                    Empty       => str!("Empty"),
                    NamespacedMap(..) => str!("NamespacedMap"),
                }
            }
        )));
    }
    bail!("Bad input {:?}", input);
}
