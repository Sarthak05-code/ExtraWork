// Inside src/ast.rs
use crate::tokens::Token;

#[derive(Debug ,Clone)]
pub enum ASTNode {
    Program(Vec<ASTNode>), // 🌟 Added to support multiple lines!
    Number(i32),
    Variable(String),
    Assignment {
        name: String,
        value: Box<ASTNode>,
    },
    BinaryOp {
        left: Box<ASTNode>,
        op: Token,
        right: Box<ASTNode>,
    },
    Generate(Box<ASTNode>),
}