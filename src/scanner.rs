use crate::tokentype::*;

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}
impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 0,
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token {
            tokentype: TokenType::EOF,
            lexeme: String::new(),
            literal: None,
            line: self.line,
        });

        &self.tokens
    }

    fn scan_token(&mut self) {
        match self.advance() {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                if self.matches('=') {
                    self.add_token(TokenType::BangEqual);
                } else {
                    self.add_token(TokenType::Bang);
                }
            }
            '=' => {
                if self.matches('=') {
                    self.add_token(TokenType::EqualEqual);
                } else {
                    self.add_token(TokenType::Equal);
                }
            }
            '<' => {
                if self.matches('=') {
                    self.add_token(TokenType::LessEqual);
                } else {
                    self.add_token(TokenType::Less);
                }
            }
            '>' => {
                if self.matches('=') {
                    self.add_token(TokenType::GreaterEqual);
                } else {
                    self.add_token(TokenType::Greater);
                }
            }
            '/' => {
                if self.matches('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            '"' => self.scan_string(),
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            _ => self.add_token(TokenType::Error(TokenError {
                msg: "Unexpected character".to_string(),
                line: self.line,
            })),
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len() - 1
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source[self.current - 1..].chars().next().unwrap()
    }

    fn add_token(&mut self, tokentype: TokenType) {
        self.add_new_token(tokentype, None);
    }

    fn add_new_token(&mut self, tokentype: TokenType, literal: Option<String>) {
        self.tokens.push(Token {
            tokentype,
            lexeme: self.source[self.start..self.current].to_string(),
            literal,
            line: self.line,
        })
    }

    fn matches(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.source.chars().nth(self.current).unwrap() != expected {
            false
        } else {
            self.current += 1;
            true
        }
    }

    fn peek(&mut self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.current - 1..].chars().next().unwrap()
        }
    }

    fn scan_string(&mut self) {
        println!("begin string() cur:{}", self.current);
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_at_end() {
            self.add_token(TokenType::Error(TokenError {
                line: self.line,
                msg: format!("Unterminated string"),
            }));
            return;
        }
        self.advance();
        self.add_new_token(
            TokenType::String,
            Some(self.source[self.start + 1..self.current - 1].to_string()),
        );
    }
}
