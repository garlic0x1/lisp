use crate::value::*;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Frame {
    pub capture: bool,
    pub values: HashMap<String, Value>,
}

pub type Stack = Vec<Frame>;

#[derive(Clone, Debug)]
pub struct Env {
    pub stack:  Stack,
    pub global: HashMap<String, Value>
}

impl Env {
    pub fn from_core(global: HashMap<String, Value>) -> Self {
        Self {stack: vec![], global}
    }

    pub fn define(&mut self, key: &String, val: Value) {
        self.global.insert(key.clone(), val);
    }

    pub fn find(&self, name: &String) -> Option<&Value> {
        let stack = self.stack
            .iter()
            .rev()
            .take_while(|f| f.capture)
            .find_map(|f| f.values.get(name));

        if stack.is_some() {return stack};
        self.global.get(name)
    }
}
