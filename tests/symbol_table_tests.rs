use neutron::symbol_table::SymbolTable;

#[test]
fn test_declare_variable() {
    let mut symbol_table = SymbolTable::new();
    symbol_table.declare_variable("x");

    assert!(symbol_table.variables.contains_key("x"));
    assert_eq!(symbol_table.variables.get("x").unwrap(), &0);
}

#[test]
fn test_declare_function() {
    let mut symbol_table = SymbolTable::new();
    symbol_table.declare_function("myFunc", vec!["param1".to_string()]);

    assert!(symbol_table.functions.contains_key("myFunc"));
    assert_eq!(
        symbol_table.functions.get("myFunc").unwrap(),
        &vec!["param1".to_string()]
    );
}
