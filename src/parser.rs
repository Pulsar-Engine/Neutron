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

    fn consume_token(&mut self, token: Token) {
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
            Token::If => self.parse_if_else(),
            Token::Ret => self.parse_ret(),
            _ => panic!("Unexpected token: {:?}", self.current_token),
        }
    }

    fn parse_class_declaration(&mut self) -> ASTNode {
        self.consume_token(Token::Class);
        let name = if let Token::Identifier(name) = self.current_token.clone() {
            name
        } else {
            panic!("Expected class name");
        };
        self.consume_token(Token::Identifier(name.clone()));
        let mut members = Vec::new();
        self.consume_token(Token::Then);
        while self.current_token != Token::End {
            members.push(self.parse_statement());
        }
        self.consume_token(Token::End);
        ASTNode::ClassDeclaration { name, members }
    }

    fn parse_function_declaration(&mut self) -> ASTNode {
        self.consume_token(Token::Func);
        let name = if let Token::Identifier(name) = self.current_token.clone() {
            name
        } else {
            panic!("Expected function name");
        };
        self.consume_token(Token::Identifier(name.clone()));
        self.consume_token(Token::LParen);
        let mut params = Vec::new();
        while self.current_token != Token::RParen {
            if let Token::Identifier(param_name) = self.current_token.clone() {
                params.push(param_name);
                self.advance();
            }
            if self.current_token == Token::Comma {
                self.advance();
            }
        }
        self.consume_token(Token::RParen);
        self.consume_token(Token::Then);
        let mut body = Vec::new();
        while self.current_token != Token::End {
            body.push(self.parse_statement());
        }
        self.consume_token(Token::End);
        ASTNode::FunctionDeclaration { name, params, body }
    }

    fn parse_variable_declaration(&mut self) -> ASTNode {
        self.consume_token(Token::Var);
        let name = if let Token::Identifier(name) = self.current_token.clone() {
            name
        } else {
            panic!("Expected variable name");
        };
        self.consume_token(Token::Identifier(name.clone()));

        let var_type = if let Token::Type(type_name) = self.current_token.clone() {
            self.consume_token(Token::Type(type_name.clone()));
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
            self.consume_token(Token::Identifier(name.clone()));
            self.consume_token(Token::Assign);
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
        let mut node = self.parse_term();
    
        while matches!(
            self.current_token,
            Token::LessThan | Token::GreaterThan | Token::Equal | Token::Plus | Token::Minus
        ) {
            let op = self.current_token.clone();
            self.advance();
            let right = self.parse_term();
    
            node = match op {
                Token::LessThan => ASTNode::Comparison {
                    left: Box::new(node),
                    operator: "<".to_string(),
                    right: Box::new(right),
                },
                Token::GreaterThan => ASTNode::Comparison {
                    left: Box::new(node),
                    operator: ">".to_string(),
                    right: Box::new(right),
                },
                Token::Equal => ASTNode::Comparison { 
                    left: Box::new(node),
                    operator: "==".to_string(), 
                    right: Box::new(right), 
                },
                Token::Plus => ASTNode::Arithmetic {
                    left: Box::new(node),
                    operator: "+".to_string(),
                    right: Box::new(right),
                },
                Token::Minus => ASTNode::Arithmetic {
                    left: Box::new(node),
                    operator: "-".to_string(),
                    right: Box::new(right),
                },
                _ => unreachable!(),
            };
        }
    
        node
    }    
    
    fn parse_term(&mut self) -> ASTNode {
        let mut node = self.parse_factor();
    
        while matches!(self.current_token, Token::Plus | Token::Minus) {
            let op = self.current_token.clone();
            self.advance();
            let right = self.parse_factor();
    
            node = match op {
                Token::Plus => ASTNode::Arithmetic {
                    left: Box::new(node),
                    operator: "+".to_string(),
                    right: Box::new(right),
                },
                Token::Minus => ASTNode::Arithmetic {
                    left: Box::new(node),
                    operator: "-".to_string(),
                    right: Box::new(right),
                },
                _ => unreachable!(),
            };
        }
    
        node
    }
    
    fn parse_factor(&mut self) -> ASTNode {
        let mut node = self.parse_primary();
    
        while matches!(self.current_token, Token::Multiply | Token::Divide) {
            let op = self.current_token.clone();
            self.advance();
            let right = self.parse_primary();
    
            node = match op {
                Token::Multiply => ASTNode::Arithmetic {
                    left: Box::new(node),
                    operator: "*".to_string(),
                    right: Box::new(right),
                },
                Token::Divide => ASTNode::Arithmetic {
                    left: Box::new(node),
                    operator: "/".to_string(),
                    right: Box::new(right),
                },
                _ => unreachable!(),
            };
        }
    
        node
    }    

    fn parse_primary(&mut self) -> ASTNode {
        match self.current_token.clone() {
            Token::StringLiteral(s) => {
                self.advance();
                ASTNode::StringLiteral(s)
            }
            Token::Number(value) => {
                self.advance();
                ASTNode::Number(value)
            }
            Token::Boolean(value) => {
                self.advance();
                ASTNode::Boolean(value)
            }
            Token::Float(value) => {
                self.advance();
                ASTNode::Float(value)
            }
            Token::Identifier(name) => {
                self.advance();
                if self.current_token == Token::LParen {
                    self.advance();
                    let mut args = Vec::new();
                    while self.current_token != Token::RParen {
                        args.push(self.parse_expression());
                        if self.current_token == Token::Comma {
                            self.advance();
                        }
                    }
                    self.consume_token(Token::RParen);
                    ASTNode::FunctionCall { name, args }
                } else {
                    ASTNode::Identifier(name)
                }
            }
            Token::LParen => {
                self.advance();
                let expr = self.parse_expression();
                self.consume_token(Token::RParen);
                expr
            }
            _ => panic!("Unexpected token in primary: {:?}", self.current_token),
        }
    }
    

    fn parse_if_else(&mut self) -> ASTNode {
        self.consume_token(Token::If);
        let condition = self.parse_expression();
        self.consume_token(Token::Then);
        let mut then_block = Vec::new();
        while self.current_token != Token::Else && self.current_token != Token::End {
            then_block.push(self.parse_statement());
        }
        let mut else_block = None;
        if self.current_token == Token::Else {
            self.consume_token(Token::Else);
            else_block = Some(self.parse_block());
        }
        self.consume_token(Token::End);
        ASTNode::IfElse {
            condition: Box::new(condition),
            then_block,
            else_block,
        }
    }

    fn parse_block(&mut self) -> Vec<ASTNode> {
        let mut block = Vec::new();
        while self.current_token != Token::End && self.current_token != Token::Else {
            block.push(self.parse_statement());
        }
        block
    }

    fn parse_ret(&mut self) -> ASTNode {
        self.consume_token(Token::Ret);
        let expression = self.parse_expression();
        ASTNode::Ret { expression: Box::new(expression) }
    }    

    fn parse_while_loop(&mut self) -> ASTNode {
        self.consume_token(Token::While);
        let condition = self.parse_expression();
        self.consume_token(Token::Then);
        let mut body = Vec::new();
        self.symbol_table.enter_scope();
        while self.current_token != Token::End {
            body.push(self.parse_statement());
        }
        self.consume_token(Token::End);
        self.symbol_table.exit_scope();
        ASTNode::WhileLoop { condition: (Box::new(condition)), body, }
    }

    fn parse_for_loop(&mut self) -> ASTNode {
        self.consume_token(Token::For);
        self.symbol_table.enter_scope();
        let variable = if let Token::Identifier(name) = self.current_token.clone() {
            self.consume_token(Token::Identifier(name.clone()));
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
        
        self.consume_token(Token::Assign);
        let start = self.parse_expression();
        let end = self.parse_expression();
        let mut body = Vec::new();
        while self.current_token != Token::End {
            body.push(self.parse_statement());
        }
        self.consume_token(Token::End);
        self.symbol_table.exit_scope();
        ASTNode::ForLoop { variable, start: (Box::new(start)), end: (Box::new(end)), body, }
    }
}
