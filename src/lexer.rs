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

        if current_identifier.len() > 0 {
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

    if current_identifier.len() > 0 {
        tokens.push(Token::Variable(current_identifier));
    }

    Ok(tokens)
}
