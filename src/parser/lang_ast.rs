use logos::Span;

use crate::parser::lang_parser::{ParsingError, TokenStream};

#[derive(Debug, Clone)]
pub struct Node<T> {
    pub id: usize,
    pub span: Span,
    pub kind: T,
}

impl<T> Node<T> {
    pub fn new(id: usize, span: Span, kind: T) -> Self {
        Self { id, span, kind }
    }
}

pub fn span_from_to(start: Span, end: Span) -> Span {
    start.start..end.end
}

pub fn span_from_to_node<T>(start: &Node<T>, end: &Node<T>) -> Span {
    start.span.start..end.span.end
}

pub type Expr = Node<Expression>;
pub type Stmt = Node<Statement>;
pub type Pat = Node<Pattern>;

#[derive(Debug, Clone)]
pub struct Module {
    pub name: String,
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub enum Statement {
    Expression(Expr),
    //Import(Import),
    //Return(Return),
    //ForLoop(ForLoop),
}

#[derive(Debug, Clone)]
pub enum Expression {
    Integer(i64),
    Float(f64),
    String(String),
    Variable(String),
    Tuple(Vec<Expr>),
    Array(Vec<Expr>),

    Assignment {
        lhs: Pat,
        rhs: Box<Expr>,
    },

    Function {
        signature: FunctionSignature,
        body: Vec<Statement>,
    },

    Unary {
        op: UnaryOp,
        expression: Box<Expr>,
    },

    Binary {
        lhs: Box<Expr>,
        op: BinaryOp,
        rhs: Box<Expr>,
    },

    Conditional {
        condition: Box<Expr>,
        then_branch: Box<Expr>,
        else_branch: Option<Box<Expr>>,
    },

    Block {
        statements: Vec<Statement>,
        final_expr: Option<Box<Expr>>,
    },

    Index {
        target: Box<Expr>,
        index: Box<Expr>,
    },

    FunctionCall {
        callee: Box<Expr>,
        arguments: Vec<Expr>,
    },
}

impl Node<Expression> {
    pub fn into_pattern(self) -> Result<Node<Pattern>, ParsingError> {
        let Node { id, span, kind } = self;

        let pattern_kind = match kind {
            Expression::Variable(name) => Pattern::Identifier(name),

            Expression::Tuple(elements) => Pattern::Tuple(
                elements
                    .into_iter()
                    .map(|e| e.into_pattern())
                    .collect::<Result<Vec<_>, _>>()?,
            ),

            _ => {
                return Err(ParsingError::new("Invalid assignment target", Some(span)));
            }
        };

        Ok(Node::new(id, span, pattern_kind))
    }
}

#[derive(Debug, Clone)]
pub enum Pattern {
    Identifier(String),
    Tuple(Vec<Pat>),
    // future:
    // Wildcard,
    // StructPattern { ... },
    // List(Vec<Pattern>),
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Neg,
    Not,
}

#[derive(Debug, Clone)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Lt,
    Gt,
    Eq,
    Assign,
}

#[derive(Debug, Clone)]
pub struct FunctionSignature {
    pub name: String,
    pub args: Vec<Parameter>,
    pub output: DataType,
}

#[derive(Debug, Clone)]
pub struct Parameter {
    pub binding: Pat,
    pub ptype: DataType,
}

#[derive(Debug, Clone)]
pub enum DataType {
    Identifier,
    Implicit,
}
