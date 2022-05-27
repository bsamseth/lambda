// #![allow(dead_code)]
#![allow(unused_variables)]

use super::lexer::{LexResult, Token};
use std::fmt;

#[derive(Debug)]
pub enum ExpressionKind {
    Variable,
    Function,
    Application,
}

pub struct Expression {
    kind: ExpressionKind,

    // If this is a variable:
    label: Option<String>,

    // If this is a function:
    params: Option<Vec<String>>,
    body: Option<Box<Expression>>,

    // If this is an applicaiton:
    lhs: Option<Box<Expression>>,
    rhs: Option<Box<Expression>>,
}

pub fn parse(maybe_tokens: &LexResult) -> Option<Expression> {
    match maybe_tokens {
        Ok(tokens) => parse_expression(tokens, 0).1,
        Err(_) => None,
    }
}

impl Expression {
    fn new_variable(label: &str) -> Self {
        Self {
            kind: ExpressionKind::Variable,
            label: Some(String::from(label)),
            params: None,
            body: None,
            lhs: None,
            rhs: None,
        }
    }

    fn new_function(params: Vec<String>, body: Expression) -> Self {
        Self {
            kind: ExpressionKind::Function,
            label: None,
            params: Some(params),
            body: Some(Box::new(body)),
            lhs: None,
            rhs: None,
        }
    }

    fn new_application(lhs: Expression, rhs: Expression) -> Self {
        Self {
            kind: ExpressionKind::Application,
            label: None,
            params: None,
            body: None,
            lhs: Some(Box::new(lhs)),
            rhs: Some(Box::new(rhs)),
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            ExpressionKind::Variable => write!(f, "{}", self.label.as_ref().unwrap()),
            ExpressionKind::Function => write!(
                f,
                "λ{}.{}",
                self.params.as_ref().unwrap().join(".λ"),
                self.body.as_ref().unwrap()
            ),
            ExpressionKind::Application => {
                // TODO: Fix this monstrosity (for some reason this was hard to do simply).
                let &lhs = &self.lhs.as_ref().unwrap();
                let &rhs = &self.rhs.as_ref().unwrap();
                match lhs.kind {
                    ExpressionKind::Variable => match rhs.kind {
                        ExpressionKind::Variable => write!(f, "{} {}", lhs, rhs),
                        _ => write!(f, "{} ({})", lhs, rhs),
                    },
                    _ => match rhs.kind {
                        ExpressionKind::Variable => write!(f, "({}) {}", lhs, rhs),
                        _ => write!(f, "({}) ({})", lhs, rhs),
                    },
                }
            }
        }
    }
}

impl fmt::Debug for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", &self)
    }
}

fn parse_expression(tokens: &Vec<Token>, offset: usize) -> (usize, Option<Expression>) {
    println!("starting parsing with offset {}", offset);
    let mut last_expr: Option<Expression> = None;
    let mut offset = offset;
    while offset < tokens.len() {
        let token = &tokens[offset];
        println!("parsing token {:?}", token);
        let expr;
        match token {
            Token::LeftParen => {
                (offset, expr) = parse_expression(&tokens, offset + 1);
            }
            Token::RightParen => {
                offset += 1;
                break;
            }
            Token::Variable(label) => {
                offset += 1;
                expr = Some(Expression::new_variable(label));
            }
            _ => break,
        };

        last_expr = match expr {
            None => continue,
            Some(y) => match last_expr {
                None => Some(y),
                Some(x) => Some(Expression::new_application(x, y)),
            },
        }
    }
    println!("returning {} {:?}", offset, last_expr);

    (offset, last_expr)
}

#[cfg(test)]
mod tests {

    use super::super::lexer;
    use super::*;

    fn check_parsed_correctly(code: &str, expected: &str) {
        let expr = parse(&lexer::lex(code));
        assert!(expr.is_some());
        assert_eq!(expr.unwrap().to_string(), expected);
    }

    #[test]
    fn single_variable() {
        check_parsed_correctly("x", "x");
    }
    #[test]
    fn two_variable_application() {
        check_parsed_correctly("x y", "x y");
    }
    #[test]
    fn variable_application_with_parens() {
        check_parsed_correctly("(x y)", "x y");
        check_parsed_correctly("(x (y))", "x y");
        check_parsed_correctly("z ((x (y)))", "z (x y)");
        check_parsed_correctly("(z ((x (y))))", "z (x y)");
        check_parsed_correctly("((z u) ((x (y))))", "(z u) (x y)");
        check_parsed_correctly("(x y) (u v)", "(x y) (u v)");
    }
    #[test]
    fn left_associativity() {
        check_parsed_correctly("x y z", "(x y) z");
        check_parsed_correctly("x (y z)", "x (y z)");
        check_parsed_correctly("x (y z) u", "(x (y z)) u");
    }
}
