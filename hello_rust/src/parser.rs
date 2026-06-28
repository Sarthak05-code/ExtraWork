// Inside src/parser.rs

use crate::ast::ASTNode;
use crate::lexer::Lexer;
use crate::tokens::Token;

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let current_token = lexer.next_token();
        Self {
            lexer,
            current_token,
        }
    }

    fn advance(&mut self) {
        self.current_token = self.lexer.next_token();
    }

    pub fn parser(&mut self) -> ASTNode {
        // 1. Enforce 'field' keyword
        if self.current_token != Token::Field {
            panic!("Syntax Error: Programs must start with 'field' keyword");
        }
        self.advance();

        // 2. Enforce function name is exactly 'main'
        if let Token::Identifier(name) = &self.current_token {
            if name != "main" {
                panic!("Syntax Error: Expected entry name 'main', found {}", name);
            }
        } else {
            panic!("Syntax Error: Expected function name 'main'");
        }
        self.advance();

        // 3. Enforce matching parameter parentheses ()
        if self.current_token != Token::LeftParen {
            panic!("Syntax Error: Expected '(' after main");
        }
        self.advance();
        if self.current_token != Token::RightParen {
            panic!("Syntax Error: Expected ')' after main(");
        }
        self.advance();

        // 4. Enforce opening brace {
        if self.current_token != Token::LeftBrace {
            panic!("Syntax Error: Expected '{{' to open main block");
        }
        self.advance();

        // 5. Read all the statements inside the function body
        let mut statements = Vec::new();
        while self.current_token != Token::RightBrace && self.current_token != Token::EOF {
            statements.push(self.parse_statement());
        }

        // 6. Enforce closing brace }
        if self.current_token != Token::RightBrace {
            panic!("Syntax Error: Missing closing '}}' for field main()");
        }
        self.advance(); // Consume the '}'

        // 7. Return the final program node (No semicolon!)
        ASTNode::Program(statements)
    }

    /// 1. LOW PRECEDENCE LAYER: Addition, Subtraction, and Variable Assignment
    fn parse_statement(&mut self) -> ASTNode {
        if self.current_token == Token::Generate {
            self.advance();
            let expr = self.parse_statement();
            return ASTNode::Generate(Box::new(expr));
        }
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

        let mut left = self.parse_term();

        while self.current_token == Token::Plus || self.current_token == Token::Minus {
            let op = self.current_token.clone();
            self.advance();

            let right = self.parse_term();
            left = ASTNode::BinaryOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }
        left
    }

    /// 2. HIGH PRECEDENCE LAYER: Multiplication and Division
    fn parse_term(&mut self) -> ASTNode {
        let mut left = self.parse_primary();

        while self.current_token == Token::Star || self.current_token == Token::Slash {
            let op = self.current_token.clone();
            self.advance();

            let right = self.parse_primary();
            left = ASTNode::BinaryOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }
        left
    }

    /// 3. GROUND LAYER: Leaf values (Numbers and Variables)
    fn parse_primary(&mut self) -> ASTNode {
        match &self.current_token {
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
            _ => panic!(
                "Expected a number or identifier, got {:?}",
                self.current_token
            ),
        }
    }
}
