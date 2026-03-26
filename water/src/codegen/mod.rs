use crate::ast;
use crate::bytecode;

use std::collections::HashMap;

pub struct CompilerArtifacts {
    pub bytecode: Vec<bytecode::Instruction>,
    pub errors: Vec<i32>,
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

pub fn compile (module: &ast::Module) -> CompilerArtifacts {
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

pub fn compile_function (function: &ast::Expression, symbol_table: &mut SymbolTable) 
-> Vec<bytecode::Instruction> {

    let mut bytecode = Vec::new();

    if let ast::Expression::Function { signature, body } = function {
        bytecode.append(&mut compile_expression(&body.kind, symbol_table));
    } 

    bytecode
}

pub fn compile_statement (statement: &ast::Statement, symbol_table: &mut SymbolTable) 
-> Vec<bytecode::Instruction> {
    let mut bytecode = Vec::new();

    use ast::Statement as Statement;
    match statement {
        Statement::Expression(expression) => {
            bytecode.append(&mut compile_expression(&expression.kind, symbol_table));

        },
        Statement::Return(return_statement) => {

        },
    }

    bytecode

}

pub fn compile_assignment (assignment: &ast::Expression, symbol_table: &mut SymbolTable) 
-> Vec<bytecode::Instruction> {
    let mut bytecode = Vec::new();
    bytecode.append(&mut compile_expression(&assignment.rhs, symbol_table));
    bytecode.push(
        bytecode::Instruction::Mov(
            symbol_table.register_variable(&assignment.lhs[0].name), 
            0,
        ));

    bytecode
}

pub fn compile_expression (expression: &ast::Expression, symbol_table: &mut SymbolTable) 
-> Vec<bytecode::Instruction> {
    let mut bytecode = Vec::new();

    use ast::Expression as Expression;
    match expression {
        Expression::Constant(constant) => {
            match constant {
                ast::Constant::Integer(integer) => {
                    bytecode.push(bytecode::Instruction::MovConst(0, *integer));
                }
            }
        },
        Expression::FunctionCall => {

        },
        Expression::BinaryExpression(binary_expression) => {
            bytecode.append(&mut compile_binary_expression(binary_expression, symbol_table));
        },
        _ => {
            
        }
    }

    bytecode
}

pub fn compile_binary_expression (binary_expression: &ast::BinaryExpression, symbol_table: &mut SymbolTable) 
-> Vec<bytecode::Instruction> {
    let mut bytecode = Vec::new();

    bytecode.append(&mut compile_expression(&binary_expression.rhs, symbol_table));

    bytecode.push(bytecode::Instruction::Mov(1, 0));

    bytecode.append(&mut compile_expression(&binary_expression.lhs, symbol_table));
    
    use ast::BinaryOperator as BinaryOperator;
    
    bytecode.push(match binary_expression.operator {
        BinaryOperator::Addition => {
            bytecode::Instruction::Add(0, 1)
        }
        BinaryOperator::Subtraction => {
            bytecode::Instruction::Sub(0, 1)
        }
        BinaryOperator::Multiplication => {
            bytecode::Instruction::Mul(0, 1)
        }
        BinaryOperator::Division => {
            bytecode::Instruction::Div(0, 1)
        }
        BinaryOperator::Mod => {
            bytecode::Instruction::Mod(0, 1)
        }
    });

    bytecode
}

pub fn compile_print (symbol: &ast::Symbol, symbol_table: &mut SymbolTable) 
-> Vec<bytecode::Instruction> {
    let mut bytecode = Vec::new();
    if let Some(register_id) = symbol_table.get_variable(&symbol.name) {
        bytecode.push(bytecode::Instruction::Print(*register_id));
    }
    
    bytecode
}