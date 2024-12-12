use std::{collections::{HashMap, HashSet}, hash::Hash};

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
    pub local_variables_stack: Vec<HashSet<String>>,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            variables: HashMap::new(),
            functions: HashMap::new(),
            local_variables_stack: vec![HashSet::new()],
        }
    }

    pub fn declare_variable(&mut self, name: &str, var_type: Type) {
        if self.local_variables_stack.last().unwrap().contains(name) {
            panic!("Variable '{}' is already declared in the current scope.", name);
        }
        if self.variables.contains_key(name) {
            return;
        }
        self.local_variables_stack.last_mut().unwrap().insert(name.to_string());
        self.variables.insert(name.to_string(), var_type);
    }

    pub fn get_variable_type(&self, name: &str) -> Option<&Type> {
        if self.local_variables_stack.last().unwrap().contains(name) {
            self.variables.get(name)
        } else {
            None
        }
    }

    pub fn exit_scope(&mut self) {
        self.local_variables_stack.pop();
    }

    pub fn enter_scope(&mut self) {
        self.local_variables_stack.push(HashSet::new());
    }

    pub fn declare_function(&mut self, name: &str, params: Vec<String>) {
        if self.functions.contains_key(name) {
            panic!("Function '{}' is already declared.", name);
        }
        self.functions.insert(name.to_string(), params);
    }
}
