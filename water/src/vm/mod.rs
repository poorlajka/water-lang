pub mod heap;
use crate::bytecode::{Instruction, CompiledFunction, Opcode, Program};
use crate::bytecode::value::{tag_int, untag_int, is_int, is_bool, untag_bool, tag_bool, tag_pointer, untag_pointer, is_pointer, Value};
use heap::{Heap, ObjectKind};
use std::io::{self, Write};

pub fn exec(program: &Program) {
    exec_with(program, Box::new(io::stdout()));
}

pub fn exec_with(program: &Program, output: Box<dyn Write>) {
    let mut interpreter = Interpreter::new(output);
    interpreter.run(program);
}

const NR_OF_REGISTERS: usize = 100;

struct VM {
    pub gp_registers: [u64; NR_OF_REGISTERS],
    pub fp_registers: [f64; NR_OF_REGISTERS],
    pub call_stack: Vec<CallFrame>,
    pub heap: Heap,
    pub output: Box<dyn Write>,
}

struct CallFrame {
    registers: [u64; NR_OF_REGISTERS],
    return_ip: usize,
    function_index: usize,
}

impl VM {
    fn new(output: Box<dyn Write>) -> Self {
        Self {
            gp_registers: [0; NR_OF_REGISTERS],
            fp_registers: [0.0; NR_OF_REGISTERS],
            call_stack: Vec::new(),
            heap: Heap::new(1024 * 1024),
            output,
        }
    }
}
struct Interpreter {
    pub vm: VM,
    ip: usize,
    current_function: usize,
}

type Handler = fn(&mut Interpreter, Instruction, &Program);

const DISPATCH_TABLE: [Handler; Opcode::COUNT as usize] = [
    Interpreter::handle_mov,
    Interpreter::handle_mov_const,
    Interpreter::handle_add,
    Interpreter::handle_sub,
    Interpreter::handle_mul,
    Interpreter::handle_div,
    Interpreter::handle_mod,
    Interpreter::handle_eq,
    Interpreter::handle_neq,
    Interpreter::handle_gt,
    Interpreter::handle_lt,
    Interpreter::handle_geq,
    Interpreter::handle_leq,
    Interpreter::handle_jmp,
    Interpreter::handle_jmp_cond,
    Interpreter::handle_call,
    Interpreter::handle_return,
    Interpreter::handle_load_string,
    Interpreter::handle_halt,
    Interpreter::handle_create_array,
    Interpreter::handle_load_index,
    Interpreter::handle_store_index,
];

impl Interpreter {
    pub fn new(output: Box<dyn Write>) -> Self {
        Self {
            vm: VM::new(output),
            ip: 0,
            current_function: 0,
        }
    }

    pub fn run(&mut self, program: &Program) {
        loop {
            let instr = {
                let bytecode = if self.current_function == 0 {
                    &program.main
                } else {
                    &program.functions[self.current_function].code_block
                };

                if self.ip >= bytecode.len() {
                    None
                } else {
                    Some(bytecode[self.ip].clone())
                }
            };

            match instr {
                None => {
                    if let Some(frame) = self.vm.call_stack.pop() {
                        let return_value = self.vm.gp_registers[0];
                        self.vm.gp_registers = frame.registers;
                        self.ip = frame.return_ip;
                        self.vm.gp_registers[0] = return_value;
                        self.current_function = frame.function_index;
                    } else {
                        break;
                    }
                }
                Some(instr) => self.dispatch(instr, program),
            }
        }
    }

    pub fn dispatch(&mut self, instr: Instruction, program: &Program) {
        let handler = DISPATCH_TABLE[instr.opcode as usize];
        handler(self, instr, program);
    }

    fn handle_mov(&mut self, instr: Instruction, _program: &Program) {
        self.vm.gp_registers[instr.op0 as usize] = self.vm.gp_registers[instr.op1 as usize];
        self.ip += 1;
    }

    fn handle_mov_const(&mut self, instr: Instruction, _program: &Program) {
        self.vm.gp_registers[instr.op0 as usize] = instr.op1;
        self.ip += 1;
    }

    fn handle_add(&mut self, instr: Instruction, _program: &Program) {
        let a = untag_int(self.vm.gp_registers[instr.op1 as usize]);
        let b = untag_int(self.vm.gp_registers[instr.op2 as usize]);
        self.vm.gp_registers[instr.op0 as usize] = tag_int(a + b);
        self.ip += 1;
    }

    fn handle_sub(&mut self, instr: Instruction, _program: &Program) {
        let a = untag_int(self.vm.gp_registers[instr.op1 as usize]);
        let b = untag_int(self.vm.gp_registers[instr.op2 as usize]);
        self.vm.gp_registers[instr.op0 as usize] = tag_int(a - b);
        self.ip += 1;
    }

    fn handle_mul(&mut self, instr: Instruction, _program: &Program) {
        let a = untag_int(self.vm.gp_registers[instr.op1 as usize]);
        let b = untag_int(self.vm.gp_registers[instr.op2 as usize]);
        self.vm.gp_registers[instr.op0 as usize] = tag_int(a * b);
        self.ip += 1;
    }

    fn handle_div(&mut self, instr: Instruction, _program: &Program) {
        let a = untag_int(self.vm.gp_registers[instr.op1 as usize]);
        let b = untag_int(self.vm.gp_registers[instr.op2 as usize]);
        self.vm.gp_registers[instr.op0 as usize] = tag_int(a / b);
        self.ip += 1;
    }

    fn handle_mod(&mut self, instr: Instruction, _program: &Program) {
        let a = untag_int(self.vm.gp_registers[instr.op1 as usize]);
        let b = untag_int(self.vm.gp_registers[instr.op2 as usize]);
        self.vm.gp_registers[instr.op0 as usize] = tag_int(a % b);
        self.ip += 1;
    }

    fn handle_eq(&mut self, instr: Instruction, _program: &Program) {
        let a = untag_int(self.vm.gp_registers[instr.op1 as usize]);
        let b = untag_int(self.vm.gp_registers[instr.op2 as usize]);
        self.vm.gp_registers[instr.op0 as usize] = tag_bool(a == b);
        self.ip += 1;
    }

    fn handle_neq(&mut self, instr: Instruction, _program: &Program) {
        let a = untag_int(self.vm.gp_registers[instr.op1 as usize]);
        let b = untag_int(self.vm.gp_registers[instr.op2 as usize]);
        self.vm.gp_registers[instr.op0 as usize] = tag_bool(a != b);
        self.ip += 1;
    }

    fn handle_gt(&mut self, instr: Instruction, _program: &Program) {
        let a = untag_int(self.vm.gp_registers[instr.op1 as usize]);
        let b = untag_int(self.vm.gp_registers[instr.op2 as usize]);
        self.vm.gp_registers[instr.op0 as usize] = tag_bool(a > b);
        self.ip += 1;
    }

    fn handle_lt(&mut self, instr: Instruction, _program: &Program) {
        let a = untag_int(self.vm.gp_registers[instr.op1 as usize]);
        let b = untag_int(self.vm.gp_registers[instr.op2 as usize]);
        self.vm.gp_registers[instr.op0 as usize] = tag_bool(a < b);
        self.ip += 1;
    }

    fn handle_geq(&mut self, instr: Instruction, _program: &Program) {
        let a = untag_int(self.vm.gp_registers[instr.op1 as usize]);
        let b = untag_int(self.vm.gp_registers[instr.op2 as usize]);
        self.vm.gp_registers[instr.op0 as usize] = tag_bool(a >= b);
        self.ip += 1;
    }

    fn handle_leq(&mut self, instr: Instruction, _program: &Program) {
        let a = untag_int(self.vm.gp_registers[instr.op1 as usize]);
        let b = untag_int(self.vm.gp_registers[instr.op2 as usize]);
        self.vm.gp_registers[instr.op0 as usize] = tag_bool(a <= b);
        self.ip += 1;
    }

    fn handle_jmp(&mut self, instr: Instruction, _program: &Program) {
        self.ip = (self.ip as i64 + instr.op0 as i64 + 1) as usize;
    }

    fn handle_jmp_cond(&mut self, instr: Instruction, _program: &Program) {
        let cond = untag_bool(self.vm.gp_registers[instr.op1 as usize]);
        if !cond {
            self.ip = (self.ip as i64 + instr.op0 as i64 + 1) as usize;
        } else {
            self.ip += 1;
        }
    }

    fn handle_call(&mut self, instr: Instruction, _program: &Program) {
        let index = self.vm.gp_registers[instr.op0 as usize];
        match index {
            0 => {
                let val = self.vm.gp_registers[1];
                if is_bool(val) {
                    writeln!(self.vm.output, "{}", untag_bool(val)).ok();
                } else if is_int(val) {
                    writeln!(self.vm.output, "{}", untag_int(val)).ok();
                } else if is_pointer(val) {
                    let ptr = untag_pointer(val);
                    let kind = self.vm.heap.get_kind(ptr);
                    match kind {
                        k if k == ObjectKind::String as u8 => {
                            let size = self.vm.heap.get_size(ptr);
                            let data_ptr = self.vm.heap.data_ptr(ptr);
                            let bytes = self.vm.heap.read_bytes(data_ptr, size);
                            let s = std::str::from_utf8(bytes).expect("invalid utf8");
                            writeln!(self.vm.output, "{}", s).ok();
                        }
                        _ => { writeln!(self.vm.output, "<object at {:x}>", val).ok(); }
                    }
                }

                self.ip += 1;
            }
            _ => {
                self.vm.call_stack.push(CallFrame {
                    registers: self.vm.gp_registers,
                    return_ip: self.ip + 1,
                    function_index: self.current_function,
                });
                self.current_function = index as usize;
                self.ip = 0;
            }

        }
    }

    fn handle_return(&mut self, _instr: Instruction, _program: &Program) {
        if let Some(frame) = self.vm.call_stack.pop() {
            let return_value = self.vm.gp_registers[0];
            self.vm.gp_registers = frame.registers;
            self.ip = frame.return_ip;
            self.vm.gp_registers[0] = return_value;
            self.current_function = frame.function_index;
        } else {
            self.ip = usize::MAX;
        }
    }

    fn handle_load_string(&mut self, instr: Instruction, program: &Program) {
        let string_index = instr.op1 as usize;
        let s = &program.strings[string_index];
        let bytes = s.as_bytes();

        // Allocate on heap: header + string bytes
        let ptr = self.vm.heap.alloc(ObjectKind::String, bytes.len())
            .expect("out of memory");

        // Write the string bytes into the data section
        self.vm.heap.write_bytes(self.vm.heap.data_ptr(ptr), bytes);

        // Store tagged pointer in destination register
        self.vm.gp_registers[instr.op0 as usize] = tag_pointer(ptr);
        self.ip += 1;
    }

    fn handle_halt(&mut self, _instr: Instruction, _program: &Program) {
        self.ip = usize::MAX;
    }

    fn handle_create_array(&mut self, instr: Instruction, _program: &Program) {
        let size = instr.op1 as usize;
        let data_size = size * 8; // each element is a tagged Value (u64)

        let ptr = self.vm.heap.alloc(ObjectKind::Array, data_size)
            .expect("out of memory");

        self.vm.gp_registers[instr.op0 as usize] = tag_pointer(ptr);
        self.ip += 1;
    }

    fn handle_load_index(&mut self, instr: Instruction, _program: &Program) {
        let arr_ptr = untag_pointer(self.vm.gp_registers[instr.op1 as usize]);
        let idx = untag_int(self.vm.gp_registers[instr.op2 as usize]) as usize;

        let data_ptr = self.vm.heap.data_ptr(arr_ptr);
        let element_offset = data_ptr + idx * 8;
        let value = self.vm.heap.read_u64(element_offset);

        self.vm.gp_registers[instr.op0 as usize] = value;
        self.ip += 1;
    }

    fn handle_store_index(&mut self, instr: Instruction, _program: &Program) {
        let arr_ptr = untag_pointer(self.vm.gp_registers[instr.op0 as usize]);
        let idx = untag_int(self.vm.gp_registers[instr.op1 as usize]) as usize;
        let value = self.vm.gp_registers[instr.op2 as usize];

        let data_ptr = self.vm.heap.data_ptr(arr_ptr);
        let element_offset = data_ptr + idx * 8;
        self.vm.heap.write_u64(element_offset, value);

        self.ip += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dispatch_table_ordering() {
        // Verify each opcode's discriminant matches its position in the table
        assert_eq!(Opcode::Mov as usize, 0);
        assert_eq!(Opcode::MovConst as usize, 1);
        assert_eq!(Opcode::Add as usize, 2);
        assert_eq!(Opcode::Sub as usize, 3);
        assert_eq!(Opcode::Mul as usize, 4);
        assert_eq!(Opcode::Div as usize, 5);
        assert_eq!(Opcode::Mod as usize, 6);
        assert_eq!(Opcode::Eq as usize, 7);
        assert_eq!(Opcode::NEq as usize, 8);
        assert_eq!(Opcode::GT as usize, 9);
        assert_eq!(Opcode::LT as usize, 10);
        assert_eq!(Opcode::GEq as usize, 11);
        assert_eq!(Opcode::LEq as usize, 12);
        assert_eq!(Opcode::Jmp as usize, 13);
        assert_eq!(Opcode::JmpCond as usize, 14);
        assert_eq!(Opcode::Call as usize, 15);
        assert_eq!(Opcode::Return as usize, 16);
        assert_eq!(Opcode::LoadString as usize, 17);
        assert_eq!(Opcode::Halt as usize, 18);
        assert_eq!(Opcode::CreateArray as usize, 19);
        assert_eq!(Opcode::LoadIndex as usize, 20);
        assert_eq!(Opcode::StoreIndex as usize, 21);
        
        // Verify table length matches number of opcodes
        assert_eq!(DISPATCH_TABLE.len(), Opcode::COUNT as usize);
    }
}