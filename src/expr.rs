use crate::evaluate::evaluate;
use crate::normalize::normalize_variables;
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

    pub fn evaluate(self) -> Expression {
        evaluate(self)
    }

    pub fn normalize(self) -> Expression {
        normalize_variables(self)
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

impl PartialEq for Expression {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Expression::Variable(lhs), Expression::Variable(rhs)) => lhs == rhs,
            (
                Expression::Function(lhs_param, lhs_body),
                Expression::Function(rhs_param, rhs_body),
            ) => lhs_param == rhs_param && lhs_body == rhs_body,
            (
                Expression::Application(lhs_lhs, lhs_rhs),
                Expression::Application(rhs_lhs, rhs_rhs),
            ) => lhs_lhs == rhs_lhs && lhs_rhs == rhs_rhs,
            _ => false,
        }
    }
}
