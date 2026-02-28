use crate::lexer::lang_token::Token;
use crate::parser::lang_ast;
use crate::parser::lang_ast::{Expression, Node, Statement};
use crate::parser::pratt;
use logos::Span;

pub struct ParsingArtifacts {
    pub ast: lang_ast::Module,
    pub errors: Vec<ParsingError>,
}

#[derive(Debug)]
pub struct ParsingError {
    pub message: String,
    pub span: Option<Span>,
}

impl ParsingError {
    pub fn new(message: &str, span: Option<Span>) -> Self {
        Self {
            message: message.into(),
            span,
        }
    }
}

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

    pub fn save(&mut self) {
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

pub fn create_node<T>(token_stream: &mut TokenStream, span: Span, kind: T) -> Node<T> {
    let id = token_stream.next_id();
    Node::<T>::new(id, span, kind)
}

pub fn parse_module(tokens: &Vec<(Token, Span)>, name: &String) -> ParsingArtifacts {
    let mut parsing_artifacts = ParsingArtifacts {
        ast: lang_ast::Module {
            name: name.to_string(),
            statements: Vec::new(),
        },
        errors: Vec::new(),
    };

    let mut token_stream = TokenStream {
        tokens: tokens.to_vec(),
        pos: 0,
        save_pos: 0,
        group_depth: 0,
        next_id: 0,
    };

    while !token_stream.peek().is_none() {
        match parse_statement(&mut token_stream) {
            Ok(statement) => parsing_artifacts.ast.statements.push(statement),
            Err(error) => parsing_artifacts.errors.push(error),
        }
    }

    parsing_artifacts
}

/*
    <statement> ::= <assignment>
        | <expression>
        | <loop>
        | return
        | import
*/
pub fn parse_statement(token_stream: &mut TokenStream) -> Result<Statement, ParsingError> {
    token_stream.skip_newlines();
    let save_pos = token_stream.pos;
    match pratt::parse_expression(token_stream, 0) {
        Ok(expression) => {
            return Ok(Statement::Expression(expression));
        }
        Err(error) => {
            println!("{:?}", error);
        }
    }

    Err(ParsingError::new("Parsing error", None))
}
