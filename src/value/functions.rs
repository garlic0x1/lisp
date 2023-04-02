use std::collections::HashMap;

use edn_rs::Edn;
use anyhow::Result;
use crate::{environment::*, eval};
use crate::value::Value;

#[derive(Clone)]
pub enum FnTypes {
    Native(Box<Fn>),
    Special(Box<Proc>),
    Lambda(Lambda),
}

// native functions
pub type Fn = fn(&[Value]) -> Result<Value>;
pub type Proc = fn(&[Value], env: &mut Env) -> Result<Value>;

// lisp functions
#[derive(Clone, Debug)]
pub struct Lambda {
    pub capture: bool,
    pub params: Vec<Edn>,
    pub body: Edn,
}

impl Lambda {
    pub fn apply(&self, args: &[Value], env: &mut Env) -> Result<Value> {
        let zip = self.params.iter().zip(args.iter());
        let values: HashMap<String, Value> =
            zip.map(|x| (x.0.to_string(), x.1.clone())).collect();
        env.stack.push(Frame{capture: self.capture, values});
        let res = eval(&Value::Expr(self.body.clone()), env);
        env.stack.pop();
        res
    }
}
