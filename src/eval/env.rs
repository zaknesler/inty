use crate::core::*;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub struct Environment {
    table: HashMap<String, Value>,
    parent: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            table: HashMap::new(),
            parent: None,
        }
    }

    pub fn new_with_parent(parent: Rc<RefCell<Environment>>) -> Self {
        Self {
            table: HashMap::new(),
            parent: Some(parent),
        }
    }

    pub fn wrap(self) -> Self {
        Self {
            table: HashMap::new(),
            parent: Some(Rc::new(RefCell::new(self))),
        }
    }

    /// Try to get a value from the environment by its identifier
    pub fn get(&self, ident: String) -> Option<Value> {
        match self.table.get(&ident) {
            Some(val) => Some(val.clone()),
            None => match &self.parent {
                Some(parent) => parent.borrow().get(ident),
                None => None,
            },
        }
    }

    /// Does the environment contain a certain identifier
    pub fn has(&self, ident: String) -> bool {
        self.table.contains_key(&ident)
    }

    /// Insert a new value into the environment
    pub fn put(&mut self, ident: String, value: Value) -> Option<Value> {
        self.table.insert(ident, value)
    }
}
