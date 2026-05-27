mod pratt;
mod token_stream;
mod utility;

use crate::ast::{Module, Statement, ImportItem};
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

    if let Some((Token::Return, _)) = token_stream.peek() {
        token_stream.next();
        let value = match token_stream.peek() {
            None | Some((Token::Newline, _)) | Some((Token::Dedent, _)) | Some((Token::Eof, _)) => None,
            _ => Some(pratt::parse_expression(token_stream, 0)?),
        };
        token_stream.expect_statement_end()?;
        return Ok(Statement::Return(value));
    }

    if let Some((Token::Break, _)) = token_stream.peek() {
        token_stream.next();
        token_stream.expect_statement_end()?;
        return Ok(Statement::Break);
    }

    if let Some((Token::Continue, _)) = token_stream.peek() {
        token_stream.next();
        token_stream.expect_statement_end()?;
        return Ok(Statement::Continue);
    }

    if let Some((Token::From, _)) = token_stream.peek() {
        token_stream.next();
        let path = parse_module_path(token_stream)?;
        token_stream.expect(Token::Import)?;
        let items = parse_import_items(token_stream)?;
        token_stream.expect_statement_end()?;
        return Ok(Statement::ImportFrom { path, items });
    }

    if let Some((Token::Import, _)) = token_stream.peek() {
        token_stream.next();
        let path = parse_module_path(token_stream)?;
        let alias = if matches!(token_stream.peek(), Some((Token::As, _))) {
            token_stream.next();
            match token_stream.next() {
                Some((Token::Identifier(s), _)) => Some(s),
                _ => return Err(ParsingError::new("Expected alias after 'as'", None)),
            }
        } else {
            None
        };
        token_stream.expect_statement_end()?;
        return Ok(Statement::ImportModule { path, alias });
    }

    match pratt::parse_expression(token_stream, 0) {
        Ok(expression) => {
            token_stream.expect_statement_end()?;
            Ok(Statement::Expression(expression))
        }
        Err(error) => Err(error),
    }
}

fn parse_module_path(token_stream: &mut TokenStream) -> Result<String, ParsingError> {
    let mut parts = Vec::new();
    match token_stream.next() {
        Some((Token::Identifier(s), _)) => parts.push(s),
        _ => return Err(ParsingError::new("Expected module path", None)),
    }
    while matches!(token_stream.peek(), Some((Token::Slash, _))) {
        token_stream.next();
        match token_stream.next() {
            Some((Token::Identifier(s), _)) => parts.push(s),
            _ => return Err(ParsingError::new("Expected path segment after '/'", None)),
        }
    }
    Ok(parts.join("/"))
}

fn parse_import_items(token_stream: &mut TokenStream) -> Result<Vec<ImportItem>, ParsingError> {
    let mut items = Vec::new();
    loop {
        let name = match token_stream.next() {
            Some((Token::Identifier(s), _)) => s,
            _ => return Err(ParsingError::new("Expected import name", None)),
        };
        let alias = if matches!(token_stream.peek(), Some((Token::As, _))) {
            token_stream.next();
            match token_stream.next() {
                Some((Token::Identifier(s), _)) => Some(s),
                _ => return Err(ParsingError::new("Expected alias after 'as'", None)),
            }
        } else {
            None
        };
        items.push(ImportItem { name, alias });
        if !matches!(token_stream.peek(), Some((Token::Comma, _))) {
            break;
        }
        token_stream.next();
    }
    Ok(items)
}
