pub mod display;

use logos::Span;

use crate::parser::ParsingError;

#[derive(Debug, Clone)]
pub struct Node<T> {
    pub id: usize,
    pub span: Span,
    pub kind: T,
}

impl<T: PartialEq> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
    }
}

impl<T> Node<T> {
    pub fn new(id: usize, span: Span, kind: T) -> Self {
        Self { id, span, kind }
    }
}

pub type ExprNode = Node<Expression>;
pub type PatNode = Node<Pattern>;
pub type TypeNode = Node<Type>;

#[derive(Debug, Clone, PartialEq)]
pub struct Module {
    pub name: String,
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Expression(ExprNode),
    //Import(Import),
    Return(Return),
    //ForLoop(ForLoop),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Return {}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Integer(i64),
    Float(f64),
    String(String),
    Identifier(String),
    Tuple(Vec<ExprNode>),
    Array(Vec<ExprNode>),

    Assignment {
        lhs: PatNode,
        rhs: Box<ExprNode>,
    },

    Function {
        signature: FunctionSignature,
        body: Box<ExprNode>,
    },

    Unary {
        op: UnaryOp,
        expression: Box<ExprNode>,
    },

    Binary {
        lhs: Box<ExprNode>,
        op: BinaryOp,
        rhs: Box<ExprNode>,
    },

    Conditional {
        condition: Box<ExprNode>,
        then_branch: Box<ExprNode>,
        else_branch: Option<Box<ExprNode>>,
    },

    Block {
        statements: Vec<Statement>,
        final_expr: Option<Box<ExprNode>>,
    },

    Index {
        target: Box<ExprNode>,
        index: Box<ExprNode>,
    },

    FunctionCall {
        callee: Box<ExprNode>,
        arguments: Vec<ExprNode>,
    },

    Type(Type),

    Typed {
        expr: Box<Node<Expression>>,
        ty: Box<Node<Type>>,
    },
}

impl Node<Expression> {
    pub fn into_pattern(self) -> Result<PatNode, ParsingError> {
        let Node { id, span, kind } = self;

        let pattern_kind = match kind {
            Expression::Identifier(name) => Pattern::Identifier(name),

            Expression::Tuple(elements) => Pattern::Tuple(
                elements
                    .into_iter()
                    .map(|e| e.into_pattern())
                    .collect::<Result<Vec<_>, _>>()?,
            ),

            _ => {
                return Err(ParsingError::new(
                    "Invalid assignment target",
                    Some(span),
                ));
            }
        };

        Ok(Node::new(id, span, pattern_kind))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    Identifier(String),
    Tuple(Vec<PatNode>),

    Typed { pattern: Box<PatNode>, ty: TypeNode }, // future:
                                                   // Wildcard,
                                                   // StructPattern { ... },
                                                   // List(Vec<Pattern>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Neg,
    Not,
    Plus,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    Lt,
    Gt,
    GEq,
    LEq,
    Eq,
    NEq,
    And,
    Or,
    Assign,
    TypeAnnotation,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionSignature {
    pub args: Vec<PatNode>,
    pub return_type: TypeNode,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Named(String),

    Tuple(Vec<TypeNode>),

    Array(Box<TypeNode>),

    Function {
        params: Vec<TypeNode>,
        return_type: Box<TypeNode>,
    },

    Dynamic,
}
