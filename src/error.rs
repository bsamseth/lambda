use std::fmt;

/// An error with a lambda expression.
#[derive(Debug, Clone)]
pub enum LambdaError {
    SyntaxError(String),
    RuntimeError(String),
}

impl fmt::Display for LambdaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LambdaError::SyntaxError(s) => write!(f, "SyntaxError: {}", s),
            LambdaError::RuntimeError(s) => write!(f, "RuntimeError: {}", s),
        }
    }
}
