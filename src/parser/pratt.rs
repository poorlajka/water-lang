use crate::lexer::lang_token::{self, Token};
use crate::parser::lang_ast;
use crate::parser::lang_ast::{BinaryOp, Expression, Pattern, Statement};
use crate::parser::lang_parser::{parse_statement, ParsingError, TokenStream};
use logos::Span;

fn parse_block(token_stream: &mut TokenStream) -> Result<Expression, ParsingError> {
    token_stream.skip_newlines();
    token_stream.expect(Token::Indent)?;

    let mut statements = Vec::new();
    loop {
        token_stream.skip_newlines();
        match token_stream.peek() {
            Some((Token::Dedent, _)) | None => {
                token_stream.next(); // consume }
                break;
            }
            Some((t, span)) => {
                let stmt = parse_statement(token_stream)?;
                statements.push(stmt);
            }
        }
    }

    let final_expr = if let Some(Statement::Expression(expr)) = statements.pop() {
        Some(Box::new(expr))
    } else {
        None
    };

    Ok(Expression::Block {
        statements,
        final_expr,
    })
}

fn parse_conditional(token_stream: &mut TokenStream) -> Result<Expression, ParsingError> {
    let condition = parse_expression(token_stream, 0)?;

    let then_branch = match token_stream.peek() {
        Some((Token::Then, span)) => parse_expression(token_stream, 0)?,
        _ => parse_block(token_stream)?,
    };

    let else_branch = match token_stream.peek() {
        Some((Token::Else, span)) => {
            token_stream.next();
            if matches!(token_stream.peek(), Some((Token::Indent, _))) {
                Some(parse_block(token_stream)?)
            } else {
                Some(parse_expression(token_stream, 0)?)
            }
        }
        _ => None,
    };

    Ok(Expression::Conditional {
        condition: Box::new(condition),
        then_branch: Box::new(then_branch),
        else_branch: Box::new(else_branch),
    })
}

fn convert_expr_to_pattern(expr: Expression) -> Result<Pattern, ParsingError> {
    match expr {
        Expression::Variable(name) => Ok(Pattern::Identifier(name)),

        Expression::Tuple(elements) => {
            let mut patterns = Vec::new();

            for element in elements {
                patterns.push(convert_expr_to_pattern(element)?);
            }

            Ok(Pattern::Tuple(patterns))
        }

        _ => Err(ParsingError {
            message: "Invalid assignment target".into(),
        }),
    }
}

fn parse_prefix(token_stream: &mut TokenStream) -> Result<Expression, ParsingError> {
    match token_stream.next() {
        Some((Token::If, _span)) => parse_conditional(token_stream),
        Some((Token::Integer(i), _span)) => Ok(Expression::Integer(i)),
        Some((Token::Float(f), _span)) => Ok(Expression::Float(f)),
        Some((Token::DoubleQuotedString(s), _span)) => Ok(Expression::String(s)),
        Some((Token::Identifier(i), _span)) => Ok(Expression::Variable(i)),

        Some((Token::LParen, _)) => {
            let expr = parse_expression(token_stream, 0)?;

            match token_stream.next() {
                Some((Token::RParen, _)) => Ok(expr),

                Some((_token, span)) => Err(ParsingError {
                    message: "Expected ')'".to_string(),
                }),

                None => Err(ParsingError {
                    message: "Unexpected end of input".to_string(),
                }),
            }
        }

        Some((_token, span)) => Err(ParsingError {
            message: "Unexpected token in prefix".to_string(),
        }),
        None => Err(ParsingError {
            message: "Unexpected end of input".to_string(),
        }),
    }
}

pub fn parse_expression(
    token_stream: &mut TokenStream,
    min_bp: u8,
) -> Result<Expression, ParsingError> {
    let mut lhs = parse_prefix(token_stream)?;

    loop {
        let op = match token_stream.peek() {
            Some((token, _)) => token,
            None => break,
        };

        let (left_bp, right_bp, bin_op) = match infix_binding_power(&op) {
            Some(info) => info,
            None => break,
        };

        if left_bp < min_bp {
            break;
        }

        token_stream.next();

        let rhs = parse_expression(token_stream, right_bp)?;

        lhs = match bin_op {
            BinaryOp::Assign => {
                let pattern = convert_expr_to_pattern(lhs)?;
                Expression::Assignment {
                    lhs: pattern,
                    rhs: Box::new(rhs),
                }
            }
            _ => Expression::Binary {
                lhs: Box::new(lhs),
                op: bin_op,
                rhs: Box::new(rhs),
            },
        };
    }

    Ok(lhs)
}

fn infix_binding_power(tok: &Token) -> Option<(u8, u8, BinaryOp)> {
    match tok {
        Token::Plus => Some((10, 11, BinaryOp::Add)),
        Token::Minus => Some((10, 11, BinaryOp::Sub)),
        Token::Star => Some((20, 21, BinaryOp::Mul)),
        Token::Slash => Some((20, 21, BinaryOp::Div)),

        Token::Lt => Some((5, 6, BinaryOp::Lt)),
        Token::Gt => Some((5, 6, BinaryOp::Gt)),
        Token::EqEq => Some((4, 5, BinaryOp::Eq)),
        Token::Eq => Some((1, 0, BinaryOp::Assign)),

        _ => None,
    }
}
