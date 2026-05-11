use crate::ast::{Expression, Node, Pattern, FunctionSignature, Module, Statement, BinaryOp, UnaryOp};
use crate::bytecode::{self, Instruction, CompiledFunction, Program};
use crate::bytecode::value::{tag_int, untag_int, is_int, tag_pointer, untag_pointer, is_pointer, Value};

use std::collections::HashMap;
use std::env::VarsOs;

pub struct Compiler {
    pub main: Vec<Instruction>,
    pub functions: Vec<CompiledFunction>,
    pub strings: Vec<String>,
}

struct SymbolTable {
    scopes: Vec<(HashMap<String, usize>, usize)>,
    reg_top: usize,
}

impl SymbolTable {
    fn new () -> Self {
        Self {
            scopes: vec![],
            reg_top: 10,
        }
    }

    fn register_variable (&mut self, name: &str) -> usize {
        self.scopes.last_mut()
            .expect("If this happens it means the global scope is somehow closed")
            .0.insert(name.to_string(), self.reg_top);

        self.reg_top += 1;
        self.reg_top - 1 
    }

    fn register_intermediate (&mut self) -> usize {
        self.reg_top += 1;
        self.reg_top - 1 
    }

    fn push_scope(&mut self) { 
        self.scopes.push((HashMap::new(), self.reg_top)); 
    }

    fn pop_scope(&mut self) { 
        if let Some((_, watermark)) = self.scopes.pop() {
            self.reg_top = watermark;
        }
    }

    fn get_variable(&self, name: &str) -> Option<usize> {
        // Walk scopes from innermost outward
        self.scopes.iter().rev()
            .find_map(|scope| scope.0.get(name).copied())
    }
    
}

pub fn compile_module (module: &Module) -> Program {
    let mut symbol_table = SymbolTable::new();
    symbol_table.push_scope();
    let mut compiler = Compiler {
        main: Vec::new(),
        functions: vec![CompiledFunction{code_block: Vec::new()}],
        strings: Vec::new(),
    };

    for statement in &module.statements {
        let mut compiled_statement = compiler.compile_statement(statement, &mut symbol_table);
        compiler.main.append(&mut compiled_statement);
    }

    Program {
        main: compiler.main,
        functions: compiler.functions,
        strings: compiler.strings,
    }
}

impl Compiler {

    fn compile_function (&mut self, signature: &FunctionSignature, body: &Expression, symbol_table: &mut SymbolTable) 
    -> (Vec<Instruction>, usize) {

        symbol_table.push_scope();
        let mut bytecode = Vec::new();

        let function_index = self.functions.len();
        for (i, arg) in signature.args.iter().enumerate() {
            match &arg.kind {
                Pattern::Identifier(ident) => {
                    let reg = symbol_table.register_variable(ident);
                    bytecode.push(Instruction::mov(reg, i+1))
                }
                _ => {

                }
            }
        }

        let (mut body_code, result_reg) = self.compile_expression(body, symbol_table);
        bytecode.append(&mut body_code);
        bytecode.push(Instruction::mov(0, result_reg));
        self.functions.push(CompiledFunction { code_block: bytecode });

        symbol_table.pop_scope();
        let reg = symbol_table.register_intermediate();
        (vec![Instruction::mov_const(reg, function_index as u64)], reg)
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
            Statement::Return(expr) => {

                match expr {
                    Some(expr) => {
                        let (mut expr_code, expr_reg) = self.compile_expression(&expr.kind, symbol_table);
                        bytecode.append(&mut expr_code);
                        bytecode.push(Instruction::mov(0, expr_reg));
                    }
                    None => {

                    }
                }
                bytecode.push(Instruction::return_());
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
            Instruction::mov(
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
                        (vec![Instruction::mov_const(reg, 0)], reg)
                    }
                    _ => {
                        let reg = symbol_table.get_variable(name);
                        (Vec::new(), reg.expect(&format!("variable {} was not found fix this later", name))) 
                    }
                }
            }
            Expression::Integer(value) => {
                let reg = symbol_table.register_intermediate();
                (vec![Instruction::mov_const(reg, tag_int(*value))], reg)
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
            Expression::String(s) => {
                let string_index = self.strings.len();
                self.strings.push(s.clone());
                let reg = symbol_table.register_intermediate();
                (vec![Instruction::load_string(reg, string_index)], reg)
            }
            Expression::Array(elements) => {
                let mut bytecode = Vec::new();
                let size = elements.len();

                // Allocate the array
                let arr_reg = symbol_table.register_intermediate();
                bytecode.push(Instruction::create_array(arr_reg, size));

                // Compile and store each element
                for (i, element) in elements.iter().enumerate() {
                    let (mut elem_code, elem_reg) = self.compile_expression(&element.kind, symbol_table);
                    bytecode.append(&mut elem_code);

                    // index is a compile time constant so we need a register for it
                    let idx_reg = symbol_table.register_intermediate();
                    bytecode.push(Instruction::mov_const(idx_reg, tag_int(i as i64)));
                    bytecode.push(Instruction::store_index(arr_reg, idx_reg, elem_reg));
                }

                (bytecode, arr_reg)
            }
            Expression::Index { target, index } => {
                let (mut target_code, target_reg) = self.compile_expression(&target.kind, symbol_table);
                let (mut index_code, index_reg) = self.compile_expression(&index.kind, symbol_table);
                
                let dst_reg = symbol_table.register_intermediate();
                
                let mut bytecode = Vec::new();
                bytecode.append(&mut target_code);
                bytecode.append(&mut index_code);
                bytecode.push(Instruction::load_index(dst_reg, target_reg, index_reg));
                
                (bytecode, dst_reg)
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
            BinaryOp::Add => Instruction::add(res_reg, lhs_reg, rhs_reg),
            BinaryOp::Sub => Instruction::sub(res_reg, lhs_reg, rhs_reg),
            BinaryOp::Mul => Instruction::mul(res_reg, lhs_reg, rhs_reg),
            BinaryOp::Div => Instruction::div(res_reg, lhs_reg, rhs_reg),
            BinaryOp::Mod => Instruction::mod_(res_reg, lhs_reg, rhs_reg),
            BinaryOp::LEq => Instruction::leq(res_reg, lhs_reg, rhs_reg),
            BinaryOp::Lt  => Instruction::lt(res_reg, lhs_reg, rhs_reg),
            BinaryOp::GEq => Instruction::geq(res_reg, lhs_reg, rhs_reg),
            BinaryOp::Gt  => Instruction::gt(res_reg, lhs_reg, rhs_reg),
            BinaryOp::Eq  => Instruction::eq(res_reg, lhs_reg, rhs_reg),
            BinaryOp::NEq => Instruction::neq(res_reg, lhs_reg, rhs_reg),
            _ => Instruction::add(res_reg, lhs_reg, rhs_reg),
        });

        (bytecode, res_reg)
    }

    fn compile_block (&mut self, statements: &Vec<Statement>, final_expr: &Option<Box<Node<Expression>>>, symbol_table: &mut SymbolTable) 
    -> (Vec<Instruction>, usize) {

        symbol_table.push_scope();
        let mut bytecode = Vec::new();

        for statement in statements {
            let mut compiled_statement = self.compile_statement(statement, symbol_table);
            bytecode.append(&mut compiled_statement);
        }

        let final_expr_reg = if let Some(expr) = final_expr {
            let (mut expr_code, expr_reg) = self.compile_expression(&expr.kind, symbol_table);
            bytecode.append(&mut expr_code);

            Some(expr_reg)
        }
        else {
            None
        };

        symbol_table.pop_scope();

        if let Some(inner_reg) = final_expr_reg {
            let result_reg = symbol_table.register_intermediate();
                bytecode.push(Instruction::mov(result_reg, inner_reg));
                (bytecode, result_reg)
            } 
            else {
                (bytecode, 0) // no value produced, caller shouldn't use this
        }
    }

    fn compile_conditional (&mut self, condition: &Expression, then_branch: &Expression, else_branch: &Option<Box<Node<Expression>>>, symbol_table: &mut SymbolTable) 
    -> (Vec<Instruction>, usize) {
        let mut bytecode = Vec::new();
        let (mut cond_code, cond_reg) = self.compile_expression(condition, symbol_table);
        bytecode.append(&mut cond_code);

        let (mut then_code, then_reg) = self.compile_expression(then_branch, symbol_table);

        let final_reg = symbol_table.register_intermediate();

        if let Some(els) = else_branch {
            let jump = Instruction::jmp_cond(then_code.len()+2, cond_reg);
            bytecode.push(jump);
            bytecode.append(&mut then_code);
            bytecode.push(Instruction::mov(final_reg, then_reg));

            let (mut els_code, els_reg) = self.compile_expression(&els.kind, symbol_table);

            let jump = Instruction::jmp(els_code.len()+1);
            bytecode.push(jump);
            bytecode.append(&mut els_code);
            bytecode.push(Instruction::mov(final_reg, els_reg));
        }
        else {
            let jump = Instruction::jmp_cond(then_code.len(), cond_reg);
            bytecode.push(jump);
            bytecode.append(&mut then_code);
        };

        (bytecode, final_reg)
    }

    fn compile_function_call (&mut self, callee: &Expression, arguments: &Vec<Node<Expression>>, symbol_table: &mut SymbolTable) 
    -> (Vec<Instruction>, usize) {
        let mut bytecode = Vec::new();

        for (i, arg) in arguments.iter().enumerate() {
            let (mut arg_code, arg_reg) = self.compile_expression(&arg.kind, symbol_table);
            bytecode.append(&mut arg_code);
            bytecode.push(Instruction::mov(i+1, arg_reg));
        }

        let (mut callee_code, callee_reg) = self.compile_expression(callee, symbol_table);
        bytecode.append(&mut callee_code);

        bytecode.push(Instruction::call(callee_reg));

        // Evacuate return value out of reg 0 before anything can clobber it
    let result_reg = symbol_table.register_intermediate();
    bytecode.push(Instruction::mov(result_reg, 0));

        (bytecode, result_reg)
    }
}