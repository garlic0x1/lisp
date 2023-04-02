pub mod functions;
use functions::FnTypes::*;
use std::{future::Future, sync::Arc, fmt};
use edn_rs::Edn;
use Value::*;

use self::functions::FnTypes;

pub type Fut = Arc<dyn Future<Output = Value>>;
pub const NIL: Value = Value::Expr(Edn::Nil);

#[derive(Clone)]
pub enum Value {
    Fn(FnTypes),
    Future(Fut),
    Atom(Arc<Value>),
    Expr(Edn),
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr(x)   => write!(f, "{:?}", x),
            Atom(x)   => write!(f, "{:?}", x),
            Future(_) => write!(f, "#future"),
            Fn(x) => match x {
                Lambda(x)  => write!(f, "{:?}", x),
                Native(_)  => write!(f, "#native"),
                Special(_) => write!(f, "#special"),
            }
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr(x)   => write!(f, "{}", x.to_string()),
            Atom(x)   => write!(f, "(atom {})", x.to_string()),
            Future(_) => write!(f, "#future"),
            Fn(x) => match x {
                Lambda(x)  => write!(f, "{:?}", x),
                Native(_)  => write!(f, "#native"),
                Special(_) => write!(f, "#special"),
            }
        }
    }
}
