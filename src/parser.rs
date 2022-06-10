use super::lexer;
use crate::error;
use crate::error::LambdaError;
use crate::expr::Expression;
use crate::token::Token;
use crate::token::TokenIterator;
use std::str;

pub type ParseResult = Result<Expression, error::LambdaError>;

impl str::FromStr for Expression {
    type Err = LambdaError;

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
            Token::Dot => Err(LambdaError::SyntaxError(
                "Unexpected dot outside of function.".to_string(),
            )),
            Token::Variable(label) => Ok(Expression::new_variable(label)),
            Token::LeftParen => parse_expression(tokens),
            Token::RightParen => break,
            Token::Lambda => {
                let param = match tokens.next() {
                    Some(Token::Variable(label)) => label.clone(),
                    _ => {
                        return Err(LambdaError::SyntaxError(
                            "Expected variable after lambda.".to_string(),
                        ))
                    }
                };
                assert!(match tokens.next() {
                    Some(Token::Dot) => true,
                    _ => {
                        return Err(LambdaError::SyntaxError(
                            "Expected dot after parameter.".to_string(),
                        ));
                    }
                });

                let body = parse_expression(tokens)?;
                Ok(Expression::new_function(param, body))
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
        None => Err(LambdaError::SyntaxError(
            "The code does not contain an expression.".to_string(),
        )),
    }
}
