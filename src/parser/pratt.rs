use crate::lexer::lang_token::{self, Token};
use logos::Span;
use crate::parser::lang_ast;
use crate::parser::lang_parser::{
    TokenStream,
    ParsingError,
    parse_statement,
};

fn parse_block(token_stream: &mut TokenStream)
    -> Result<lang_ast::Expression, ParsingError>
{
    let mut statements = Vec::new();

    if !matches!(token_stream.next(1), Some((Token::Indent, _))) {
        return Err(ParsingError {
            message: "Expected block".to_string(),
        });
    }

    loop {
        token_stream.skip_newlines();
        match token_stream.peek(1) {
            Some((Token::Dedent, _)) | None => {
                token_stream.next(1); // consume }
                break;
            }
            Some((t, span)) => {
                let stmt = parse_statement(token_stream)?;
                statements.push(stmt);
            }
        }
    }

    let final_expr = match statements.last() {
        Some(lang_ast::Statement::Expression(expression)) => {
            Some(Box::new(expression.clone()))
        }
        _ => None,
    };

    Ok(lang_ast::Expression::Block {
        statements,
        final_expr,
    })
}

fn parse_conditional(token_stream: &mut TokenStream) -> Result<lang_ast::Expression, ParsingError>{
    let condition = parse_expression(token_stream, 0)?;

    token_stream.skip_newlines();
    let then_branch = match token_stream.peek(1) {
        Some((Token::Then, span)) => { 
            parse_expression(token_stream, 0)?
        }
        _ => {
            parse_block(token_stream)?
        }
    };

    token_stream.skip_newlines();
    let else_branch = match token_stream.peek(1) {
        Some((Token::Else, span)) => {
            token_stream.next(1);
            token_stream.skip_newlines();
            if let Ok(block) = parse_block(token_stream) {
                Some(block)
            }
            else if let Ok(expr) = parse_expression(token_stream, 0) {
                Some(expr)
            }
            else {
                None
            }
        }
        _ => None,
    };

    Ok(lang_ast::Expression::Conditional { 
        condition: Box::new(condition), 
        then_branch: Box::new(then_branch), 
        else_branch: Box::new(else_branch),
    })
}

fn parse_prefix(token_stream: &mut TokenStream) -> Result<lang_ast::Expression, ParsingError> {

    token_stream.skip_newlines();
    match token_stream.next(1) {
        Some((Token::If, _span)) => {
            parse_conditional(token_stream)
        }
        Some((Token::Integer(i), _span)) => {
            Ok(lang_ast::Expression::Integer(i))
        }
        Some((Token::Float(f), _span)) => {
            Ok(lang_ast::Expression::Float(f))
        }
        Some((Token::DoubleQuotedString(s), _span)) => {
            Ok(lang_ast::Expression::String(s))
        }
        Some((Token::Identifier(i), _span)) => {
            Ok(lang_ast::Expression::Variable(i))
        }

        Some((Token::LParen, _)) => {
            let expr = parse_expression(token_stream, 0)?;

            match token_stream.next(1) {
                Some((Token::RParen, _)) => Ok(expr),

                Some((_token, span)) => {
                    Err(ParsingError {message: "Expected ')'".to_string()})
                }

                None => {
                    Err(ParsingError {message: "Unexpected end of input".to_string()})
                }
            }
        }

        Some((_token, span)) => {
            Err(ParsingError {message: "Unexpected token in prefix".to_string()})
        }
        None => {
            Err(ParsingError {message: "Unexpected end of input".to_string()})
        }
    }
}

pub fn parse_expression(token_stream: &mut TokenStream, min_bp: u8) -> Result<lang_ast::Expression, ParsingError> {

    token_stream.skip_newlines();

    let mut lhs = parse_prefix(token_stream)?;

    loop {
        token_stream.skip_newlines();
        let op = match token_stream.peek(1) {
            Some((token, _)) => token.clone(),
            None => break,
        };

        let (left_bp, right_bp, bin_op) =
            match infix_binding_power(&op) {
                Some(info) => info,
                None => break,
            };

        if left_bp < min_bp {
            break;
        }

        token_stream.next(1);

        let rhs = parse_expression(token_stream, right_bp)?;

        lhs = lang_ast::Expression::Binary {
            lhs: Box::new(lhs),
            op: bin_op,
            rhs: Box::new(rhs),
        };
    }

    Ok(lhs)
}

fn infix_binding_power(tok: &Token) -> Option<(u8, u8, lang_ast::BinaryOp)> {
    match tok {
        Token::Plus  => Some((10, 11, lang_ast::BinaryOp::Add)),
        Token::Minus => Some((10, 11, lang_ast::BinaryOp::Sub)),
        Token::Star  => Some((20, 21, lang_ast::BinaryOp::Mul)),
        Token::Slash => Some((20, 21, lang_ast::BinaryOp::Div)),

        Token::Lt    => Some((5, 6, lang_ast::BinaryOp::Lt)),
        Token::Gt    => Some((5, 6, lang_ast::BinaryOp::Gt)),
        Token::EqEq  => Some((4, 5, lang_ast::BinaryOp::Eq)),

        _ => None,
    }
}



