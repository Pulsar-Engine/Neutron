use std::collections::HashMap;
use crate::ast::ASTNode;

#[derive(Debug, Clone)]
pub enum Value {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Void,
    Return(Box<Value>),
}

#[derive(Debug, Clone)]
pub struct FunctionInfo {
    pub params: Vec<String>,
    pub body: Vec<ASTNode>,
}

pub struct Interpreter {
    pub variables: HashMap<String, Value>,
    pub functions: HashMap<String, FunctionInfo>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            functions: HashMap::new(),
        }
    }

    pub fn interpret(&mut self, node: &ASTNode) -> Value {
        match node {
            ASTNode::Program(nodes) => {
                for stmt in nodes {
                    self.interpret(stmt);
                }

                if self.functions.contains_key("run") {
                    self.call_function("run", vec![])
                } else {
                    Value::Void
                }
            }

            ASTNode::ClassDeclaration { name: _, members } => {
                for member in members {
                    self.interpret(member);
                }
                Value::Void
            }

            ASTNode::FunctionDeclaration { name, params, body } => {
                self.functions.insert(
                    name.clone(),
                    FunctionInfo {
                        params: params.clone(),
                        body: body.clone(),
                    },
                );
                Value::Void
            }

            ASTNode::VariableDeclaration { name, .. } => {
                self.variables.insert(name.clone(), Value::Void);
                Value::Void
            }

            ASTNode::Assignment { variable, expression } => {
                let value = self.interpret(expression);
                self.variables.insert(variable.clone(), value);
                Value::Void
            }

            ASTNode::Identifier(name) => {
                self.variables.get(name).cloned().unwrap_or_else(|| {
                    panic!("Runtime error: undefined variable '{}'", name)
                })
            }

            ASTNode::Number(n) => Value::Int(*n),
            ASTNode::Float(f) => Value::Float(*f),
            ASTNode::Boolean(b) => Value::Bool(*b),
            ASTNode::StringLiteral(s) => Value::String(s.clone()),

            ASTNode::Arithmetic { left, operator, right } => {
                let l = self.interpret(left);
                let r = self.interpret(right);
                match (l, r, operator.as_str()) {
                    (Value::Int(a), Value::Int(b), "+") => Value::Int(a + b),
                    (Value::Int(a), Value::Int(b), "-") => Value::Int(a - b),
                    (Value::Int(a), Value::Int(b), "*") => Value::Int(a * b),
                    (Value::Int(a), Value::Int(b), "/") => Value::Int(a / b),
                    (Value::Float(a), Value::Float(b), "+") => Value::Float(a + b),
                    (Value::Float(a), Value::Float(b), "-") => Value::Float(a - b),
                    (Value::Float(a), Value::Float(b), "*") => Value::Float(a * b),
                    (Value::Float(a), Value::Float(b), "/") => Value::Float(a / b),
                    _ => panic!("Unsupported arithmetic operation"),
                }
            }

            ASTNode::Ret { expression } => {
                let value = self.interpret(expression);
                Value::Return(Box::new(value))
            }

            ASTNode::IfElse { condition, then_block, else_block } => {
                let cond = self.interpret(condition);
            
                let truthy = match cond {
                    Value::Bool(b) => b,
                    _ => panic!("Runtime error: 'if' condition must be a boolean"),
                };

                let empty_block: Vec<ASTNode> = vec![];
            
                let block = if truthy {
                    then_block 
                } else {
                    else_block.as_ref().unwrap_or(&empty_block)
                };
            
                for stmt in block {
                    let result = self.interpret(stmt);
                    if let Value::Return(_) = result {
                        return result;
                    }
                }
            
                Value::Void
            }

            ASTNode::Comparison { left, operator, right } => {
                let l = self.interpret(left);
                let r = self.interpret(right);
            
                match (l, r, operator.as_str()) {
                    (Value::Int(a), Value::Int(b), "<") => Value::Bool(a < b),
                    (Value::Int(a), Value::Int(b), ">") => Value::Bool(a > b),
                    (Value::Int(a), Value::Int(b), "==") => Value::Bool(a == b),
            
                    (Value::Float(a), Value::Float(b), "<") => Value::Bool(a < b),
                    (Value::Float(a), Value::Float(b), ">") => Value::Bool(a > b),
                    (Value::Float(a), Value::Float(b), "==") => Value::Bool(a == b),
            
                    (Value::Bool(a), Value::Bool(b), "==") => Value::Bool(a == b),
                    (Value::String(a), Value::String(b), "==") => Value::Bool(a == b),
            
                    _ => panic!("Invalid comparison between incompatible types"),
                }
            }
            
            ASTNode::WhileLoop { condition, body } => {
                loop {
                    let cond_val = self.interpret(condition);
                    let is_true = match cond_val {
                        Value::Bool(b) => b,
                        _ => panic!("Runtime error: while condition must be a boolean"),
                    };
            
                    if !is_true {
                        break;
                    }
            
                    for stmt in body {
                        let result = self.interpret(stmt);
                        if let Value::Return(_) = result {
                            return result;
                        }
                    }
                }
            
                Value::Void
            }

            ASTNode::ForLoop { variable, start, end, body } => {
                let start_val = self.interpret(start);
                let end_val = self.interpret(end);
            
                let (start_i, end_i) = match (start_val, end_val) {
                    (Value::Int(s), Value::Int(e)) => (s, e),
                    _ => panic!("Runtime error: 'for' loop range must be integers"),
                };
            
                let old_var = self.variables.get(variable).cloned();
            
                for i in start_i..end_i {
                    self.variables.insert(variable.clone(), Value::Int(i));
            
                    for stmt in body {
                        let result = self.interpret(stmt);
                        if let Value::Return(_) = result {
                            if let Some(v) = old_var {
                                self.variables.insert(variable.clone(), v);
                            } else {
                                self.variables.remove(variable);
                            }
                            return result;
                        }
                    }
                }
            
                if let Some(v) = old_var {
                    self.variables.insert(variable.clone(), v);
                } else {
                    self.variables.remove(variable);
                }
            
                Value::Void
            }

            ASTNode::FunctionCall { name, args } => {
                let evaluated_args: Vec<Value> = args.iter()
                    .map(|arg| self.interpret(arg))
                    .collect();
            
                self.call_function(name, evaluated_args)
            }            

            _ => panic!("Interpretation not yet implemented for: {:?}", node),
        }
    }

    pub fn call_function(&mut self, name: &str, args: Vec<Value>) -> Value {
        let func = self.functions.get(name).cloned().expect(&format!("Function '{}' not found", name));

        if args.len() != func.params.len() {
            panic!(
                "Function '{}' expects {} arguments, got {}",
                name, func.params.len(), args.len()
            );
        }

        let old_vars = self.variables.clone();

        for (param, arg) in func.params.iter().zip(args) {
            self.variables.insert(param.clone(), arg);
        }

        let mut return_value = Value::Void;

        let body = func.body.clone();

        for stmt in &body {
            let value = self.interpret(stmt);
            if let Value::Return(inner) = value {
                return_value = *inner;
                break;
            }
        }

        self.variables = old_vars;
        return_value
    }
}
