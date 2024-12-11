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
}
