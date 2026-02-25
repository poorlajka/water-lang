use logos::Span;

#[derive(Debug, Clone)]
pub struct Spanned<T> {
    pub node: T,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Program {
    pub modules: Vec<Module>,
}

#[derive(Debug, Clone)]
pub struct Module {
    pub name: String,
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub enum Statement {
    Expression(Expression),
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
    Tuple(Vec<Expression>),
    Assignment {
        lhs: Pattern,
        rhs: Box<Expression>,
    },
    Function {
        signature: FunctionSignature,
        body: Vec<Statement>,
    },
    FunctionCall {},
    Unary {
        op: UnaryOp,
        expression: Box<Expression>,
    },
    Binary {
        lhs: Box<Expression>,
        op: BinaryOp,
        rhs: Box<Expression>,
    },
    Conditional {
        condition: Box<Expression>,
        then_branch: Box<Expression>,
        else_branch: Box<Option<Expression>>,
    },
    Block {
        statements: Vec<Statement>,
        final_expr: Option<Box<Expression>>,
    },
    Array(Vec<Expression>),
    Index {
        target: Box<Expression>,
        index: Box<Expression>,
    },
}

#[derive(Debug, Clone)]
pub enum Pattern {
    Identifier(String),
    Tuple(Vec<Pattern>),
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
    pub name: String,
    pub ptype: DataType,
}

#[derive(Debug, Clone)]
pub enum DataType {
    Enum,
    Struct,
    Implicit,
}
