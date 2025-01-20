#[derive(Debug)]
pub enum ASTNode {
    Program(Vec<ASTNode>),
    ClassDeclaration { name: String, members: Vec<ASTNode> },
    FunctionDeclaration { name: String, params: Vec<String>, body: Vec<ASTNode> },
    VariableDeclaration { name: String },
    Assignment { variable: String, expression: Box<ASTNode> },
    Number(i64),
    Identifier(String),
    BinaryOperation {
        operator: char,
        left: Box<ASTNode>,
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
}
