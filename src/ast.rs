#[derive(Debug)]
pub enum ASTNode {
    Program(Vec<ASTNode>),
    ClassDeclaration { name: String, members: Vec<ASTNode> },
    FunctionDeclaration { name: String, params: Vec<String>, body: Vec<ASTNode> },
    VariableDeclaration { name: String },
    Assignment { variable: String, expression: Box<ASTNode> },
    Number(i64),
    Identifier(String),
}
