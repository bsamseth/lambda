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

    // If this is an application:
    lhs: Option<Box<Expression>>,
    rhs: Option<Box<Expression>>,
}

pub fn parse(maybe_tokens: &LexResult) -> Option<Expression> {
    match maybe_tokens {
        Ok(tokens) => parse_expression(&mut TokenIterator::new(tokens)),
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

#[derive(Copy, Clone)]
struct TokenIterator<'a> {
    tokens: &'a Vec<Token>,
    index: usize,
}

impl<'a> TokenIterator<'a> {
    fn new(tokens: &'a Vec<Token>) -> Self {
        Self { tokens, index: 0 }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.index)
    }
}

impl<'a> Iterator for TokenIterator<'a> {
    type Item = &'a Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.tokens.len() {
            None
        } else {
            let token = &self.tokens[self.index];
            self.index += 1;
            Some(token)
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

fn expression_end(tokens: &TokenIterator) -> usize {
    let mut depth = 0;
    let mut tokens = tokens.clone();
    while let Some(token) = tokens.next() {
        depth += match token {
            Token::LeftParen => 1,
            Token::RightParen => -1,
            _ => 0,
        };
        if depth < 0 {
            break;
        }
    }
    tokens.index
}

fn parse_expression(tokens: &mut TokenIterator) -> Option<Expression> {
    let mut last_expr: Option<Expression> = None;
    let expression_end = expression_end(tokens);
    // println!(
    //     "starting parsing with offset {}, end {}",
    //     tokens.index, expression_end
    // );
    while let Some(token) = tokens.next() {
        // println!("parsing token {:?}", token);
        let expr = match token {
            Token::Dot => panic!("Syntax error: Unexpected dot outside of function."),
            Token::Variable(label) => Some(Expression::new_variable(label)),
            Token::LeftParen => parse_expression(tokens),
            Token::RightParen => break,
            Token::Lambda => {
                let mut params: Vec<String> = Vec::new();
                loop {
                    match tokens.next() {
                        Some(Token::Variable(label)) => params.push(label.clone()),
                        _ => panic!("Syntax error: Expected variable after lambda."),
                    };
                    match tokens.next() {
                        Some(Token::Dot) => true,
                        _ => panic!("Syntax error: Expected dot after parameter."),
                    };
                    match tokens.peek() {
                        Some(Token::Lambda) => tokens.next(),
                        _ => break,
                    };
                }

                // println!(
                //     "Parsed params for lambda: {:?}, offset {}",
                //     &params, tokens.index
                // );

                let body = parse_expression(tokens);
                match body {
                    Some(e) => Some(Expression::new_function(params, e)),
                    None => panic!("Syntax error: Missing function body"),
                }
            }
        };

        last_expr = match expr {
            None => continue,
            Some(y) => match last_expr {
                None => Some(y),
                Some(x) => Some(Expression::new_application(x, y)),
            },
        };
        // println!("at offset {}, last_expr {:?}", tokens.index, last_expr);

        if tokens.index == expression_end {
            break;
        }
    }
    // println!("returning {} {:?}", tokens.index, last_expr);

    last_expr
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
    #[test]
    fn lambdas() {
        check_parsed_correctly("λx.x", "λx.x");
        check_parsed_correctly("λx.λy.x (x y)", "λx.λy.x (x y)");
        check_parsed_correctly("λf.λx.f (f (f x))", "λf.λx.f (f (f x))");
        check_parsed_correctly("λf.λx.f (f (f x))", "λf.λx.f (f (f x))");
    }
    #[test]
    fn combined_expressions() {
        check_parsed_correctly("(λx.x) (λy.y) z", "((λx.x) (λy.y)) z");
        check_parsed_correctly("λu.(λx.x) (λy.y)", "λu.(λx.x) (λy.y)");
        check_parsed_correctly(
            "(λh.(λx.h (x x)) (λx.h (x x))) g",
            "(λh.(λx.h (x x)) (λx.h (x x))) g",
        );
        check_parsed_correctly(
            "g (\n(λx.g(x x))(λx.g (  x   x)))",
            "g ((λx.g (x x)) (λx.g (x x)))",
        );
    }
}
