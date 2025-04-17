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
            symbol_table.enter_scope();

            for param in params {
                symbol_table.declare_variable(param, Type::Int);
            }
            for statement in body {
                analyze(statement, symbol_table);
            }

            symbol_table.exit_scope();
        }
        ASTNode::VariableDeclaration { name, var_type } => {
            symbol_table.declare_variable(name, var_type.clone());
        }
        ASTNode::Assignment { variable, expression } => {
            if let Some(var_type) = symbol_table.get_variable_type(variable) {
                let expr_type = get_expression_type(expression, symbol_table);
                if &expr_type != var_type {
                    panic!(
                        "Type mismatch in assignment to '{}': expected {:?}, got {:?}",
                        variable, var_type, expr_type
                    );
                }
            } else {
                panic!("Variable '{}' not declared", variable);
            }
        
            analyze(expression, symbol_table);
        }        
        ASTNode::Arithmetic { left, right, operator: _ } => {
            analyze(left, symbol_table);
            analyze(right, symbol_table);

            let left_type = get_expression_type(left, symbol_table);
            let right_type = get_expression_type(right, symbol_table);

            if left_type != right_type {
                panic!("Type mismatch in arithmetic: {:?} vs {:?}", left_type, right_type);
            }
        }
        ASTNode::WhileLoop { condition, body } => {
            let cond_type = get_expression_type(condition, symbol_table);
            if cond_type != Type::Bool {
                panic!("Condition in 'while' loop must be boolean, got {:?}", cond_type);
            }
        
            analyze(condition, symbol_table);
        
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
        ASTNode::IfElse { condition, then_block, else_block } => {
            let cond_type = get_expression_type(condition, symbol_table);
            if cond_type != Type::Bool {
                panic!("Condition in 'if' must be boolean, got {:?}", cond_type);
            }
        
            analyze(condition, symbol_table);
            for stmt in then_block {
                analyze(stmt, symbol_table);
            }
            if let Some(else_block) = else_block {
                for stmt in else_block {
                    analyze(stmt, symbol_table);
                }
            }
        }        
        ASTNode::Comparison { left, right, operator: _ } => {
            analyze(left, symbol_table);
            analyze(right, symbol_table);
        
            let left_type = get_expression_type(left, symbol_table);
            let right_type = get_expression_type(right, symbol_table);
        
            if left_type != right_type {
                panic!(
                    "Type mismatch in comparison: {:?} vs {:?}",
                    left_type, right_type
                );
            }
        }
        
        _ => {}
    }
}

fn get_expression_type(expr: &ASTNode, symbol_table: &SymbolTable) -> Type {
    match expr {
        ASTNode::Number(_) => Type::Int,
        ASTNode::Float(_) => Type::Float,
        ASTNode::Boolean(_) => Type::Bool,
        ASTNode::StringLiteral(_) => Type::String,
        ASTNode::Identifier(name) => symbol_table
            .get_variable_type(name)
            .cloned()
            .unwrap_or_else(|| panic!("Undefined variable: {}", name)),
        ASTNode::Arithmetic { left, right, .. } => {
            let left_type = get_expression_type(left, symbol_table);
            let right_type = get_expression_type(right, symbol_table);
            if left_type != right_type {
                panic!(
                    "Type mismatch in arithmetic expression: {:?} vs {:?}",
                    left_type, right_type
                );
            }
            left_type
        }
        ASTNode::Comparison { left, right, .. } => {
            let left_type = get_expression_type(left, symbol_table);
            let right_type = get_expression_type(right, symbol_table);
            if left_type != right_type {
                panic!(
                    "Type mismatch in comparison expression: {:?} vs {:?}",
                    left_type, right_type
                );
            }
            Type::Bool
        }
        _ => panic!("Unsupported expression type in get_expression_type: {:?}", expr),
    }
}
