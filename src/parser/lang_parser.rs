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
fn parse_statement(token_stream: &mut TokenStream) -> Result<lang_ast::Statement, ParsingError> {

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
            token_stream.pos = save_pos;
        }
    }

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










































    /*
        <loop> ::= <for_loop> | <while_loop>
    */

    /*
        <branch> ::= <expression> ":" <indent> (<statement>)* <dedent>
        <conditional> ::= "if" <branch> ( "else" "if" <branch> )* [ "else" <branch> ]
    */

    /*
        <assignment> ::= [ "mut" ] IDENTIFIER ( "," [ "mut" ] IDENTIFIER )* "=" <expression>
    */

            /*
pub fn parser<'tokens, 'src: 'tokens, I>() 
    -> impl Parser<'tokens, I,lang_ast::Module, 
    extra::Err<Rich<'tokens, (Token<'src>, SimpleSpan)>>
>
where
    I: ValueInput<
        'tokens,
        Token = (Token<'src>, SimpleSpan),
        Span = SimpleSpan,
    >,
{
    recursive(|module| {

        recursive(|statement| {



            // still incomplete, as requested
            choice((
                function.map(lang_ast::Statement::Function),
                // assignment,
                // expression,
            ))
        })
        .repeated()
        .map(lang_ast::Module {
            name: "main".to_string(),
            statements: vec![],
        })
    })
}
                */



    /*
        <param> ::= IDENTIFIER [ ":" IDENTIFIER ]
    */

    /*
        <param_list> ::= "(" [ <param> ( "," <param> )* [ "," ] ] ")"
    let param_list = just(Token::LParen)
        .ignore_then(
            param
                .separated_by(just(Token::Comma))
                .allow_trailing()
                .or_not()
        )
        .then_ignore(just(Token::RParen))
        .map(|params| params.unwrap_or_default());
    */


        /*
            <function_body> ::= [ IDENTIFIER ] INDENT <statement>* DEDENT
        let function_body = ident
            .or_not()
            .then_ignore(just(Token::Indent))
            .then(statement.clone().repeated())
            .then_ignore(just(Token::Dedent))
            .map(|(ftype, statements)| {
                (
                    ftype.unwrap_or(lang_ast::DataType::Implicit),
                    statements,
                )
            });
        */

        /*
            <function> ::= IDENTIFIER "=" <param_list> "=>" <function_body>
        let function = just(Token::Identifier)
            .then_ignore(just(Token::Assignment))
            .then(param_list)
            .then_ignore(just(Token::RArrow))
            .then(function_body)
            .map(|((name, params), (out_type, body))| {
                lang_ast::Function {
                    name,
                    params,
                    out_type,
                    body,
                }
            });
        */



