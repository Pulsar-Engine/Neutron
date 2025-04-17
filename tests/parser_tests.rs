<<<<<<< HEAD
use neutron::lexer::{Lexer};
use neutron::parser::Parser;
use neutron::ast::{ASTNode};

#[test]
fn test_parse_simple_class() {
    let source_code = "
        class MyClass then
            var x
            func myFunc then
                x = 42
            end
        end
    ";
    
    let mut parser = Parser::new(Lexer::new(source_code));
=======
use neutron::lexer::{Lexer, Token};
use neutron::parser::Parser;
use neutron::ast::ASTNode;

#[test]
fn test_parse_program() {
    let input = "class MyClass then var x end";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
>>>>>>> neutron/parsing
    let ast = parser.parse_program();

    match ast {
        ASTNode::Program(nodes) => {
<<<<<<< HEAD
            // Recherche dans l'AST une déclaration de classe "MyClass"
            assert!(nodes.iter().any(|node| matches!(node, ASTNode::ClassDeclaration { name, .. } if name == "MyClass")));

            // Vérifie que la variable "x" et la fonction "myFunc" sont présentes dans la déclaration de classe
            for node in nodes {
                if let ASTNode::ClassDeclaration { members, .. } = node {
                    assert!(members.iter().any(|member| matches!(member, ASTNode::VariableDeclaration { name } if name == "x")));
                    assert!(members.iter().any(|member| matches!(member, ASTNode::FunctionDeclaration { name, .. } if name == "myFunc")));
                }
            }
        }
        _ => panic!("Le programme n'est pas au format attendu."),
=======
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
    let input = "class MyClass then var x end";
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
                        ASTNode::VariableDeclaration { name } => {
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
    let input = "func myFunc(a, b) then var x end";
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
                        ASTNode::VariableDeclaration { name } => {
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
    let input = "var x";
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    let ast = parser.parse_program();

    match ast {
        ASTNode::Program(nodes) => {
            assert_eq!(nodes.len(), 1);
            match &nodes[0] {
                ASTNode::VariableDeclaration { name } => {
                    assert_eq!(name, "x");
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
                        ASTNode::BinaryOperation { operator, ref left, ref right } => {
                            assert_eq!(operator, '+');
                            match **left {
                                ASTNode::Number(value) => assert_eq!(value, 3),
                                _ => panic!("Expected a Number on the left side"),
                            }
                            match **right {
                                ASTNode::BinaryOperation { operator, ref left, ref right } => {
                                    assert_eq!(operator, '*');
                                    match **left {
                                        ASTNode::Number(value) => assert_eq!(value, 4),
                                        _ => panic!("Expected a Number on the left side"),
                                    }
                                    match **right {
                                        ASTNode::Number(value) => assert_eq!(value, 2),
                                        _ => panic!("Expected a Number on the right side"),
                                    }
                                }
                                _ => panic!("Expected a BinaryOperation on the right side"),
                            }
                        }
                        _ => panic!("Expected a BinaryOperation"),
                    }
                }
                _ => panic!("Expected an Assignment"),
            }
        }
        _ => panic!("Expected a Program ASTNode"),
>>>>>>> neutron/parsing
    }
}
