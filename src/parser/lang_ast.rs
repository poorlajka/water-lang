use logos::Span;

#[derive(Debug, Clone)]
pub struct Spanned<T> {
    pub node: T,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Program {
    pub modules: Vec<Module>
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
    Assignment(Assignment),
    Function(Function),
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
        else_branch: Box<Option<Expression>>,
    },
    Block {
        statements: Vec<Statement>,
        final_expr: Option<Box<Expression>>,
    }
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
}

#[derive(Debug, Clone)]
pub struct Function {
    pub signature: FunctionSignature,
    pub body: Vec<Statement>,
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

#[derive(Debug, Clone)]
pub struct Assignment {
    pub lhs: String,
    pub rhs: Expression,
}

#[derive(Debug, Clone)]
pub struct FunctionCall {
}







