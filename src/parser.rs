use crate::lexer::{Lexer, Token};
use crate::ast::ASTNode;
use crate::symbol_table::{SymbolTable, Type};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
    symbol_table: SymbolTable,
}

impl<'a> Parser<'a> {
    pub fn new(mut lexer: Lexer<'a>) -> Self {
        let current_token = lexer.get_next_token();
        Parser { lexer, current_token, symbol_table: SymbolTable::new(), }
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
        println!("Parsing statement: {:?}", self.current_token);
        match self.current_token {
            Token::Class => self.parse_class_declaration(),
            Token::Func => self.parse_function_declaration(),
            Token::Var => self.parse_variable_declaration(),
            Token::While => self.parse_while_loop(),
            Token::For => self.parse_for_loop(),
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

        let var_type = if let Token::Type(type_name) = self.current_token.clone() {
            self.eat(Token::Type(type_name.clone()));
            match type_name.as_str() {
                "int" => Type::Int,
                "float" => Type::Float,
                "string" => Type::String,
                "bool" => Type::Bool,
                _ => panic!("Unknown type: {}", type_name),
            }
        } else {
            panic!("Expected variable type after variable name");
        };
        self.symbol_table.declare_variable(&name, var_type.clone());
        ASTNode::VariableDeclaration { name, var_type }
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
        let left = match self.current_token.clone() {
            Token::Number(value) => {
                self.eat(Token::Number(value));
                ASTNode::Number(value)
            }
            Token::Identifier(name) => {
                self.eat(Token::Identifier(name.clone()));
                ASTNode::Identifier(name)
            }
            _ => panic!("Unexpected token in expression: {:?}", self.current_token),
        };
    
        match self.current_token {
            Token::LessThan => {
                self.eat(Token::LessThan);
                let right = self.parse_expression();
                ASTNode::Comparison {
                    left: Box::new(left),
                    operator: "<".to_string(),
                    right: Box::new(right),
                }
            }
            Token::GreaterThan => {
                self.eat(Token::GreaterThan);
                let right = self.parse_expression();
                ASTNode::Comparison {
                    left: Box::new(left),
                    operator: ">".to_string(),
                    right: Box::new(right),
                }
            }
            Token::Plus => {
                self.eat(Token::Plus);
                let right = self.parse_expression();
                ASTNode::Arithmetic {
                    left: Box::new(left),
                    operator: "+".to_string(),
                    right: Box::new(right),
                }
            }
            Token::Minus => {
                self.eat(Token::Minus);
                let right = self.parse_expression();
                ASTNode::Arithmetic {
                    left: Box::new(left),
                    operator: "-".to_string(),
                    right: Box::new(right),
                }
            }
            _ => left,
        }
    }
    

    fn parse_while_loop(&mut self) -> ASTNode {
        self.eat(Token::While);
        let condition = self.parse_expression();
        let mut body = Vec::new();
        self.symbol_table.enter_scope();
        while self.current_token != Token::End {
            body.push(self.parse_statement());
        }
        self.eat(Token::End);
        self.symbol_table.exit_scope();
        ASTNode::WhileLoop { condition: (Box::new(condition)), body, }
    }

    fn parse_for_loop(&mut self) -> ASTNode {
        self.eat(Token::For);
        self.symbol_table.enter_scope();
        let variable = if let Token::Identifier(name) = self.current_token.clone() {
            self.eat(Token::Identifier(name.clone()));
            if !self.symbol_table.variables.contains_key(&name) {
                self.symbol_table.declare_variable(&name, Type::Int);
            }
            name
        } else {
            panic!("Expected variable name after 'for'");
        };
        if self.current_token != Token::Assign {
            panic!("Expected '=', found {:?}", self.current_token);
        }
        
        self.eat(Token::Assign);
        let start = self.parse_expression();
        let end = self.parse_expression();
        let mut body = Vec::new();
        while self.current_token != Token::End {
            body.push(self.parse_statement());
        }
        self.eat(Token::End);
        self.symbol_table.exit_scope();
        ASTNode::ForLoop { variable, start: (Box::new(start)), end: (Box::new(end)), body, }
    }
}
