use logos::Span;

#[derive(Debug)]
pub struct Spanned<T> {
    pub node: T,
    pub span: Span,
}

#[derive(Debug)]
pub struct Program {
    pub modules: Vec<Module>
}

#[derive(Debug)]
pub struct Module {
    pub name: String,
    pub statements: Vec<Statement>,
}


#[derive(Debug)]
pub enum Statement {
    Expression(Expression),
    Assignment(Assignment),
    Function(Function),
}

#[derive(Debug)]
pub enum Expression {
    Integer(i64),
    Float(f64),
    String(String),
    Variable(String),
    FunctionCall(FunctionCall),
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
        else_branch: Box<Expression>,
    },
    Block {
        statements: Vec<Statement>,
        final_expr: Option<Box<Expression>>,
    }
}

#[derive(Debug)]
pub enum UnaryOp {
    Neg,
    Not,
}

#[derive(Debug)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Lt,
    Gt,
    Eq,
}

#[derive(Debug)]
pub struct Function {
    pub signature: FunctionSignature,
    pub body: Vec<Statement>,
}

#[derive(Debug)]
pub struct FunctionSignature {
    pub name: String,
    pub args: Vec<Parameter>,
    pub output: DataType, 
}

#[derive(Debug)]
pub struct Parameter {
    pub name: String,
    pub ptype: DataType,
}

#[derive(Debug)]
pub enum DataType {
    Enum,
    Struct,
    Implicit,
}

#[derive(Debug)]
pub struct Assignment {
    pub lhs: String,
    pub rhs: Expression,
}

#[derive(Debug)]
pub struct FunctionCall {
}







