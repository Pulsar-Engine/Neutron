use std::collections::HashMap;

#[derive(Debug)]
pub struct SymbolTable {
    pub variables: HashMap<String, i64>,
    pub functions: HashMap<String, Vec<String>>,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            variables: HashMap::new(),
            functions: HashMap::new(),
        }
    }

    pub fn declare_variable(&mut self, name: &str) {
        if self.variables.contains_key(name) {
            panic!("Variable '{}' is already declared.", name);
        }
        self.variables.insert(name.to_string(), 0);
    }

    pub fn declare_function(&mut self, name: &str, params: Vec<String>) {
        if self.functions.contains_key(name) {
            panic!("Function '{}' is already declared.", name);
        }
        self.functions.insert(name.to_string(), params);
    }
}
