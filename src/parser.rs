// #![allow(dead_code)]
#![allow(unused_variables)]
mod expr;

use super::lexer;
use super::lexer::token::Token;
use super::lexer::token::TokenIterator;
use super::lexer::LexResult;
use crate::error;
use expr::Expression;
use std::str;

pub fn parse(maybe_tokens: &LexResult) -> Option<Expression> {
    match maybe_tokens {
        Ok(tokens) => parse_expression(&mut TokenIterator::new(tokens)),
        Err(_) => None,
    }
}

impl str::FromStr for Expression {
    type Err = error::SyntaxError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = lexer::lex(s)?;
        let mut tokens = TokenIterator::new(&tokens);
        let expr = parse_expression(&mut tokens);
        match expr {
            Some(e) => Ok(e),
            None => Err("Failed to parse code end of input".into()),
        }
    }
}

fn parse_expression(tokens: &mut TokenIterator) -> Option<Expression> {
    let mut last_expr: Option<Expression> = None;
    let expression_end = tokens.end_of_expr();
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

        if tokens.index() == expression_end {
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
