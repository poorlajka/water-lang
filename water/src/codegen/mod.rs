use crate::ast::{Expression, Node, Pattern, FunctionSignature, Module};
use crate::bytecode::Instruction;

use std::collections::HashMap;

pub struct ByteCode {
    pub functions: Vec<CompiledFunction>,
}

pub struct CompiledFunction {
    pub code_block: Vec<Instruction>,
    pub symbol_table: SymbolTable,
}

struct SymbolTable {
    symbols: HashMap<String, usize>,
    reg_top: usize,
}

impl SymbolTable {
    fn new () -> Self {
        Self {
            symbols: HashMap::new(),
            reg_top: 2,
        }
    }

    fn register_variable (&mut self, name: &str) -> usize {
        self.symbols.insert(name.to_string(), self.reg_top);
        self.reg_top += 1;
        self.reg_top - 1 
    }

    fn get_variable (&self, name: &str) -> Option<&usize> {
        self.symbols.get(name)
    }
    
}

pub fn compile_module (module: &Module) -> ByteCode {
    let mut symbol_table = SymbolTable::new();
    let mut bytecode = Vec::new();

    for statement in &module.statements {
        bytecode.append(&mut compile_statement(statement, &mut symbol_table));
    }

    let compiler_artifacts = CompilerArtifacts {
        bytecode,
        errors: Vec::new(),
    };

    compiler_artifacts
}

fn compile_function (signature: &FunctionSignature, body: &Expression, symbol_table: &mut SymbolTable) 
-> Vec<Instruction> {

    let mut bytecode = Vec::new();

    for arg in &signature.args {
        match &arg.kind {
            Pattern::Identifier(ident) => {

            }
            _ => {

            }
        }
    }

    bytecode.append(&mut compile_expression(body, symbol_table));

    bytecode
}

fn compile_statement (statement: &Statement, symbol_table: &mut SymbolTable) 
-> Vec<Instruction> {
    let mut bytecode = Vec::new();

    use Statement as Statement;
    match statement {
        Statement::Expression(expression) => {
            bytecode.append(&mut compile_expression(&expression.kind, symbol_table));

        },
        Statement::Return(return_statement) => {

        },
    }

    bytecode

}

fn compile_assignment (lhs: &Pattern, rhs: &Expression, symbol_table: &mut SymbolTable) 
-> Vec<Instruction> {
    let mut bytecode = Vec::new();
    bytecode.append(&mut compile_expression(&rhs, symbol_table));
    match &lhs{
        Pattern::Identifier(ident) => {
            bytecode.push(
                Instruction::Mov(
                    symbol_table.register_variable(ident), 
                    0,
                ));
        }
        _ => {

        }
    }
    bytecode
}

fn compile_expression (expression: &Expression, symbol_table: &mut SymbolTable) 
-> (Vec<Instruction>, usize) {

    match expression {
        Expression::Function{signature, body} => {
            compile_function(&signature, &body.kind, symbol_table)
        },
        Expression::Assignment{lhs, rhs} => {
            compile_assignment(&lhs.kind, &rhs.kind, symbol_table)
        },
        Expression::Unary { op, expression } => {
            (Vec::new(), 0)
        },
        Expression::Binary { lhs, op, rhs } => {
            compile_binary_expression(&lhs.kind, op, &rhs.kind, symbol_table)
        }
        Expression::FunctionCall { callee, arguments } => {
            compile_function_call(&callee.kind, arguments, symbol_table)
        }
        _ => {
            (Vec::new(), 0)
        }
    }
}

fn compile_binary_expression (lhs: &Expression, op: &BinaryOp, rhs: &Expression, symbol_table: &mut SymbolTable) 
-> Vec<Instruction> {
    let mut bytecode = Vec::new();

    bytecode.append(&mut compile_expression(&lhs, symbol_table));

    bytecode.append(&mut compile_expression(&rhs, symbol_table));

    bytecode.push(Instruction::Mov(1, 0));
    
    use BinaryOp as BinaryOp;
    
    bytecode.push(match op {
        BinaryOp::Add => {
            Instruction::Add(0, 1)
        }
        BinaryOp::Sub => {
            Instruction::Sub(0, 1)
        }
        BinaryOp::Mul => {
            Instruction::Mul(0, 1)
        }
        BinaryOp::Div => {
            Instruction::Div(0, 1)
        }
        BinaryOp::Mod => {
            Instruction::Mod(0, 1)
        }
        _ => {
            Instruction::Add(0, 1)
        }
    });

    bytecode
}

fn compile_function_call (callee: &Expression, arguments: &Vec<Node<Expression>>, symbol_table: &mut SymbolTable) 
-> Vec<Instruction> {
    let mut bytecode = Vec::new();
    for (i, arg) in arguments.iter().enumerate() {
        bytecode.append(&mut compile_expression(&arg.kind, symbol_table));
        bytecode.push(Instruction::Mov(i+1, 0));
    }
    bytecode.append(&mut compile_expression(callee, symbol_table));
    bytecode.push(Instruction::Call(0));
    bytecode
}