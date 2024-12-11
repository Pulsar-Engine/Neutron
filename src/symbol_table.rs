use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Type {
    Int,
    Float,
    String,
    Bool,
}

#[derive(Debug)]
pub struct SymbolTable {
    pub variables: HashMap<String, Type>,
    pub functions: HashMap<String, Vec<String>>,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            variables: HashMap::new(),
            functions: HashMap::new(),
        }
    }

    pub fn declare_variable(&mut self, name: &str, var_type: Type) {
        if self.variables.contains_key(name) {
            panic!("Variable '{}' is already declared.", name);
        }
        self.variables.insert(name.to_string(), var_type);
    }

    pub fn get_variable_type(&self, name: &str) -> Option<&Type> {
        self.variables.get(name)
    }

    pub fn declare_function(&mut self, name: &str, params: Vec<String>) {
        if self.functions.contains_key(name) {
            panic!("Function '{}' is already declared.", name);
        }
        self.functions.insert(name.to_string(), params);
    }
}
