

use crate::ast::ASTNode;
use crate::lexer::Lexer;
use crate::tokens::Token;

// Inside src/parser.rs
pub struct Parser {
    lexer: Lexer,
    current_token: Token,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let current_token = lexer.next_token();
        Self { lexer, current_token }
    }

    fn advance(&mut self) {
        self.current_token = self.lexer.next_token();
    }

    /// 🌟 The new entry point: Parses lines until EOF
    pub fn parser(&mut self) -> ASTNode {
        let mut statements = Vec::new();

        while self.current_token != Token::EOF {
            statements.push(self.parse_statement());
        }

        ASTNode::Program(statements)
    }

    /// Your original parsing logic, now isolated for a single statement
    fn parse_statement(&mut self) -> ASTNode {
        if let Token::Identifier(name) = &self.current_token {
            if self.lexer.peek_char() == Some(&'=') {
                let var_name = name.clone();
                self.advance(); 
                self.advance(); 
                let expr = self.parse_statement(); 
                return ASTNode::Assignment {
                    name: var_name,
                    value: Box::new(expr),
                };
            }
        }

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