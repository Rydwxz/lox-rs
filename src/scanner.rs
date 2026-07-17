use crate::tokentype::*;
use core::fmt;
use std::{collections::HashMap, fmt::Display};

#[derive(Debug)]
pub struct ScanError {
    line: usize,
    msg: String,
}

impl Display for ScanError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Scan error on line {}: {}", self.line, self.msg)
    }
}

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    keywords: HashMap<&'static str, TokenType>,
    errors: Vec<ScanError>,
}
impl Scanner {
    pub fn new(source: String) -> Self {
        let mut map = HashMap::new();
        map.insert("and", TokenType::And);
        map.insert("class", TokenType::Class);
        map.insert("else", TokenType::Else);
        map.insert("false", TokenType::FalseT);
        map.insert("for", TokenType::For);
        map.insert("fun", TokenType::Fun);
        map.insert("if", TokenType::If);
        map.insert("nil", TokenType::Nil);
        map.insert("or", TokenType::Or);
        map.insert("print", TokenType::Print);
        map.insert("return", TokenType::Return);
        map.insert("super", TokenType::Super);
        map.insert("this", TokenType::This);
        map.insert("true", TokenType::TrueT);
        map.insert("var", TokenType::Var);
        map.insert("while", TokenType::While);
        Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 0,
            keywords: map,
            errors: Vec::new(),
        }
    }

    pub fn scan_tokens(mut self) -> Result<Vec<Token>, Vec<ScanError>> {
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

        if self.errors.len() == 0 {
            Ok(self.tokens)
        } else {
            Err(self.errors)
        }
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
            '"' => self.string(),
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            c if is_digit(c) => self.number(),
            c if is_alpha(c) => self.identifier(),
            _ => self.errors.push(ScanError {
                msg: "unexpected character".to_string(),
                line: self.line,
            }),
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source[self.current - 1..self.current]
            .chars()
            .next()
            .unwrap()
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

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.current..].chars().next().unwrap()
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source.chars().nth(self.current + 1).unwrap()
        }
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_at_end() {
            self.errors.push(ScanError {
                line: self.line,
                msg: format!("unterminated string"),
            });
            return;
        }
        self.advance();
        self.add_new_token(
            TokenType::StringT,
            Some(self.source[self.start + 1..self.current - 1].to_string()),
        );
    }

    fn number(&mut self) {
        while is_digit(self.peek()) {
            self.advance();
        }
        if self.peek() == '.' && is_digit(self.peek_next()) {
            self.advance();
            while is_digit(self.peek()) {
                self.advance();
            }
        }
        self.add_new_token(
            TokenType::Number,
            Some(self.source[self.start..self.current].to_string()),
        );
    }

    fn identifier(&mut self) {
        while is_alphanumeric(self.peek()) {
            self.advance();
        }
        let text = &self.source[self.start..self.current];
        if let Some(tokentype) = self.keywords.get(text) {
            let t = tokentype.clone();
            self.add_token(t);
        } else {
            self.add_token(TokenType::Identifier);
        }
    }
}

fn is_digit(c: char) -> bool {
    c >= '0' && c <= '9'
}

fn is_alpha(c: char) -> bool {
    (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
}

fn is_alphanumeric(c: char) -> bool {
    is_alpha(c) || is_digit(c)
}
