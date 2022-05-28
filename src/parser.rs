// #![allow(dead_code)]
#![allow(unused_variables)]
mod expr;

use super::lexer;
use super::lexer::token::Token;
use super::lexer::token::TokenIterator;
use crate::error;
use expr::Expression;
use std::str;

pub type ParseResult = error::Result<Expression>;

impl str::FromStr for Expression {
    type Err = error::SyntaxError;

    fn from_str(s: &str) -> ParseResult {
        let tokens = lexer::lex(s)?;
        let mut tokens = TokenIterator::new(&tokens);
        parse_expression(&mut tokens)
    }
}

fn parse_expression(tokens: &mut TokenIterator) -> ParseResult {
    let mut last_expr: Option<Expression> = None;
    let expression_end = tokens.end_of_expr();

    while let Some(token) = tokens.next() {
        let expr = match token {
            Token::Dot => Err("Unexpected dot outside of function.".into()),
            Token::Variable(label) => Ok(Expression::new_variable(label)),
            Token::LeftParen => parse_expression(tokens),
            Token::RightParen => break,
            Token::Lambda => {
                let mut params: Vec<String> = Vec::new();
                loop {
                    match tokens.next() {
                        Some(Token::Variable(label)) => params.push(label.clone()),
                        _ => return Err("Expected variable after lambda.".into()),
                    };
                    match tokens.next() {
                        Some(Token::Dot) => true,
                        _ => return Err("Expected dot after parameter.".into()),
                    };
                    match tokens.peek() {
                        Some(Token::Lambda) => tokens.next(),
                        _ => break,
                    };
                }

                let body = parse_expression(tokens)?;
                Ok(Expression::new_function(params, body))
            }
        };

        let y = expr?;
        last_expr = match last_expr {
            None => Some(y),
            Some(x) => Some(Expression::new_application(x, y)),
        };

        if tokens.index() == expression_end {
            break;
        }
    }

    match last_expr {
        Some(expr) => Ok(expr),
        None => Err("The code does not contain an expression.".into()),
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    fn check_parsed_correctly(code: &str, expected: &str) {
        let expr: ParseResult = code.parse();
        assert!(expr.is_ok());
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
