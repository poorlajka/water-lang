
use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(error = LexingError)]
pub enum Token {

    // Keywords
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

    #[token("print")]
    Print,

    #[token("\n")]
    #[token(";")]
    Newline,
    
    #[token("==")]
    EqEq,
    
    #[token("!=")]
    NotEq,

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

    #[token("..")]
    Range,

    #[token("<")]
    Lt,

    #[token(">")]
    Gt,


    // Whitespace (all spaces/tabs)
    #[regex(r"[ \t]+", logos::skip)]
    Whitespace,

    Indent,
    Dedent,

    Error,

    Eof,
}

#[derive(Debug, PartialEq, Clone, Default)]
pub enum LexingError {
    NumberParseError,
    #[default]
    Other
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
        } else {
            result.push(c);
        }
    }

    result
}