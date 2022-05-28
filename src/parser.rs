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
