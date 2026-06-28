#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Number(i32),
    Identifier(String),
    Assign,
    Plus,
    Minus,
    Star,
    Slash,
    Generate,
    Field,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    EOF,
}

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    Push(i32),
    Load(usize),
    Store(usize),
    Add,
    Subtract,
    Multiply,
    Divide,
    Pop,
    Print,
}