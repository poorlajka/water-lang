use crate::lexer::token::Token;
use crate::parser::ast::{BinaryOp, Expression, Node, Statement};
use crate::parser::ast::{FunctionSignature, UnaryOp};
use crate::parser::token_stream::TokenStream;
use crate::parser::utility::{create_node, span_from_to, span_from_to_node};
use crate::parser::{parse_statement, ParsingError};
use logos::Span;

pub fn parse_expression(
    token_stream: &mut TokenStream,
    min_bp: u8,
) -> Result<Node<Expression>, ParsingError> {
    let mut lhs = parse_prefix(token_stream)?;

    loop {
        if let Some(new_lhs) = parse_postfix(token_stream, &lhs)? {
            lhs = new_lhs;
            continue;
        }

        if let Some(new_lhs) = parse_infix(token_stream, &lhs, min_bp)? {
            lhs = new_lhs;
            continue;
        }

        break;
    }

    Ok(lhs)
}

fn infix_binding_power(tok: &Token) -> Option<(u8, u8, BinaryOp)> {
    match tok {
        Token::Plus => Some((10, 11, BinaryOp::Add)),
        Token::Minus => Some((10, 11, BinaryOp::Sub)),
        Token::Star => Some((20, 21, BinaryOp::Mul)),
        Token::Slash => Some((20, 21, BinaryOp::Div)),

        Token::And => Some((3, 4, BinaryOp::And)),
        Token::Or => Some((2, 3, BinaryOp::Or)),

        Token::Lt => Some((5, 6, BinaryOp::Lt)),
        Token::Gt => Some((5, 6, BinaryOp::Gt)),
        Token::LEq => Some((5, 6, BinaryOp::LEq)),
        Token::GEq => Some((5, 6, BinaryOp::GEq)),
        Token::EqEq => Some((4, 5, BinaryOp::Eq)),
        Token::NotEq => Some((4, 5, BinaryOp::NEq)),
        Token::Eq => Some((1, 0, BinaryOp::Assign)),

        _ => None,
    }
}

fn parse_prefix(token_stream: &mut TokenStream) -> Result<Node<Expression>, ParsingError> {
    match token_stream.next() {
        Some((Token::Minus, op_span)) => parse_prefix_unary(token_stream, op_span, UnaryOp::Neg),
        Some((Token::Bang, op_span)) => parse_prefix_unary(token_stream, op_span, UnaryOp::Not),
        Some((Token::Integer(i), span)) => {
            Ok(create_node(token_stream, span, Expression::Integer(i)))
        }
        Some((Token::Float(f), span)) => Ok(create_node(token_stream, span, Expression::Float(f))),
        Some((Token::DoubleQuotedString(s), span)) => {
            Ok(create_node(token_stream, span, Expression::String(s)))
        }
        Some((Token::Identifier(i), span)) => {
            Ok(create_node(token_stream, span, Expression::Identifier(i)))
        }
        Some((Token::LBracket, start_span)) => parse_array(token_stream, start_span),
        Some((Token::If, _span)) => parse_conditional(token_stream),
        Some((Token::LParen, start_span)) => {
            /*
                Paren could mean either a grouping, a tuple, or a function
            */

            let expr = parse_paren_expr(token_stream, start_span.clone())?;

            if matches!(token_stream.peek(), Some((Token::RArrow, _))) {
                parse_lambda_after_paren(token_stream, expr, start_span)
            } else {
                Ok(expr)
            }
        }

        Some((_, span)) => Err(ParsingError::new(
            "Unexpected token {:?} in prefix",
            Some(span),
        )),
        None => Err(ParsingError::new("Unexpected end of input", None)),
    }
}

fn parse_prefix_unary(
    token_stream: &mut TokenStream,
    op_span: Span,
    op: UnaryOp,
) -> Result<Node<Expression>, ParsingError> {
    // binding power for prefix operators
    const PREFIX_BP: u8 = 30;

    let rhs = parse_expression(token_stream, PREFIX_BP)?;

    let span = span_from_to(op_span, rhs.span.clone());

    Ok(create_node(
        token_stream,
        span,
        Expression::Unary {
            op,
            expression: Box::new(rhs),
        },
    ))
}

fn parse_postfix(
    token_stream: &mut TokenStream,
    lhs: &Node<Expression>,
) -> Result<Option<Node<Expression>>, ParsingError> {
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

            let span = span_from_to(lhs.span.clone(), end_span);

            Ok(Some(create_node(
                token_stream,
                span,
                Expression::Index {
                    target: Box::new(lhs.clone()),
                    index: Box::new(index_expr),
                },
            )))
        }

        Some((Token::LParen, _)) => {
            // Function call
            token_stream.next(); // consume '('
            let mut args = Vec::new();

            if !matches!(token_stream.peek(), Some((Token::RParen, _))) {
                loop {
                    args.push(parse_expression(token_stream, 0)?);
                    match token_stream.peek() {
                        Some((Token::Comma, _)) => {
                            token_stream.next();
                        }
                        Some((Token::RParen, _)) => break,
                        Some((_, span)) => {
                            return Err(ParsingError::new("Expected ',' or ')'", Some(span)))
                        }
                        None => {
                            return Err(ParsingError::new("Unexpected EOF in function call", None))
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

            Ok(Some(create_node(
                token_stream,
                span,
                Expression::FunctionCall {
                    callee: Box::new(lhs.clone()),
                    arguments: args,
                },
            )))
        }

        _ => Ok(None), // No postfix operator found
    }
}

fn parse_infix(
    token_stream: &mut TokenStream,
    lhs: &Node<Expression>,
    min_bp: u8,
) -> Result<Option<Node<Expression>>, ParsingError> {
    let (left_bp, right_bp, bin_op) = match token_stream.peek() {
        Some((tok, _)) => match infix_binding_power(&tok) {
            Some(info) => info,
            None => return Ok(None),
        },
        None => return Ok(None),
    };

    if left_bp < min_bp {
        return Ok(None);
    }

    token_stream.next(); // consume operator
    let rhs = parse_expression(token_stream, right_bp)?;
    let span = span_from_to_node(&lhs, &rhs);

    let new_lhs = match bin_op {
        BinaryOp::Assign => {
            let pattern = lhs.clone().into_pattern()?;
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
                lhs: Box::new(lhs.clone()),
                op: bin_op,
                rhs: Box::new(rhs),
            },
        ),
    };

    Ok(Some(new_lhs))
}

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
            else_branch,
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

fn parse_lambda_after_paren(
    token_stream: &mut TokenStream,
    params_expr: Node<Expression>,
    start_span: Span,
) -> Result<Node<Expression>, ParsingError> {
    token_stream.next(); // consume =>

    let return_type = parse_expression(token_stream, 0)?;
    let body = parse_block(token_stream)?;

    // Convert params_expr into patterns
    let params = match params_expr.kind {
        Expression::Tuple(elements) => elements
            .into_iter()
            .map(Node::into_pattern)
            .collect::<Result<Vec<_>, _>>()?,
        Expression::Identifier(_) => {
            vec![params_expr.into_pattern()?]
        }
        _ => {
            return Err(ParsingError::new(
                "Invalid lambda parameter list",
                Some(params_expr.span),
            ));
        }
    };

    let span = span_from_to(start_span, body.span);

    Ok(create_node(
        token_stream,
        span,
        Expression::Function {
            signature: FunctionSignature {
                args: create_node(token_stream, span, params),
                return_type: Box::new(return_type),
            },
            body: Box::new(body),
        },
    ))
}

fn parse_paren_expr(
    token_stream: &mut TokenStream,
    start_span: Span,
) -> Result<Node<Expression>, ParsingError> {
    let mut elements = Vec::new();

    if matches!(token_stream.peek(), Some((Token::RParen, _))) {
        let (_, end_span) = token_stream.next().unwrap();
        let span = span_from_to(start_span, end_span);
        return Ok(create_node(token_stream, span, Expression::Tuple(vec![])));
    }

    elements.push(parse_expression(token_stream, 0)?);

    while matches!(token_stream.peek(), Some((Token::Comma, _))) {
        token_stream.next();
        elements.push(parse_expression(token_stream, 0)?);
    }

    let end_span = match token_stream.next() {
        Some((Token::RParen, span)) => span,
        _ => return Err(ParsingError::new("Expected ')'", None)),
    };

    let span = span_from_to(start_span, end_span);

    if elements.len() == 1 {
        Ok(elements.into_iter().next().unwrap())
    } else {
        Ok(create_node(token_stream, span, Expression::Tuple(elements)))
    }
}
