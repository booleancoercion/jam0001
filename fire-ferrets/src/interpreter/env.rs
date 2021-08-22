use std::collections::HashMap;

use super::value::Value;
use super::ValueResult;

pub struct Env {
    store: HashMap<String, Value>,
    stack: Vec<Value>,
}

impl Env {
    pub fn new() -> Self {
        Self {
            stack: vec![],
            store: HashMap::new(),
        }
    }

    pub fn set(&mut self, name: String, value: Value) {
        self.store.insert(name, value);
    }

    pub fn get(&self, name: &str) -> ValueResult {
        self.store
            .get(name)
            .map(Clone::clone)
            .ok_or(format!("{} is undefined", name))
    }

    pub fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    pub fn pop(&mut self) -> ValueResult {
        self.stack
            .pop()
            .ok_or_else(|| "The stack is empty".to_string())
    }
}
