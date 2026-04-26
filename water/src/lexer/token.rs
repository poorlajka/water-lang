use crate::lexer::LexingError;
use logos::Logos;

/*
   Reserved identifiers (keywords):
       Loops:
           for
           in
           while
           break
           continue

       Conditionals:
           if
           then
           else
           match

       Logical operators:
           not
           and
           or

       Boolean values:
           true
           false

       Imports:
           import
           from
           as

       Functions:
           return
           defer

       Modifiers:
           pub
           mut
           with


   Reserved tokens:
       Operators:
           Logical operators:
               ==
               !=
               ||
               &&
               >
               <
               >=
               <=
               !
           Arithmetic operators:
               +
               -
               *
               /
               %
               **
           Assignment operators:
               =
               +=
               -=
               *=
               /=
               %=
               **=
       Syntax markers:
       $

       "
       =>
       {
       }
       |
       &
       ^
*/

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(error = LexingError)]
pub enum Token {
    // Keywords
    #[token("and")]
    #[token("&&")]
    And,

    #[token("or")]
    #[token("||")]
    Or,

    #[token("for")]
    For,

    #[token("in")]
    In,

    #[token("if")]
    If,

    #[token("then")]
    Then,

    #[token("else")]
    Else,

    #[token("return")]
    Return,

    #[token("\r\n")]
    #[token("\n")]
    #[token(";")]
    Newline,

    #[token("==")]
    EqEq,

    #[token("!=")]
    NotEq,

    #[token("!")]
    Bang,

    #[token("=>")]
    RArrow,

    #[token("=")]
    Eq,

    #[regex(r#""([^"\\]|\\.)*""#, parse_string)]
    DoubleQuotedString(String),

    #[regex(r#"'([^'\\]|\\.)*'"#, parse_string)]
    SingleQuotedString(String),

    // Identifiers
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", |lex| String::from(lex.slice()))]
    Identifier(String),

    #[regex("-?[0-9]+", |lex| lex.slice().parse())]
    Integer(i64),

    #[regex("-?[0-9]+\\.[0-9]+", |lex| lex.slice().parse())]
    Float(f64),

    // Punctuation
    #[token(":")]
    Colon,

    #[token(",")]
    Comma,

    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,

    #[token("(")]
    LParen,

    #[token(")")]
    RParen,

    #[token("[")]
    LBracket,

    #[token("]")]
    RBracket,

    #[token("/")]
    Slash,

    #[token("%")]
    Percent,

    #[token("..")]
    Range,

    #[token("<=")]
    LEq,

    #[token(">=")]
    GEq,

    #[token("<")]
    Lt,

    #[token(">")]
    Gt,

    #[token("$")]
    DollarSign,

    // Whitespace (all spaces/tabs)
    #[regex(r"[ \t]+", logos::skip)]
    Whitespace,

    Indent,
    Dedent,

    Error,

    Eof,
}

fn parse_string(lex: &mut logos::Lexer<Token>) -> String {
    let slice = lex.slice();
    let inner = &slice[1..slice.len() - 1];

    let mut result = String::new();
    let mut chars = inner.chars();

    while let Some(c) = chars.next() {
        if c == '\\' {
            if let Some(next) = chars.next() {
                match next {
                    'n' => result.push('\n'),
                    't' => result.push('\t'),
                    '\\' => result.push('\\'),
                    '"' => result.push('"'),
                    '\'' => result.push('\''),
                    other => result.push(other),
                }
            }
        }
        else {
            result.push(c);
        }
    }

    result
}
