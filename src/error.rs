use core::fmt;

use crate::token::{Token, TokenType};

#[derive(Clone)]
pub struct LoxError {
    pub token: Option<Token>,
    pub line: usize,
    pub r#where: String,
    pub message: String,
}

impl LoxError {
    pub fn new(line: usize, r#where: String, message: String) -> Self {
        Self {
            token: None,
            line,
            r#where,
            message,
        }
    }

    pub fn new_with_token(token: Token, message: String) -> Self {
        Self {
            token: Some(token.clone()),
            line: token.get_line(),
            r#where: "".to_string(),
            message,
        }
    }

    pub fn error(token: Token, message: String) {
        if token.get_token_type() == TokenType::Eof {
            LoxError::report(token.get_line(), "at end".to_string(), message);
        } else {
            LoxError::report(
                token.get_line(),
                "at '".to_string() + token.get_lexeme().as_str() + "'",
                message,
            )
        }
    }

    pub fn report(line: usize, r#where: String, message: String) {
        eprintln!("[line {}] Error {}: {}", line, r#where, message);
    }
}

impl fmt::Debug for LoxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // f.debug_struct("Error").finish()
        f.write_fmt(format_args!(
            "[line {}] Error {}: {}",
            self.line, self.r#where, self.message
        ))
    }
}

impl fmt::Display for LoxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("[line {}] error: {}", self.line, self.message))
    }
}

impl std::error::Error for LoxError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self)
    }
}
