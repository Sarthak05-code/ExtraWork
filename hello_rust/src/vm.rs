use crate::tokens::Instruction;

pub struct VM {
    stack: Vec<i32>,
    variables: Vec<i32>,
}

impl VM {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            variables: vec![0; 26],
        }
    }

    pub fn run(&mut self, instructions: &[Instruction]) -> i32 {
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
                Instruction::Multiply => {
                    let right = self.stack.pop().unwrap();
                    let left = self.stack.pop().unwrap();
                    self.stack.push(left * right);
                }
                Instruction::Divide => {
                    let right = self.stack.pop().unwrap();
                    let left = self.stack.pop().unwrap();
                    if right == 0 {
                        panic!("Runtime error : Divison By Zero");
                    }
                    self.stack.push(left / right);
                }
                Instruction::Store(idx) => {
                    let val = self.stack.pop().unwrap_or(0);
                    if *idx >= self.variables.len() {
                        self.variables.resize(*idx + 1, 0);
                    }
                    self.variables[*idx] = val;
                }
                Instruction::Load(idx) => {
                    let val  = self.variables.get(*idx).cloned().unwrap_or(0);
                    self.stack.push(val);
                }
                Instruction::Print => {
                    let value = self
                        .stack
                        .pop()
                        .expect("Runtime error: Nothing to generate");
                    println!("[Generated Value]: {}", value);
                }
                Instruction::Pop => {
                    self.stack.pop();
                }
            }
        }
        self.stack.pop().unwrap_or(0)
    }
}
