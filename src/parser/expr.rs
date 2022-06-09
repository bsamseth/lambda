use std::fmt;

#[derive(Debug, Clone)]
pub enum Expression {
    Variable(String),
    Function(String, Box<Expression>),
    Application(Box<Expression>, Box<Expression>),
}

impl Expression {
    pub fn new_variable(label: &str) -> Self {
        Expression::Variable(label.to_string())
    }

    pub fn new_function(param: String, body: Expression) -> Self {
        Expression::Function(param, Box::new(body))
    }

    pub fn new_application(lhs: Expression, rhs: Expression) -> Self {
        Expression::Application(Box::new(lhs), Box::new(rhs))
    }
}

impl std::ops::Mul for Expression {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Expression::new_application(self, other)
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expression::Variable(label) => write!(f, "{}", label),
            Expression::Function(param, body) => write!(f, "λ{}.{}", param, body),
            Expression::Application(lhs, rhs) => match lhs.as_ref() {
                Expression::Variable(_) => match rhs.as_ref() {
                    Expression::Variable(_) => write!(f, "{} {}", lhs, rhs),
                    _ => write!(f, "{} ({})", lhs, rhs),
                },
                _ => match rhs.as_ref() {
                    Expression::Variable(_) => write!(f, "({}) {}", lhs, rhs),
                    _ => write!(f, "({}) ({})", lhs, rhs),
                },
            },
        }
    }
}
