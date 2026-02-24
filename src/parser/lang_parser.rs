use crate::lexer::lang_token::Token;
use crate::parser::lang_ast;
use crate::parser::pratt;
use logos::Span;

pub struct ParsingArtifacts {
    pub ast: lang_ast::Module,
    pub errors: Vec<ParsingError>,
}

#[derive(Debug)]
pub struct ParsingError {
    pub message: String,
}

pub struct TokenStream {
    tokens: Vec<(Token, Span)>,
    pos: usize,
    save_pos: usize,
}

impl TokenStream {
    pub fn next(&mut self, offset: usize) -> Option<(Token, Span)> {
        if self.pos < self.tokens.len() - offset {
            self.pos += offset;
            Some(self.tokens[self.pos].clone())
        }
        else {
            None
        }
    }

    pub fn peek(&self, offset: usize) -> Option<(Token, Span)> {
        if self.pos < self.tokens.len() - offset {
            Some(self.tokens[self.pos + offset].clone())
        }
        else {
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
        while matches!(self.peek(1), Some((Token::Newline, _))) {
            self.next(1);
        }
    }
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
    };

    loop {
        match parse_statement(&mut token_stream) {
            Ok(statement) => parsing_artifacts.ast.statements.push(statement),
            Err(error) => parsing_artifacts.errors.push(error),
        }

        if token_stream.peek(1).is_none() {
            break;
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
pub fn parse_statement(token_stream: &mut TokenStream) -> Result<lang_ast::Statement, ParsingError> {

    token_stream.skip_newlines();

    let mut save_pos = token_stream.pos;
    match parse_assignment(token_stream) {
        Ok(assignment) => {
            return Ok(lang_ast::Statement::Assignment(assignment));
        }
        Err(error) => {
            token_stream.pos = save_pos;
        }
    }

    save_pos = token_stream.pos;
    match pratt::parse_expression(token_stream, 0) {
        Ok(expression) => {
            return Ok(lang_ast::Statement::Expression(expression));
        }
        Err(error) => {
            println!("{:?}", error);
        }
    }

    /*
    match token_stream.peek(1) {
        Some((Token::Newline, _)) => {
            token_stream.next(1);
        }
        Some((Token::Dedent, _)) | None => {
            // ok — block ending or EOF
        }
        _ => {
            return Err(ParsingError {
                message: "Expected newline after statement".to_string(),
            });
        }
    }
    */

    Err(ParsingError {message: "Parsing error".to_string()})
}

fn parse_assignment(token_stream: &mut TokenStream) -> Result<lang_ast::Assignment, ParsingError> {
    match token_stream.peek(1) {
        Some((Token::Identifier(identifier), span)) => {
            match token_stream.peek(2) {
                Some((Token::Eq, span)) => {
                    token_stream.next(2);
                    match pratt::parse_expression(token_stream, 0) {
                        Ok(expression) => {
                            return Ok(lang_ast::Assignment {
                                lhs: identifier,
                                rhs: expression,
                            });
                        }
                        Err(error) => {

                        }
                    }
                }
                _ => {

                }
            }
        }
        _ => {

        }
    }

    Err(ParsingError {message: "Parsing error".to_string()})
}