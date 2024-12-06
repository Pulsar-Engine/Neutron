use crate::lexer::{Lexer, Token};
use crate::ast::ASTNode;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(mut lexer: Lexer<'a>) -> Self {
        let current_token = lexer.get_next_token();
        Parser { lexer, current_token }
    }

    fn advance(&mut self) {
        self.current_token = self.lexer.get_next_token();
    }

    fn eat(&mut self, token: Token) {
        if self.current_token == token {
            self.advance();
        } else {
            panic!("Unexpected token: {:?}, expected {:?}", self.current_token, token);
        }
    }

    pub fn parse_program(&mut self) -> ASTNode {
        let mut nodes = Vec::new();
        while self.current_token != Token::EOF {
            nodes.push(self.parse_statement());
        }
        ASTNode::Program(nodes)
    }

    fn parse_statement(&mut self) -> ASTNode {
        match self.current_token {
            Token::Class => self.parse_class_declaration(),
            Token::Func => self.parse_function_declaration(),
            Token::Var => self.parse_variable_declaration(),
            Token::Identifier(_) => self.parse_assignment(),
            _ => panic!("Unexpected token: {:?}", self.current_token),
        }
    }

    fn parse_class_declaration(&mut self) -> ASTNode {
        self.eat(Token::Class);
        let name = if let Token::Identifier(name) = self.current_token.clone() {
            name
        } else {
            panic!("Expected class name");
        };
        self.eat(Token::Identifier(name.clone()));
        let mut members = Vec::new();
        self.eat(Token::Then);
        while self.current_token != Token::End {
            members.push(self.parse_statement());
        }
        self.eat(Token::End);
        ASTNode::ClassDeclaration { name, members }
    }

    fn parse_function_declaration(&mut self) -> ASTNode {
        self.eat(Token::Func);
        let name = if let Token::Identifier(name) = self.current_token.clone() {
            name
        } else {
            panic!("Expected function name");
        };
        self.eat(Token::Identifier(name.clone()));
        let params = Vec::new();
        self.eat(Token::Then);
        let mut body = Vec::new();
        while self.current_token != Token::End {
            body.push(self.parse_statement());
        }
        self.eat(Token::End);
        ASTNode::FunctionDeclaration { name, params, body }
    }

    fn parse_variable_declaration(&mut self) -> ASTNode {
        self.eat(Token::Var);
        let name = if let Token::Identifier(name) = self.current_token.clone() {
            name
        } else {
            panic!("Expected variable name");
        };
        self.eat(Token::Identifier(name.clone()));
        ASTNode::VariableDeclaration { name }
    }

    fn parse_assignment(&mut self) -> ASTNode {
        if let Token::Identifier(name) = self.current_token.clone() {
            self.eat(Token::Identifier(name.clone()));
            self.eat(Token::Assign);
            let expression = self.parse_expression();
            ASTNode::Assignment {
                variable: name,
                expression: Box::new(expression),
            }
        } else {
            panic!("Expected variable for assignment");
        }
    }

    fn parse_expression(&mut self) -> ASTNode {
        match self.current_token.clone() {
            Token::Number(value) => {
                self.eat(Token::Number(value));
                ASTNode::Number(value)
            }
            Token::Identifier(name) => {
                self.eat(Token::Identifier(name.clone()));
                ASTNode::Identifier(name)
            }
            _ => panic!("Unexpected token in expression"),
        }
    }
}
