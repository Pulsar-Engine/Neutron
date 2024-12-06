use neutron::lexer::{Lexer, Token};

#[test]
fn test_simple_tokens() {
    let source_code = "class MyClass then var x end";
    let mut lexer = Lexer::new(source_code);

    assert_eq!(lexer.get_next_token(), Token::Class);
    assert_eq!(lexer.get_next_token(), Token::Identifier("MyClass".to_string()));
    assert_eq!(lexer.get_next_token(), Token::Then);
    assert_eq!(lexer.get_next_token(), Token::Var);
    assert_eq!(lexer.get_next_token(), Token::Identifier("x".to_string()));
    assert_eq!(lexer.get_next_token(), Token::End);
    assert_eq!(lexer.get_next_token(), Token::EOF);
}
