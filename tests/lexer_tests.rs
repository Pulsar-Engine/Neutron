<<<<<<< HEAD
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
=======
#[cfg(test)]
mod tests {
    use neutron::lexer::{Lexer, Token};

    fn lex(input: &str) -> Vec<Token> {
        let mut lexer = Lexer::new(input);
        let mut tokens = Vec::new();
        loop {
            let token = lexer.get_next_token();
            if token == Token::EOF {
                break;
            }
            tokens.push(token);
        }
        tokens
    }

    #[test]
    fn test_class_token() {
        let input = "class";
        let tokens = lex(input);
        assert_eq!(tokens, vec![Token::Class]);
    }

    #[test]
    fn test_identifier_token() {
        let input = "my_var";
        let tokens = lex(input);
        assert_eq!(tokens, vec![Token::Identifier("my_var".to_string())]);
    }

    #[test]
    fn test_number_token() {
        let input = "123";
        let tokens = lex(input);
        assert_eq!(tokens, vec![Token::Number(123)]);
    }

    #[test]
    fn test_float_token() {
        let input = "3.14";
        let tokens = lex(input);
        assert_eq!(tokens, vec![Token::Float(3.14)]);
    }

    #[test]
    fn test_multiple_tokens() {
        let input = "class MyClass var x = 10";
        let tokens = lex(input);
        assert_eq!(
            tokens,
            vec![
                Token::Class,
                Token::Identifier("MyClass".to_string()),
                Token::Var,
                Token::Identifier("x".to_string()),
                Token::Assign,
                Token::Number(10)
            ]
        );
    }

    #[test]
    fn test_skip_whitespace() {
        let input = "class    my_var 123";
        let tokens = lex(input);
        assert_eq!(
            tokens,
            vec![
                Token::Class,
                Token::Identifier("my_var".to_string()),
                Token::Number(123)
            ]
        );
    }

    #[test]
    fn test_negative_integer() {
        let input = "-42";
        let tokens = lex(input);
        assert_eq!(tokens, vec![Token::Number(-42)]);
    }

    #[test]
    fn test_negative_float() {
        let input = "-3.14";
        let tokens = lex(input);
        assert_eq!(tokens, vec![Token::Float(-3.14)]);
    }

    #[test]
    fn test_subtraction_vs_negative_number() {
        let input = "10 - 5 -3";
        let tokens = lex(input);
        assert_eq!(
            tokens,
            vec![
                Token::Number(10),
                Token::Minus,
                Token::Number(5),
                Token::Number(-3)
            ]
        );
    }

    #[test]
    fn test_mixed_negative_and_subtraction() {
        let input = "-5 - -3";
        let tokens = lex(input);
        assert_eq!(
            tokens,
            vec![
                Token::Number(-5),
                Token::Minus,
                Token::Number(-3)
            ]
        );
    }
>>>>>>> neutron/parsing
}
