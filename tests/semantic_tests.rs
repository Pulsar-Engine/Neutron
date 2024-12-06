use neutron::semantic::analyze;
use neutron::symbol_table::SymbolTable;
use neutron::ast::{ASTNode};

#[test]
fn test_variable_declaration() {
    let program = ASTNode::Program(vec![
        ASTNode::VariableDeclaration { name: "x".to_string() }
    ]);
    
    let mut symbol_table = SymbolTable::new();

    analyze(&program, &mut symbol_table);

    assert!(symbol_table.variables.contains_key("x"));
}

#[test]
#[should_panic(expected = "Variable 'y' is not declared.")]
fn test_undeclared_variable() {
    let program = ASTNode::Program(vec![
        ASTNode::Assignment {
            variable: "y".to_string(),
            expression: Box::new(ASTNode::Number(42)),
        }
    ]);

    let mut symbol_table = SymbolTable::new();

    analyze(&program, &mut symbol_table);
}
