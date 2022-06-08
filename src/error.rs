use std::fmt;

#[derive(Debug, Clone)]
pub struct SyntaxError(String);

#[derive(Debug, Clone)]
pub struct RuntimeError(String);

impl SyntaxError {
    pub fn new<S: AsRef<str>>(msg: S) -> Self {
        Self(msg.as_ref().to_string())
    }
}

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SyntaxError: {}", self.0)
    }
}

impl<S: AsRef<str>> From<S> for SyntaxError {
    fn from(msg: S) -> Self {
        Self(msg.as_ref().to_string())
    }
}

impl RuntimeError {
    pub fn new<S: AsRef<str>>(msg: S) -> Self {
        Self(msg.as_ref().to_string())
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RuntimeError: {}", self.0)
    }
}

impl<S: AsRef<str>> From<S> for RuntimeError {
    fn from(msg: S) -> Self {
        Self(msg.as_ref().to_string())
    }
}
