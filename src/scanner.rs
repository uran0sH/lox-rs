use std::collections::HashMap;

use crate::{
    error::{self, LoxError},
    token::{Literal, Token, TokenType},
    util,
};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    current: usize,
    start: usize,
    line: usize,
    keywords: HashMap<String, TokenType>,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        let keywords = {
            let mut map = HashMap::new();
            map.insert("and".to_string(), TokenType::And);
            map.insert("class".to_string(), TokenType::Class);
            map.insert("else".to_string(), TokenType::Else);
            map.insert("false".to_string(), TokenType::False);
            map.insert("for".to_string(), TokenType::For);
            map.insert("fun".to_string(), TokenType::Fun);
            map.insert("if".to_string(), TokenType::If);
            map.insert("nil".to_string(), TokenType::Nil);
            map.insert("or".to_string(), TokenType::Or);
            map.insert("print".to_string(), TokenType::Print);
            map.insert("return".to_string(), TokenType::Return);
            map.insert("super".to_string(), TokenType::Super);
            map.insert("this".to_string(), TokenType::This);
            map.insert("true".to_string(), TokenType::True);
            map.insert("var".to_string(), TokenType::Var);
            map.insert("while".to_string(), TokenType::While);
            map
        };
        Self {
            source,
            tokens: Vec::new(),
            current: 0,
            start: 0,
            line: 1,
            keywords,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, LoxError> {
        let mut had_error = false;
        while !self.is_end() {
            self.start = self.current;
            if let Err(_e) = self.scan_token() {
                had_error = true;
            }
        }
        self.tokens
            .push(Token::new(TokenType::Eof, "".to_string(), None, self.line));
        if had_error {
            return Err(LoxError::new(0, "".to_string(), "".to_string()));
        }
        Ok(self.tokens.clone())
    }

    fn scan_token(&mut self) -> Result<(), LoxError> {
        let mut had_error = false;
        let c = self.advance();
        match c {
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
                let ty = if self.is_match('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(ty)
            }
            '=' => {
                let ty = if self.is_match('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_token(ty)
            }
            '<' => {
                let ty = if self.is_match('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(ty)
            }
            '>' => {
                let ty = if self.is_match('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(ty)
            }
            '/' => {
                if self.is_match('/') {
                    // It's a comment
                    while self.peek() != '\n' && !self.is_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash)
                }
            }
            ' ' => {}
            '\r' => {}
            '\t' => {}
            '\n' => {
                self.line += 1;
            }
            '"' => {
                if let Err(_e) = self.string() {
                    had_error = true;
                }
            }
            _ => {
                if c.is_numeric() {
                    self.number()
                } else if c.is_alphabetic() {
                    self.identifier()
                } else {
                    let e = error::LoxError::new(
                        self.line,
                        "".to_string(),
                        "Unexpected character".to_string(),
                    );
                    eprintln!("{}", e.to_string());
                    return Err(e);
                }
            }
        }
        if had_error {
            return Err(LoxError::new(0, "".to_string(), "".to_string()));
        }
        Ok(())
    }

    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        c
    }

    fn peek(&self) -> char {
        if self.is_end() {
            return '\0';
        }
        return self.source.chars().nth(self.current).unwrap();
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.source.chars().nth(self.current + 1).unwrap()
    }

    fn string(&mut self) -> Result<(), LoxError> {
        while self.peek() != '"' && !self.is_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_end() {
            let e = LoxError::new(
                self.line,
                "".to_string(),
                "Unexpected character.".to_string(),
            );
            eprintln!("{}", e);
            return Err(e);
        }

        self.advance();
        let value = util::substring(&self.source, self.start + 1, self.current - 1);
        self.add_token_literal(TokenType::String, Some(Literal::String(value.to_string())));
        Ok(())
    }

    fn number(&mut self) {
        while self.peek().is_numeric() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_numeric() {
            self.advance();

            while self.peek().is_numeric() {
                self.advance();
            }
        }

        self.add_token_literal(
            TokenType::Number,
            Some(Literal::Number(
                util::substring(&self.source, self.start, self.current)
                    .to_string()
                    .parse::<f64>()
                    .unwrap(),
            )),
        )
    }

    fn identifier(&mut self) {
        while self.peek().is_alphabetic() || self.peek() == '_' {
            self.advance();
        }
        let text = util::substring(&self.source, self.start, self.current);
        let ty = self.keywords.get(text).unwrap_or(&TokenType::Identifier);
        self.add_token(ty.to_owned());
    }

    fn add_token(&mut self, ty: TokenType) {
        self.add_token_literal(ty, None);
    }

    fn add_token_literal(&mut self, ty: TokenType, literal: Option<Literal>) {
        let text = util::substring(&self.source, self.start, self.current).to_string();
        // let result = self.source.get(self.start..self.current).unwrap();
        self.tokens.push(Token::new(ty, text, literal, self.line));
    }

    fn is_match(&mut self, expected: char) -> bool {
        if self.is_end() {
            return false;
        }
        if expected != self.source.chars().nth(self.current).unwrap() {
            return false;
        }
        self.current += 1;
        true
    }

    fn is_end(&self) -> bool {
        self.current >= self.source.len()
    }
}
