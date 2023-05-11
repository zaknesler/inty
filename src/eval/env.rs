use std::collections::HashMap;

pub struct Environment {
    table: Box<HashMap<String, i32>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            table: Box::new(HashMap::new()),
        }
    }

    /// Try to get a value from the environment by its identifier
    pub fn get(&self, ident: String) -> Option<&i32> {
        self.table.get(&ident)
    }

    /// Does the environment contain a certain identifier
    pub fn has(&self, ident: String) -> bool {
        self.table.contains_key(&ident)
    }

    /// Insert a new value into the environment
    pub fn put(&mut self, ident: String, value: i32) -> Option<i32> {
        self.table.insert(ident, value)
    }
}
