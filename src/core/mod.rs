pub mod special;
pub mod math;
pub mod conditional;
pub mod data;
pub mod io;

use crate::{
    core::{
        special::*,
        math::*,
        conditional::*,
        data::*,
        io::*,
    },
    value::*,
    value::Value::*,
    functions::FnTypes::*,
};
use edn_rs::Edn;
use maplit::hashmap;
use std::collections::HashMap;

#[macro_export]
macro_rules! str {
    () => {
        String::new()
    };
    ($x:expr $(,)?) => {
        ToString::to_string(&$x)
    };
}

pub fn core() -> HashMap<String, Value> {
    let f = |it| Fn(Native(Box::new(it)));
    let s = |it| Fn(Special(Box::new(it)));
    hashmap! {
        str!("λ")      => s(closure),
        str!("lambda") => s(closure),
        str!("fn")     => s(function),
        // str!("recur")  => s(recur),
        str!("def")    => s(def),
        str!("if")     => s(if_statement),
        str!("do")     => s(progn),
        str!("quote")  => s(quote),
        str!("eval")   => s(eval_lisp),
        str!("load")   => s(load_file),
        str!("true")  => Value::Expr(Edn::Bool(true)),
        str!("false") => Value::Expr(Edn::Bool(false)),
        str!("+")       => f(add),
        str!("*")       => f(multiply),
        str!("/")       => f(divide),
        str!("mod")     => f(modulo),
        str!("int")     => f(cast_int),
        str!("type")    => f(type_of),
        str!("conj")    => f(conj),
        str!("cons")    => f(cons),
        str!("car")     => f(car),
        str!("cdr")     => f(cdr),
        str!("read")    => f(read_lisp),
        str!("slurp")   => f(slurp),
        str!("println") => f(print_line),
        str!("dbg")     => f(dbg_line),
        str!("str")     => f(str_append),
        str!("=")       => f(equals_edn),
    }
}
