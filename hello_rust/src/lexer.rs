use crate::tokens::Token;

pub struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
        }
    }

    pub fn next_token(&mut self) -> Token {
        while self.position < self.input.len() && self.input[self.position].is_whitespace() {
            self.position += 1;
        }
        if self.position >= self.input.len() {
            return Token::EOF;
        }
        let current = self.input[self.position];

        match current {
            '+' => {
                self.position += 1;
                Token::Plus
            }
            '-' => {
                self.position += 1;
                Token::Minus
            }
            '*' => {
                self.position += 1;
                Token::Star
            }
            '/' => {
                self.position += 1;
                Token::Slash
            }
            '=' => {
                self.position += 1;
                Token::Assign
            }
            '(' => {
                self.position += 1;
                Token::LeftParen
            }
            ')' => {
                self.position += 1;
                Token::RightParen
            }
            '{' => {
                self.position += 1;
                Token::LeftBrace
            }
            '}' => {
                self.position += 1;
                Token::RightBrace
            }
            '0'..='9' => {
                let start = self.position;
                while self.position < self.input.len() && self.input[self.position].is_numeric() {
                    self.position += 1;
                }
                let text: String = self.input[start..self.position].iter().collect();
                Token::Number(text.parse().unwrap())
            }
            'a'..='z' | 'A'..'Z' => {
                let start = self.position;

                while self.position < self.input.len() && self.input[self.position].is_alphabetic()
                {
                    self.position += 1;
                }
                let word: String = self.input[start..self.position].iter().collect();

                match word.as_str() {
                    "generate" => Token::Generate,
                    "field" => Token::Field,
                    _ => Token::Identifier(word),
                }
            }
            _ => panic!("Unknown character {}", current),
        }
    }
    pub fn peek_char(&self) -> Option<&char> {
        let mut look_ahead = self.position;

        while look_ahead < self.input.len() && self.input[look_ahead].is_whitespace() {
            look_ahead += 1;
        }
        self.input.get(look_ahead)
    }
}
