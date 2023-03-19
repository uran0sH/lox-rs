use core::fmt;

#[derive(Clone)]
pub struct LoxError {
    pub line: usize,
    pub r#where: String,
    pub message: String,
}

impl LoxError {
    pub fn new(line: usize, r#where: String, message: String) -> Self {
        Self {
            line,
            r#where,
            message,
        }
    }

    pub fn error(line: usize, message: String) {
        LoxError::report(line, "".to_string(), message);
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
