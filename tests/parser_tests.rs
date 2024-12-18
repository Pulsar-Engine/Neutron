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
    let ast = parser.parse_program();

    match ast {
        ASTNode::Program(nodes) => {
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
    }
}
