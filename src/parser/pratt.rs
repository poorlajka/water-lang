use crate::lexer::lang_token::{self, Token};
use crate::parser::lang_ast::{self, span_from_to, span_from_to_node};
use crate::parser::lang_ast::{BinaryOp, Expr, Expression, Node, Pat, Pattern, Statement, Stmt};
use crate::parser::lang_parser::{create_node, parse_statement, ParsingError, TokenStream};
use logos::Span;

fn parse_block(token_stream: &mut TokenStream) -> Result<Node<Expression>, ParsingError> {
    token_stream.skip_newlines();
    let (_, block_span_start) = token_stream.expect(Token::Indent)?;

    let mut statements = Vec::new();
    let block_span_end = loop {
        token_stream.skip_newlines();
        match token_stream.peek() {
            Some((Token::Dedent, block_span_end)) => {
                token_stream.next(); // consume }
                break block_span_end;
            }
            None => {
                return Err(ParsingError::new("Unexpected EOF", None));
            }
            Some(_) => {
                let stmt = parse_statement(token_stream)?;
                statements.push(stmt);
            }
        }
    };

    let final_expr = if let Some(stmt_node) = statements.pop() {
        match stmt_node {
            Statement::Expression(expr) => Some(Box::new(expr)),
            _ => {
                statements.push(stmt_node);
                None
            }
        }
    } else {
        None
    };

    let block_span = span_from_to(block_span_start, block_span_end);
    Ok(create_node(
        token_stream,
        block_span,
        Expression::Block {
            statements,
            final_expr,
        },
    ))
}

fn parse_conditional(token_stream: &mut TokenStream) -> Result<Node<Expression>, ParsingError> {
    let condition = parse_expression(token_stream, 0)?;

    let then_branch = match token_stream.peek() {
        Some((Token::Then, _span)) => {
            token_stream.next();
            parse_expression(token_stream, 0)?
        }
        _ => parse_block(token_stream)?,
    };

    let else_branch = match token_stream.peek() {
        Some((Token::Else, _span)) => {
            token_stream.next();
            if matches!(token_stream.peek(), Some((Token::Indent, _))) {
                Some(Box::new(parse_block(token_stream)?))
            } else {
                Some(Box::new(parse_expression(token_stream, 0)?))
            }
        }
        _ => None,
    };

    let conditional_span = match &else_branch {
        Some(else_branch) => span_from_to_node(&condition, else_branch),
        None => span_from_to_node(&condition, &then_branch),
    };

    Ok(create_node(
        token_stream,
        conditional_span,
        Expression::Conditional {
            condition: Box::new(condition),
            then_branch: Box::new(then_branch),
            else_branch: else_branch,
        },
    ))
}

fn parse_array(
    token_stream: &mut TokenStream,
    start_span: Span,
) -> Result<Node<Expression>, ParsingError> {
    let mut elements = Vec::new();

    loop {
        token_stream.skip_newlines();

        match token_stream.peek() {
            Some((Token::RBracket, end_span)) => {
                token_stream.next();
                let span = span_from_to(start_span, end_span);
                return Ok(create_node(token_stream, span, Expression::Array(elements)));
            }
            None => return Err(ParsingError::new("Unexpected EOF in array", None)),
            _ => {
                let expr = parse_expression(token_stream, 0)?;
                elements.push(expr);

                match token_stream.peek() {
                    Some((Token::Comma, _)) => {
                        token_stream.next();
                    }
                    Some((Token::RBracket, _)) => {}
                    Some((_, span)) => {
                        return Err(ParsingError::new("Expected ',' or ']'", Some(span)))
                    }
                    None => return Err(ParsingError::new("Unexpected EOF in array", None)),
                }
            }
        }
    }
}

fn parse_prefix(token_stream: &mut TokenStream) -> Result<Node<Expression>, ParsingError> {
    match token_stream.next() {
        Some((Token::If, _span)) => parse_conditional(token_stream),
        Some((Token::Integer(i), span)) => {
            Ok(create_node(token_stream, span, Expression::Integer(i)))
        }
        Some((Token::Float(f), span)) => Ok(create_node(token_stream, span, Expression::Float(f))),
        Some((Token::DoubleQuotedString(s), span)) => {
            Ok(create_node(token_stream, span, Expression::String(s)))
        }
        Some((Token::Identifier(i), span)) => {
            Ok(create_node(token_stream, span, Expression::Variable(i)))
        }
        Some((Token::LBracket, start_span)) => parse_array(token_stream, start_span),
        Some((Token::LParen, _)) => {
            let expr = parse_expression(token_stream, 0)?;

            match token_stream.next() {
                Some((Token::RParen, _)) => Ok(expr),

                Some((_token, span)) => Err(ParsingError::new("Expected ')'", Some(span))),

                None => Err(ParsingError::new("Unexpected end of input", None)),
            }
        }

        Some((_token, span)) => Err(ParsingError::new("Unexpected token in prefix", Some(span))),
        None => Err(ParsingError::new("Unexpected end of input", None)),
    }
}

pub fn parse_expression(
    token_stream: &mut TokenStream,
    min_bp: u8,
) -> Result<Node<Expression>, ParsingError> {
    let mut lhs = parse_prefix(token_stream)?;

    loop {
        // --- Postfix operators ---
        match token_stream.peek() {
            Some((Token::LBracket, _)) => {
                // Indexing
                token_stream.next(); // consume '['
                let index_expr = parse_expression(token_stream, 0)?;

                let end_span = match token_stream.next() {
                    Some((Token::RBracket, span)) => span,
                    Some((_, span)) => return Err(ParsingError::new("Expected ']'", Some(span))),
                    None => return Err(ParsingError::new("Unexpected EOF", None)),
                };

                let span = span_from_to_node(&lhs, &index_expr);

                lhs = create_node(
                    token_stream,
                    span,
                    Expression::Index {
                        target: Box::new(lhs),
                        index: Box::new(index_expr),
                    },
                );

                continue; // Postfix operator handled, re-check for chaining
            }

            Some((Token::LParen, _)) => {
                // Function call
                token_stream.next(); // consume '('
                let mut args = Vec::new();

                if !matches!(token_stream.peek(), Some((Token::RParen, _))) {
                    loop {
                        let arg = parse_expression(token_stream, 0)?;
                        args.push(arg);

                        match token_stream.peek() {
                            Some((Token::Comma, _)) => {
                                token_stream.next();
                            }
                            Some((Token::RParen, _)) => break,
                            Some((_, span)) => {
                                return Err(ParsingError::new("Expected ',' or ')'", Some(span)))
                            }
                            None => {
                                return Err(ParsingError::new(
                                    "Unexpected EOF in function call",
                                    None,
                                ))
                            }
                        }
                    }
                }

                let end_span = match token_stream.next() {
                    Some((Token::RParen, span)) => span,
                    Some((_, span)) => return Err(ParsingError::new("Expected ')'", Some(span))),
                    None => return Err(ParsingError::new("Unexpected EOF in function call", None)),
                };

                let span = span_from_to_node(
                    &lhs,
                    &Node::new(0, end_span, Expression::Tuple(args.clone())),
                ); // dummy node for span

                lhs = create_node(
                    token_stream,
                    span,
                    Expression::FunctionCall {
                        callee: Box::new(lhs),
                        arguments: args,
                    },
                );

                continue; // Postfix operator handled
            }

            _ => {}
        }

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
        let span = span_from_to_node(&lhs, &rhs);

        lhs = match bin_op {
            BinaryOp::Assign => {
                let pattern = lhs.into_pattern()?;
                create_node(
                    token_stream,
                    span,
                    Expression::Assignment {
                        lhs: pattern,
                        rhs: Box::new(rhs),
                    },
                )
            }
            _ => create_node(
                token_stream,
                span,
                Expression::Binary {
                    lhs: Box::new(lhs),
                    op: bin_op,
                    rhs: Box::new(rhs),
                },
            ),
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
