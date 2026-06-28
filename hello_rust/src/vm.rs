use crate::tokens::Instruction;

pub struct VM {
    stack: Vec<i32>,
    variables: Vec<i32>,
}

impl VM {
    pub fn new() -> Self {
        Self { stack: Vec::new(), variables: vec![0; 26] }
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
                Instruction::Store(idx) => {
                    self.variables[*idx] = self.stack.pop().unwrap_or(0);
                }
                Instruction::Load(idx) => {
                    self.stack.push(self.variables[*idx]);
                }
                Instruction::Pop => {
                    self.stack.pop();
                }
            }
        }
        self.stack.pop().unwrap_or(0)
    }
}