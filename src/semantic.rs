use crate::ast::ASTNode;
use crate::symbol_table::{SymbolTable, Type};

pub fn analyze(ast: &ASTNode, symbol_table: &mut SymbolTable) {
    match ast {
        ASTNode::Program(nodes) => {
            for node in nodes {
                analyze(node, symbol_table);
            }
        }
        ASTNode::ClassDeclaration { name: _, members } => {
            for member in members {
                analyze(member, symbol_table);
            }
        }
        ASTNode::FunctionDeclaration { name, params, body } => {
            symbol_table.declare_function(name, params.clone());
            for statement in body {
                analyze(statement, symbol_table);
            }
        }
        ASTNode::VariableDeclaration { name, var_type } => {
            symbol_table.declare_variable(name, var_type.clone());
        }
        ASTNode::Assignment { variable, expression } => {
            if let Some(var_type) = symbol_table.get_variable_type(variable) {
                match expression.as_ref() {
                    ASTNode::Number(_val) => {
                        if !matches!(var_type, Type::Int) {
                            panic!("Type mismatch: Expected {:?}, got number", var_type);
                        }
                    }
                    ASTNode::Identifier(id) => {
                        if let Some(expr_type) = symbol_table.get_variable_type(id) {
                            if expr_type != var_type {
                                panic!("Type mismatch: Expected {:?}, got {:?}", var_type, expr_type);
                            }
                        } else {
                            panic!("Undefined variable: {}", id);
                        }
                    }
                    _ => {
                        panic!("Unhandled expression type: {:?}", expression);
                    }
                }
            } else {
                panic!("Variable '{}' not declared", variable);
            }

            analyze(expression, symbol_table);
        }
        ASTNode::WhileLoop { condition, body } => {
            if let ASTNode::Identifier(name) = condition.as_ref() {
                if let Some(var_type) = symbol_table.get_variable_type(name) {
                    if *var_type != Type::Bool {
                        panic!("Condition in 'while' loop must be a boolean");
                    }
                } else {
                    panic!("Undefined variable: {}", name);
                }
            } else {
                panic!("Invalid condition in 'while' loop");
            }
            for statement in body {
                analyze(statement, symbol_table);
            }
        }
        ASTNode::ForLoop { variable, start, end, body } => {
            let start_type = match start.as_ref() {
                ASTNode::Number(_) => Type::Int,
                ASTNode::Identifier(name) => symbol_table
                    .get_variable_type(name)
                    .cloned()
                    .unwrap_or_else(|| panic!("Undefined variable: {}", name)),
                _ => panic!("Invalid start expression in 'for' loop"),
            };

            let end_type = match end.as_ref() {
                ASTNode::Number(_) => Type::Int,
                ASTNode::Identifier(name) => symbol_table
                    .get_variable_type(name)
                    .cloned()
                    .unwrap_or_else(|| panic!("Undefined variable: {}", name)),
                _ => panic!("Invalid end expression in 'for' loop"),
            };
            if start_type != Type::Int || end_type != Type::Int {
                panic!("Start and end expressions in 'for' loop must be integers");
            }
            symbol_table.declare_variable(variable, Type::Int);
            for statement in body {
                analyze(statement, symbol_table);
            }
        }
        _ => {}
    }
}
