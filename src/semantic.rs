use crate::ast::ASTNode;
use crate::symbol_table::SymbolTable;

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
        ASTNode::VariableDeclaration { name } => {
            symbol_table.declare_variable(name);
        }
        ASTNode::Assignment { variable, expression } => {
            if !symbol_table.variables.contains_key(variable) {
                panic!("Variable '{}' is not declared.", variable);
            }
            analyze(expression, symbol_table);
        }
        _ => {}
    }
}
