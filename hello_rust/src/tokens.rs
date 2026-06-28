#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Number(i32),
    Identifier(String),
    Assign,
    Plus,
    Minus,
    EOF,
}

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    Push(i32),
    Load(usize),
    Store(usize),
    Add,
    Subtract,
    Pop,
}