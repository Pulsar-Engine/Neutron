#[derive(Debug, PartialEq, Clone)]

pub enum Token {
    Class,
    Func,
    Var,
    Identifier(String),
    Number(i64),
    Float(f64),
    Type(String),
    If,
    Else,
    Assign,
    Equal,
    Plus,
    Minus,
    Multiply,
    Divide,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Comma,
    Then,
    End,
    Ret,
    Loop,
    For,
    While,
    Boolean(bool),
    LessThan,
    GreaterThan,
    EOF,
}

pub struct Lexer<'a> {
    source: std::str::Chars<'a>,
    current_char: Option<char>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut lexer = Lexer {
            source: source.chars(),
            current_char: None,
        };
        lexer.advance();
        lexer
    }

    fn advance(&mut self) {
        self.current_char = self.source.next();
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn read_identifier(&mut self) -> String {
        let mut identifier = String::new();
        while let Some(c) = self.current_char {
            if c.is_alphanumeric() || c == '_' {
                identifier.push(c);
                self.advance();
            } else {
                break;
            }
        }
        identifier
    }

    pub fn get_next_token(&mut self) -> Token {
        self.skip_whitespace();
        match self.current_char {
            Some(c) if c.is_alphabetic() => {
                let identifier = self.read_identifier();
                match identifier.as_str() {
                    "class" => Token::Class,
                    "func" => Token::Func,
                    "var" => Token::Var,
                    "then" => Token::Then,
                    "end" => Token::End,
                    "ret" => Token::Ret,
                    "for" => Token::For,
                    "while" => Token::While,
                    "loop" => Token::Loop,
                    "true" => Token::Boolean(true),
                    "false" => Token::Boolean(false),
                    "if" => Token::If,
                    "else" => Token::Else,
                    "int" | "float" | "string" | "bool" => Token::Type(identifier),
                    _ => Token::Identifier(identifier),
                }
            }
            Some('-') => {
                self.advance();
                match self.current_char {
                    Some(c) if c.is_digit(10) => {
                        let mut number = String::from("-");
                        while let Some(d) = self.current_char.and_then(|c| c.to_digit(10)) {
                            number.push(std::char::from_digit(d, 10).unwrap());
                            self.advance();
                        }
                        if self.current_char == Some('.') {
                            number.push('.');
                            self.advance();
                            while let Some(d) = self.current_char.and_then(|c| c.to_digit(10)) {
                                number.push(std::char::from_digit(d, 10).unwrap());
                                self.advance();
                            }
                            let value = number.parse::<f64>().unwrap();
                            return Token::Float(value);
                        } else {
                            let value = number.parse::<i64>().unwrap();
                            return Token::Number(value);
                        }
                    }
                    _ => Token::Minus,
                }
            }
            Some(c) if c.is_digit(10) => {
                let mut number = String::new();            
                while let Some(d) = self.current_char.and_then(|c| c.to_digit(10)) {
                    number.push(std::char::from_digit(d, 10).unwrap());
                    self.advance();
                }            
                if self.current_char == Some('.') {
                    number.push('.');
                    self.advance();            
                    while let Some(d) = self.current_char.and_then(|c| c.to_digit(10)) {
                        number.push(std::char::from_digit(d, 10).unwrap());
                        self.advance();
                    }
                    let value = number.parse::<f64>().unwrap();
                    Token::Float(value)
                } else {
                    let value = number.parse::<i64>().unwrap();
                    Token::Number(value)
                }
            }            
            Some('=') => {
                self.advance();
                if self.current_char == Some('=') {
                    self.advance();
                    Token::Equal
                } else {
                    Token::Assign
                }
            }
            Some('+') => {
                self.advance();
                Token::Plus
            }
            Some('*') => {
                self.advance();
                Token::Multiply
            }
            Some('/') => {
                self.advance();
                Token::Divide
            }
            Some('(') => {
                self.advance();
                Token::LParen
            }
            Some(')') => {
                self.advance();
                Token::RParen
            }
            Some('{') => {
                self.advance();
                Token::LBrace
            }
            Some('}') => {
                self.advance();
                Token::RBrace
            }
            Some(',') => {
                self.advance();
                Token::Comma
            }
            Some('<') => {
                self.advance();
                Token::LessThan
            }
            Some('>') => {
                self.advance();
                Token::GreaterThan
            }
            None => Token::EOF,
            _ => panic!("Unrecognized character: {:?}", self.current_char),
        }
    }
}
