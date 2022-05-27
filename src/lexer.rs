#[derive(Debug, PartialEq)]
pub enum Token {
    Variable(String),
    Lambda,
    Dot,
    LeftParen,
    RightParen,
}

pub type LexResult = std::io::Result<Vec<Token>>;

pub fn lex(code: &str) -> LexResult {
    let mut tokens: Vec<Token> = Vec::new();
    let mut current_identifier = String::new();

    for c in code.chars() {
        match c {
            'a'..='z' | 'A'..='Z' | '_' => {
                current_identifier.push(c);
                continue;
            }
            _ => (),
        }

        if !current_identifier.is_empty() {
            tokens.push(Token::Variable(current_identifier));
            current_identifier = String::new();
        }

        let token = match c {
            ' ' | '\t' | '\n' | '\r' => continue,
            '.' => Token::Dot,
            '\\' | 'λ' => Token::Lambda,
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            _ => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Syntax error",
                ))
            }
        };

        tokens.push(token);
    }

    if !current_identifier.is_empty() {
        tokens.push(Token::Variable(current_identifier));
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_lexed_correctly(code: &str, expected: Vec<Token>) {
        let res = lex(code);
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res.len(), expected.len());
        for (actual, expected) in std::iter::zip(res, expected) {
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn single_variable() {
        check_lexed_correctly("x", vec![Token::Variable("x".to_string())]);
    }
    #[test]
    fn multi_char_variables() {
        check_lexed_correctly(
            "foo bar",
            vec![
                Token::Variable("foo".to_string()),
                Token::Variable("bar".to_string()),
            ],
        );
    }

    #[test]
    fn compound_statement() {
        check_lexed_correctly(
            "\\  x. \n\\yy . \t(u\n\r yy)",
            vec![
                Token::Lambda,
                Token::Variable("x".to_string()),
                Token::Dot,
                Token::Lambda,
                Token::Variable("yy".to_string()),
                Token::Dot,
                Token::LeftParen,
                Token::Variable("u".to_string()),
                Token::Variable("yy".to_string()),
                Token::RightParen,
            ],
        );
    }
}
