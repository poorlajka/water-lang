use crate::ast::{Expression, Node, Pattern, FunctionSignature, Module, Statement, BinaryOp, UnaryOp};
use crate::bytecode::Instruction;

use std::collections::HashMap;
use std::env::VarsOs;

pub struct Compiler {
    pub main: Vec<Instruction>,
    pub functions: Vec<CompiledFunction>,
}

pub struct CompiledFunction {
    pub code_block: Vec<Instruction>,
}

struct SymbolTable {
    symbols: HashMap<String, usize>,
    reg_top: usize,
}

impl SymbolTable {
    fn new () -> Self {
        Self {
            symbols: HashMap::new(),
            reg_top: 10,
        }
    }

    fn register_variable (&mut self, name: &str) -> usize {
        self.symbols.insert(name.to_string(), self.reg_top);
        self.reg_top += 1;
        self.reg_top - 1 
    }

    fn register_intermediate (&mut self) -> usize {
        self.reg_top += 1;
        self.reg_top - 1 
    }

    fn get_variable (&self, name: &str) -> Option<&usize> {
        self.symbols.get(name)
    }
    
}

pub fn compile_module (module: &Module) -> Compiler {
    let mut symbol_table = SymbolTable::new();
    let mut compiler = Compiler {
        main: Vec::new(),
        functions: vec![CompiledFunction{code_block: Vec::new()}],
    };

    for statement in &module.statements {
        let mut compiled_statement = compiler.compile_statement(statement, &mut symbol_table);
        compiler.main.append(&mut compiled_statement);
    }

    compiler
}

impl Compiler {

    fn compile_function (&mut self, signature: &FunctionSignature, body: &Expression, symbol_table: &mut SymbolTable) 
    -> (Vec<Instruction>, usize) {

        let mut bytecode = Vec::new();

        let function_index = self.functions.len();
        for (i, arg) in signature.args.iter().enumerate() {
            match &arg.kind {
                Pattern::Identifier(ident) => {
                    let reg = symbol_table.register_variable(ident);
                    bytecode.push(Instruction::Mov(reg, i+1))
                }
                _ => {

                }
            }
        }

        let (mut body_code, result_reg) = self.compile_expression(body, symbol_table);
        bytecode.append(&mut body_code);
        bytecode.push(Instruction::Mov(0, result_reg));
        self.functions.push(CompiledFunction { code_block: bytecode });

        let reg = symbol_table.register_intermediate();
        (vec![Instruction::MovConst(reg, function_index as i64)], reg)
    }

    fn compile_statement (&mut self, statement: &Statement, symbol_table: &mut SymbolTable) 
    -> Vec<Instruction> {
        let mut bytecode = Vec::new();

        use Statement as Statement;
        match statement {
            Statement::Expression(expression) => {
                let (mut expr, reg) = self.compile_expression(&expression.kind, symbol_table);
                bytecode.append(&mut expr);
            },
            Statement::Return(return_statement) => {

            },
        }

        bytecode

    }

    fn compile_assignment (&mut self, lhs: &Pattern, rhs: &Expression, symbol_table: &mut SymbolTable) 
    -> (Vec<Instruction>, usize) {
        let mut bytecode = Vec::new();

        let lhs_reg = match &lhs{
            Pattern::Identifier(ident) => {
                symbol_table.register_variable(ident)
            }
            _ => {
                99
            }
        };
        let (mut expr, rhs_reg) = self.compile_expression(&rhs, symbol_table);
        bytecode.append(&mut expr);
        bytecode.push(
            Instruction::Mov(
                lhs_reg,
                rhs_reg,
            ));

        (bytecode, rhs_reg)
    }

    fn compile_expression (&mut self, expression: &Expression, symbol_table: &mut SymbolTable) 
    -> (Vec<Instruction>, usize) {

        match expression {
            Expression::Identifier(name) => {
                match name.as_str() {
                    "print" => {
                        let reg = symbol_table.register_intermediate();
                        (vec![Instruction::MovConst(reg, 0)], reg)
                    }
                    _ => {
                        let reg = symbol_table.get_variable(name);
                        (Vec::new(), *reg.expect(&format!("variable {} was not found fix this later", name))) 
                    }
                }
            }
            Expression::Integer(value) => {
                let reg = symbol_table.register_intermediate();
                (vec![Instruction::MovConst(reg, *value)], reg) 
            }
            Expression::Block { statements, final_expr } => {
                self.compile_block(&statements, final_expr, symbol_table)
            },
            Expression::Function{signature, body} => {
                self.compile_function(&signature, &body.kind, symbol_table)
            },
            Expression::Assignment{lhs, rhs} => {
                self.compile_assignment(&lhs.kind, &rhs.kind, symbol_table)
            },
            Expression::Conditional { condition, then_branch, else_branch } => {
                self.compile_conditional(
                    &condition.kind, 
                    &then_branch.kind, 
                    &else_branch,
                    symbol_table,
                )
            },
            Expression::Unary { op, expression } => {
                (Vec::new(), 0)
            },
            Expression::Binary { lhs, op, rhs } => {
                self.compile_binary_expression(&lhs.kind, op, &rhs.kind, symbol_table)
            }
            Expression::FunctionCall { callee, arguments } => {
                self.compile_function_call(&callee.kind, arguments, symbol_table)
            }
            _ => {
                (Vec::new(), 0)
            }
        }
    }

    fn compile_binary_expression (&mut self, lhs: &Expression, op: &BinaryOp, rhs: &Expression, symbol_table: &mut SymbolTable) 
    -> (Vec<Instruction>, usize) {
        let mut bytecode = Vec::new();

        let (mut lhs_code, lhs_reg) = self.compile_expression(&lhs, symbol_table);
        bytecode.append(&mut lhs_code);

        let (mut rhs_code, rhs_reg) = self.compile_expression(&rhs, symbol_table);
        bytecode.append(&mut rhs_code);

        let res_reg = symbol_table.register_intermediate();
        bytecode.push(match op {
            BinaryOp::Add => {
                Instruction::Add(res_reg, lhs_reg, rhs_reg)
            }
            BinaryOp::Sub => {
                Instruction::Sub(res_reg, lhs_reg, rhs_reg)
            }
            BinaryOp::Mul => {
                Instruction::Mul(res_reg, lhs_reg, rhs_reg)
            }
            BinaryOp::Div => {
                Instruction::Div(res_reg, lhs_reg, rhs_reg)
            }
            BinaryOp::Mod => {
                Instruction::Mod(res_reg, lhs_reg, rhs_reg)
            }
            BinaryOp::LEq => {
                Instruction::LEq(res_reg, lhs_reg, rhs_reg)
            }
            BinaryOp::Lt => {
                Instruction::LT(res_reg, lhs_reg, rhs_reg)
            }
            BinaryOp::GEq => {
                Instruction::GEq(res_reg, lhs_reg, rhs_reg)
            }
            BinaryOp::Gt => {
                Instruction::GT(res_reg, lhs_reg, rhs_reg)
            }
            BinaryOp::Eq => {
                Instruction::Eq(res_reg, lhs_reg, rhs_reg)
            }
            BinaryOp::NEq => {
                Instruction::NEq(res_reg, lhs_reg, rhs_reg)
            }
            _ => {
                Instruction::Add(res_reg, lhs_reg, rhs_reg)
            }
        });

        (bytecode, res_reg)
    }

    fn compile_block (&mut self, statements: &Vec<Statement>, final_expr: &Option<Box<Node<Expression>>>, symbol_table: &mut SymbolTable) 
    -> (Vec<Instruction>, usize) {

        let mut bytecode = Vec::new();

        for statement in statements {
            let mut compiled_statement = self.compile_statement(statement, symbol_table);
            bytecode.append(&mut compiled_statement);
        }

        let expr_reg = if let Some(expr) = final_expr {
            let (mut expr_code, expr_reg) = self.compile_expression(&expr.kind, symbol_table);
            bytecode.append(&mut expr_code);

            expr_reg
        }
        else {
            0
        };

        (bytecode, expr_reg)
    }

    fn compile_conditional (&mut self, condition: &Expression, then_branch: &Expression, else_branch: &Option<Box<Node<Expression>>>, symbol_table: &mut SymbolTable) 
    -> (Vec<Instruction>, usize) {
        let mut bytecode = Vec::new();
        let (mut cond_code, cond_reg) = self.compile_expression(condition, symbol_table);
        bytecode.append(&mut cond_code);

        let (mut then_code, then_reg) = self.compile_expression(then_branch, symbol_table);

        let final_reg = symbol_table.register_intermediate();

        if let Some(els) = else_branch {
            let jump = Instruction::JmpCond(then_code.len()+1, cond_reg);
            bytecode.push(jump);
            bytecode.append(&mut then_code);

            let (mut els_code, els_reg) = self.compile_expression(&els.kind, symbol_table);

            let jump = Instruction::Jmp(els_code.len());
            bytecode.push(jump);
            bytecode.append(&mut els_code);
            bytecode.push(Instruction::Mov(final_reg, els_reg));
        }
        else {
            let jump = Instruction::JmpCond(then_code.len()+1, cond_reg);
            bytecode.push(jump);
            bytecode.append(&mut then_code);
            bytecode.push(Instruction::Mov(final_reg, then_reg));
        };

        (bytecode, final_reg)
    }

    fn compile_function_call (&mut self, callee: &Expression, arguments: &Vec<Node<Expression>>, symbol_table: &mut SymbolTable) 
    -> (Vec<Instruction>, usize) {
        let mut bytecode = Vec::new();
        for (i, arg) in arguments.iter().enumerate() {
            let (mut arg_code, arg_reg) = self.compile_expression(&arg.kind, symbol_table);
            bytecode.append(&mut arg_code);
            bytecode.push(Instruction::Mov(i+1, arg_reg));
        }

        let (mut callee_code, callee_reg) = self.compile_expression(callee, symbol_table);
        bytecode.append(&mut callee_code);

        bytecode.push(Instruction::Call(callee_reg));

        (bytecode, 0)
    }
}