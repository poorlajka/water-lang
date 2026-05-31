pub mod token;

use crate::lexer::token::Token;
use logos::{Logos, Span};

#[derive(Debug, PartialEq, Clone, Default)]
pub enum LexingError {
    NumberParseError,
    #[default]
    Other,
}

impl From<std::num::ParseIntError> for LexingError {
    fn from(_: std::num::ParseIntError) -> Self {
        LexingError::NumberParseError
    }
}

impl From<std::num::ParseFloatError> for LexingError {
    fn from(_: std::num::ParseFloatError) -> Self {
        LexingError::NumberParseError
    }
}

fn count_columns(s: &str) -> usize {
    let mut column = 0;
    for c in s.chars() {
        match c {
            ' ' => column += 1,
            '\t' => column = (column / 8 + 1) * 8,
            _ => break,
        }
    }
    column
}

pub struct LexingArtifacts {
    pub tokens: Vec<(Token, Span)>,
    pub errors: Vec<(LexingError, Span)>,
}

pub fn tokenize(code: &str) -> LexingArtifacts {
    let mut lexing_artifacts = LexingArtifacts {
        tokens: vec![(Token::LParen, 2..3)],
        errors: Vec::new(),
    };

    let mut lexer = Token::lexer(code).spanned();
    let mut indent_stack: Vec<usize> = vec![0];
    let mut bracket_depth: usize = 0;

    while let Some((token, span)) = lexer.next() {
        match token {
            Err(ref error) => lexing_artifacts.errors.push((error.clone(), span.clone())),

            Ok(Token::LBracket) | Ok(Token::LParen) => {
                bracket_depth += 1;
                lexing_artifacts.tokens.push((token.unwrap(), span));
            }
            Ok(Token::RBracket) | Ok(Token::RParen) => {
                bracket_depth = bracket_depth.saturating_sub(1);
                lexing_artifacts.tokens.push((token.unwrap(), span));
            }

            Ok(Token::Newline) if bracket_depth > 0 => {
                // Inside brackets: ignore newlines and any indentation changes.
            }

            Ok(Token::Newline) => {
                lexing_artifacts.tokens.push((Token::Newline, span.clone()));

                let slice = lexer.remainder();
                let leading_ws: String = slice
                    .chars()
                    .take_while(|c| *c == ' ' || *c == '\t')
                    .collect();

                // Ignore blank lines, they don't affect indentation
                let next_non_ws = slice[leading_ws.len()..].chars().next();
                if matches!(next_non_ws, None | Some('\n') | Some('\r')) {
                    continue;
                }

                let new_indent = count_columns(&leading_ws);
                let current_indent = *indent_stack.last().unwrap();

                if new_indent > current_indent {
                    indent_stack.push(new_indent);
                    lexing_artifacts.tokens.push((Token::Indent, span.clone()));
                } else if new_indent < current_indent {
                    while *indent_stack.last().unwrap() > new_indent {
                        indent_stack.pop();
                        lexing_artifacts.tokens.push((Token::Dedent, span.clone()));
                    }
                    lexing_artifacts.tokens.push((Token::Newline, span.clone()));

                    if *indent_stack.last().unwrap() != new_indent {
                        lexing_artifacts.errors.push((LexingError::Other, span.clone()));
                    }
                }
            }

            Ok(token) => lexing_artifacts.tokens.push((token, span)),
        }
    }

    lexing_artifacts
}
