use crate::symbol_table::Type;

#[derive(Debug, Clone)]
pub enum ASTNode {
    Program(Vec<ASTNode>),
    ClassDeclaration { name: String, members: Vec<ASTNode> },
    FunctionDeclaration { name: String, params: Vec<String>, body: Vec<ASTNode> },
    VariableDeclaration { name: String, var_type: Type },
    Assignment { variable: String, expression: Box<ASTNode> },
    Number(i64),
    Float(f64),
    StringLiteral(String),
    Boolean(bool),
    Identifier(String),
    FunctionCall {
        name: String,
        args: Vec<ASTNode>,
    },
    Arithmetic {
        left: Box<ASTNode>,
        operator: String,
        right: Box<ASTNode>,
    },
    Comparison {
        left: Box<ASTNode>,
        operator: String,
        right: Box<ASTNode>,
    },
    IfElse {
        condition: Box<ASTNode>,
        then_block: Vec<ASTNode>,
        else_block: Option<Vec<ASTNode>>,
    },
    Ret {
        expression: Box<ASTNode>,
    },
    Loop { body: Vec<ASTNode> },
    ForLoop {
        variable: String,
        start: Box<ASTNode>,
        end: Box<ASTNode>,
        body: Vec<ASTNode>,
    },
    WhileLoop {
        condition: Box<ASTNode>,
        body: Vec<ASTNode>,
    },
}

