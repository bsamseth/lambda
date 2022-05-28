use std::fmt;

pub type Result<T> = std::result::Result<T, SyntaxError>;

#[derive(Debug, Clone)]
pub struct SyntaxError(String);

impl SyntaxError {
    pub fn new<S: AsRef<str>>(msg: S) -> Self {
        SyntaxError(msg.as_ref().to_string())
    }
}

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SyntaxError: {}", self.0)
    }
}

impl<S: AsRef<str>> From<S> for SyntaxError {
    fn from(msg: S) -> Self {
        SyntaxError(msg.as_ref().to_string())
    }
}
