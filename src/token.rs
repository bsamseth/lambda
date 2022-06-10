#[derive(Debug, PartialEq)]
pub enum Token {
    Variable(String),
    Lambda,
    Dot,
    LeftParen,
    RightParen,
}

#[derive(Copy, Clone)]
pub struct TokenIterator<'a> {
    tokens: &'a Vec<Token>,
    index: usize,
}

impl<'a> TokenIterator<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Self { tokens, index: 0 }
    }

    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.index)
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn end_of_expr(&self) -> usize {
        let mut depth = 0;
        let mut tokens = *self;
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
        tokens.index()
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
