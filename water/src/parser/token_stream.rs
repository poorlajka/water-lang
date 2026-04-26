use crate::lexer::token::Token;
use crate::parser::ParsingError;
use logos::Span;

pub struct TokenStream {
    tokens: Vec<(Token, Span)>,
    pos: usize,
    save_pos: usize,
    _group_depth: usize,
    next_id: usize,
}

impl TokenStream {
    pub fn new(tokens: Vec<(Token, Span)>) -> Self {
        Self {
            tokens,
            pos: 0,
            save_pos: 0,
            _group_depth: 0,
            next_id: 0,
        }
    }

    pub fn next(&mut self) -> Option<(Token, Span)> {
        if self.pos < self.tokens.len() - 1 {
            self.pos += 1;
            Some(self.tokens[self.pos].clone())
        }
        else {
            None
        }
    }

    pub fn peek(&self) -> Option<(Token, Span)> {
        if self.pos < self.tokens.len() - 1 {
            Some(self.tokens[self.pos + 1].clone())
        }
        else {
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

    pub fn expect(
        &mut self,
        expected: Token,
    ) -> Result<(Token, Span), ParsingError> {
        match self.next() {
            Some((tok, span)) if tok == expected => Ok((tok, span)),
            Some((_, span)) => Err(ParsingError::new(
                &format!("Expected {:?}", expected),
                Some(span),
            )),
            None => Err(ParsingError::new(
                &format!("Expected {:?}", expected),
                None,
            )),
        }
    }

    pub fn next_id(&mut self) -> usize {
        let id = self.next_id;
        self.next_id += 1;

        id
    }

    pub fn expect_statement_end(&mut self) -> Result<(), ParsingError> {
    match self.peek() {
        None | Some((Token::Newline, _)) | Some((Token::Dedent, _)) | Some((Token::Eof, _)) => {
            self.skip_newlines();
            Ok(())
        }
        Some((_, span)) => Err(ParsingError::new(
            "Expected newline or end of file after statement.",
            Some(span),
        )),
    }
}
}
