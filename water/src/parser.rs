mod pratt;
mod token_stream;
mod utility;

use crate::ast::{Module, Statement};
use crate::lexer::token::Token;
use crate::parser::token_stream::TokenStream;
use logos::Span;

pub struct ParsingArtifacts {
    pub ast: Module,
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

pub fn parse_module(
    tokens: &[(Token, Span)],
    name: &String,
) -> ParsingArtifacts {
    let mut parsing_artifacts = ParsingArtifacts {
        ast: Module {
            name: name.to_string(),
            statements: Vec::new(),
        },
        errors: Vec::new(),
    };

    let mut token_stream = TokenStream::new(tokens.to_vec());

    while token_stream.peek().is_some() {
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
fn parse_statement(
    token_stream: &mut TokenStream,
) -> Result<Statement, ParsingError> {
    token_stream.skip_newlines();
    token_stream.save_pos();
    match pratt::parse_expression(token_stream, 0) {
        Ok(expression) => {
            return Ok(Statement::Expression(expression));
        }
        Err(error) => {
            println!("{:?}", error);
            token_stream.backtrack();
        }
    }

    Err(ParsingError::new("Parsing error", None))
}
