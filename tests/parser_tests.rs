use neutron::lexer::Lexer;
use neutron::parser::Parser;
use neutron::ast::ASTNode;
use neutron::symbol_table::Type;

#[test]
fn test_parse_program() {
    let input = "class MyClass then var x int end";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let ast = parser.parse_program();

    match ast {
        ASTNode::Program(nodes) => {
            assert_eq!(nodes.len(), 1);
            match &nodes[0] {
                ASTNode::ClassDeclaration { name, members } => {
                    assert_eq!(name, "MyClass");
                    assert_eq!(members.len(), 1);
                }
                _ => panic!("Expected a ClassDeclaration"),
            }
        }
        _ => panic!("Expected a Program ASTNode"),
    }
}

#[test]
fn test_parse_class_declaration() {
    let input = "class MyClass then var x int end";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let ast = parser.parse_program();

    match ast {
        ASTNode::Program(nodes) => {
            assert_eq!(nodes.len(), 1);
            match &nodes[0] {
                ASTNode::ClassDeclaration { name, members } => {
                    assert_eq!(name, "MyClass");
                    assert_eq!(members.len(), 1);
                    match &members[0] {
                        ASTNode::VariableDeclaration { name, ..  } => {
                            assert_eq!(name, "x");
                        }
                        _ => panic!("Expected a VariableDeclaration"),
                    }
                }
                _ => panic!("Expected a ClassDeclaration"),
            }
        }
        _ => panic!("Expected a Program ASTNode"),
    }
}

#[test]
fn test_parse_function_declaration() {
    let input = "func myFunc(a, b) then var x int end";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let ast = parser.parse_program();

    match ast {
        ASTNode::Program(nodes) => {
            assert_eq!(nodes.len(), 1);
            match &nodes[0] {
                ASTNode::FunctionDeclaration { name, params, body } => {
                    assert_eq!(name, "myFunc");
                    assert_eq!(params.len(), 2);
                    assert_eq!(params[0], "a");
                    assert_eq!(params[1], "b");
                    assert_eq!(body.len(), 1);
                    match &body[0] {
                        ASTNode::VariableDeclaration { name, .. } => {
                            assert_eq!(name, "x");
                        }
                        _ => panic!("Expected a VariableDeclaration"),
                    }
                }
                _ => panic!("Expected a FunctionDeclaration"),
            }
        }
        _ => panic!("Expected a Program ASTNode"),
    }
}

#[test]
fn test_parse_variable_declaration() {
    let input = "var x int";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let ast = parser.parse_program();

    match ast {
        ASTNode::Program(nodes) => {
            assert_eq!(nodes.len(), 1);
            match &nodes[0] {
                ASTNode::VariableDeclaration { name, var_type } => {
                    assert_eq!(name, "x");
                    assert_eq!(var_type, &Type::Int);
                }
                _ => panic!("Expected a VariableDeclaration"),
            }
        }
        _ => panic!("Expected a Program ASTNode"),
    }
}

#[test]
fn test_parse_assignment() {
    let input = "x = 5";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let ast = parser.parse_program();

    match ast {
        ASTNode::Program(nodes) => {
            assert_eq!(nodes.len(), 1);
            match &nodes[0] {
                ASTNode::Assignment { variable, expression } => {
                    assert_eq!(variable, "x");
                    match **expression {
                        ASTNode::Number(value) => {
                            assert_eq!(value, 5);
                        }
                        _ => panic!("Expected a Number in the expression"),
                    }
                }
                _ => panic!("Expected an Assignment"),
            }
        }
        _ => panic!("Expected a Program ASTNode"),
    }
}

#[test]
fn test_parse_binary_operations() {
    let input = "x = 3 + 4 * 2";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let ast = parser.parse_program();

    match ast {
        ASTNode::Program(nodes) => {
            assert_eq!(nodes.len(), 1);
            match &nodes[0] {
                ASTNode::Assignment { variable, expression } => {
                    assert_eq!(variable, "x");
                    match **expression {
                        ASTNode::Arithmetic { ref operator, ref left, ref right } => {
                            assert_eq!(operator, "+");
                            match **left {
                                ASTNode::Number(value) => assert_eq!(value, 3),
                                _ => panic!("Expected a Number on the left side"),
                            }
                            match **right {
                                ASTNode::Arithmetic { ref operator, ref left, ref right } => {
                                    assert_eq!(operator, "*");
                                    match **left {
                                        ASTNode::Number(value) => assert_eq!(value, 4),
                                        _ => panic!("Expected a Number on the left side"),
                                    }
                                    match **right {
                                        ASTNode::Number(value) => assert_eq!(value, 2),
                                        _ => panic!("Expected a Number on the right side"),
                                    }
                                }
                                _ => panic!("Expected a Arithmetic on the right side"),
                            }
                        }
                        _ => panic!("Expected a Arithmetic"),
                    }
                }
                _ => panic!("Expected an Assignment"),
            }
        }
        _ => panic!("Expected a Program ASTNode"),
    }
}