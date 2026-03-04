use crate::parser::ParsingError;
use crate::lexer::token::Token;
use logos::Span;

pub struct TokenStream {
    tokens: Vec<(Token, Span)>,
    pos: usize,
    save_pos: usize,
    group_depth: usize,
    next_id: usize,
}

impl TokenStream {
    pub fn next(&mut self) -> Option<(Token, Span)> {
        if self.pos < self.tokens.len() - 1 {
            self.pos += 1;
            Some(self.tokens[self.pos].clone())
        } else {
            None
        }
    }

    pub fn peek(&self) -> Option<(Token, Span)> {
        if self.pos < self.tokens.len() - 1 {
            Some(self.tokens[self.pos + 1].clone())
        } else {
            None
        }
    }

    pub fn save_pos(&mut self) {
        self.save_pos = self.pos;
    }

    pub fn backtrack(&mut self) {
        self.pos = self.save_pos;
    }

    pub fn skip_newlines(&mut self) {
        while matches!(self.peek(), Some((Token::Newline, _))) {
            self.next();
        }
    }

    pub fn expect(&mut self, expected: Token) -> Result<(Token, Span), ParsingError> {
        match self.next() {
            Some((tok, span)) if tok == expected => Ok((tok, span)),
            Some((_, span)) => Err(ParsingError::new(
                &format!("Expected {:?}", expected),
                Some(span),
            )),
            None => Err(ParsingError::new(&format!("Expected {:?}", expected), None)),
        }
    }

    pub fn next_id(&mut self) -> usize {
        let id = self.next_id;
        self.next_id += 1;

        id
    }
}
