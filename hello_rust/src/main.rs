use std::panic;

#[derive(Debug, PartialEq, Clone)]
enum Token {
    Number(i32),
    Identifier(String), // Track variable names like "x"
    Assign,             // The '=' operator
    Plus,
    Minus,
    EOF,
}

struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
        }
    }
    
    fn next_token(&mut self) -> Token {
        while self.position < self.input.len() && self.input[self.position].is_whitespace() {
            self.position += 1;
        }
        if self.position >= self.input.len() {
            return Token::EOF;
        }
        let current = self.input[self.position];

        match current {
            '+' => { self.position += 1; Token::Plus }
            '-' => { self.position += 1; Token::Minus }
            '=' => { self.position += 1; Token::Assign }
            '0'..='9' => {
                let start = self.position;
                while self.position < self.input.len() && self.input[self.position].is_numeric() {
                    self.position += 1;
                }
                let text: String = self.input[start..self.position].iter().collect();
                Token::Number(text.parse().unwrap())
            }
            // Recognize identifiers (single lowercase letters for simplicity, e.g., a-z)
            'a'..='z' => {
                self.position += 1;
                Token::Identifier(current.to_string())
            }
            _ => panic!("Unknown character {}", current),
        }
    }
}

#[derive(Debug)]
enum ASTNode {
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
}

struct Parser {
    lexer: Lexer,
    current_token: Token,
}

impl Parser {
    fn new(mut lexer: Lexer) -> Self {
        let current_token = lexer.next_token();
        Self { lexer, current_token }
    }

    fn advance(&mut self) {
        self.current_token = self.lexer.next_token();
    }

    fn parser(&mut self) -> ASTNode {
        // If an identifier is followed by an '=', handle it as an assignment
        if let Token::Identifier(name) = &self.current_token {
            let peek_lexer = self.lexer.input.get(self.lexer.position);
            // Quick lookahead to check for '='
            if peek_lexer == Some(&'=') {
                let var_name = name.clone();
                self.advance(); // consume identifier
                self.advance(); // consume '='
                let expr = self.parser(); // parse the assigned expression
                return ASTNode::Assignment {
                    name: var_name,
                    value: Box::new(expr),
                };
            }
        }

        // Otherwise, fall back to standard expressions
        let mut left = match &self.current_token {
            Token::Number(val) => {
                let value = *val;
                self.advance();
                ASTNode::Number(value)
            }
            Token::Identifier(name) => {
                let var_name = name.clone();
                self.advance();
                ASTNode::Variable(var_name)
            }
            _ => panic!("Expected a number or identifier, got {:?}", self.current_token),
        };

        while self.current_token == Token::Plus || self.current_token == Token::Minus {
            let op = self.current_token.clone();
            self.advance();
            let right = match &self.current_token {
                Token::Number(val) => {
                    let value = *val;
                    self.advance();
                    ASTNode::Number(value)
                }
                Token::Identifier(name) => {
                    let var_name = name.clone();
                    self.advance();
                    ASTNode::Variable(var_name)
                }
                _ => panic!("Expected a number or identifier after operator"),
            };
            left = ASTNode::BinaryOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }
        left
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Push(i32),
    Load(usize),
    Store(usize),
    Add,
    Subtract,
}

struct Compiler {
    bytecode: Vec<Instruction>,
}

impl Compiler {
    fn new() -> Self {
        Self { bytecode: Vec::new() }
    }

    // Helper to map 'a'-'z' to variable vector indices 0-25
    fn get_var_index(&self, name: &str) -> usize {
        let ch = name.chars().next().unwrap();
        (ch as usize) - ('a' as usize)
    }

    fn compile(&mut self, node: &ASTNode) {
        match node {
            ASTNode::Number(val) => {
                self.bytecode.push(Instruction::Push(*val));
            }
            ASTNode::Variable(name) => {
                let idx = self.get_var_index(name);
                self.bytecode.push(Instruction::Load(idx));
            }
            ASTNode::Assignment { name, value } => {
                self.compile(value); // Compile value expression first to put result on top of the stack
                let idx = self.get_var_index(name);
                self.bytecode.push(Instruction::Store(idx));
            }
            ASTNode::BinaryOp { left, op, right } => {
                self.compile(left);
                self.compile(right);
                match op {
                    Token::Plus => self.bytecode.push(Instruction::Add),
                    Token::Minus => self.bytecode.push(Instruction::Subtract),
                    _ => unreachable!(),
                }
            }
        }
    }
}

struct VM {
    stack: Vec<i32>,
    variables: Vec<i32>,
}

impl VM {
    fn new() -> Self {
        Self { stack: Vec::new(), variables: vec![0; 26] }
    }
    
    fn run(&mut self, instructions: &[Instruction]) -> i32 {
        for instruction in instructions {
            match instruction {
                Instruction::Push(val) => {
                    self.stack.push(*val);
                }
                Instruction::Add => {
                    let right = self.stack.pop().unwrap();
                    let left = self.stack.pop().unwrap();
                    self.stack.push(left + right);
                }
                Instruction::Subtract => {
                    let right = self.stack.pop().unwrap();
                    let left = self.stack.pop().unwrap();
                    self.stack.push(left - right);
                }
                Instruction::Store(idx) => {
                    // Peek at the top of stack or pop it to store into variable register
                    self.variables[*idx] = *self.stack.last().unwrap_or(&0);
                }
                Instruction::Load(idx) => {
                    self.stack.push(self.variables[*idx]);
                }
            }
        }
        self.stack.pop().unwrap_or(0)
    }
}

fn main() {
    // Let's test allocating a variable and running math operations on it!
    let mut vm = VM::new();

    // 1. Compile and execute an assignment: "x = 15"
    run_pipeline("x = 15", &mut vm);

    // 2. Compile and execute an expression reading that variable: "x + 5 - 2"
    run_pipeline("x + 5 - 2", &mut vm);
}

fn run_pipeline(source_code: &str, vm: &mut VM) {
    println!("--- Processing: \"{}\" ---", source_code);

    let lexer = Lexer::new(source_code);
    let mut parser = Parser::new(lexer);
    let ast = parser.parser();
    
    let mut compiler = Compiler::new();
    compiler.compile(&ast);
    
    for (ip, instr) in compiler.bytecode.iter().enumerate() {
        println!("{:04} : {:?}", ip, instr);
    }

    let result = vm.run(&compiler.bytecode);
    println!("VM Stack Yield / Result: {}\n", result);
}