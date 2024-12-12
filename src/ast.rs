use crate::symbol_table::Type;

#[derive(Debug)]
pub enum ASTNode {
    Program(Vec<ASTNode>),
    ClassDeclaration { name: String, members: Vec<ASTNode> },
    FunctionDeclaration { name: String, params: Vec<String>, body: Vec<ASTNode> },
    VariableDeclaration { name: String, var_type: Type },
    Assignment { variable: String, expression: Box<ASTNode> },
    Number(i64),
    Identifier(String),
    Loop { body: Vec<ASTNode> },
    ForLoop { variable: String, start: Box<ASTNode>, end: Box<ASTNode>, body: Vec<ASTNode> },
    WhileLoop { condition: Box<ASTNode>, body: Vec<ASTNode> },
    Comparison { left: Box<ASTNode>, operator: String, right: Box<ASTNode> },
    Arithmetic { left: Box<ASTNode>, operator: String, right: Box<ASTNode> },
}
