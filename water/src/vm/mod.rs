pub mod heap;
use crate::bytecode::Instruction;
use crate::codegen::CompiledFunction;
use heap::Heap;

const TAG_MASK:    u64 = 0b111;
const TAG_INT:     u64 = 0b001;
const TAG_POINTER: u64 = 0b000;
const TAG_BOOL:    u64 = 0b011;

fn tag_int(n: i64) -> u64 {
    ((n << 1) as u64) | TAG_INT
}

fn untag_int(val: u64) -> i64 {
    (val as i64) >> 1
}

fn is_int(val: u64) -> bool {
    val & 1 == 1
}

fn is_pointer(val: u64) -> bool {
    val & TAG_MASK == TAG_POINTER
}

fn tag_pointer(ptr: usize) -> u64 {
    ptr as u64
}

fn untag_pointer(val: u64) -> usize {
    val as usize
}

const NR_OF_REGISTERS: usize = 100;

struct VM {
    pub ip: usize,
    pub gp_registers: [u64; NR_OF_REGISTERS],
    pub fp_registers: [f64; NR_OF_REGISTERS],
    pub call_stack: Vec<CallFrame>,
    pub heap: Heap,
    pub functions: Vec<CompiledFunction>,
}

struct CallFrame {
    registers: [u64; NR_OF_REGISTERS],
    return_ip: usize,
    function_index: usize,
}

impl VM {
    fn new(functions: Vec<CompiledFunction>) -> Self {
        Self {
            ip: 0,
            gp_registers: [0; NR_OF_REGISTERS],
            fp_registers: [0.0; NR_OF_REGISTERS],
            heap: Heap::new(1024 * 1024), // 1MB to start
            functions,
        }
    }
}

pub fn exec(main: &Vec<Instruction>, functions: Vec<CompiledFunction>) -> i32 {
    let mut vm = VM::new(functions);

    vm.exec_bytecode(main)
}

impl VM {
    pub fn exec_bytecode(&mut self, bytecode: &Vec<Instruction>) -> i32 {

        while self.ip < bytecode.len() {
            let fetched_instr = &bytecode[self.ip];
            match fetched_instr {
                Instruction::Mov(r1, r2) => {
                    self.gp_registers[*r1] = self.gp_registers[*r2];
                }
                Instruction::MovConst(r1, val) => {
                    self.gp_registers[*r1] = *val;
                }
                Instruction::Halt => {

                }
                Instruction::Jmp(offset) => {
                    self.ip += offset;
                }
                Instruction::JmpCond(offset, cond_reg) => {
                    if self.gp_registers[*cond_reg] == 0 {
                        self.ip += offset;
                    }
                }
                Instruction::Call(index) => {
                    let index = self.gp_registers[*index];
                    match index {
                        0 => {
                            println!("{:?}", self.gp_registers[1]);
                        }
                        _ => {
                            let code_block = &self.functions[index as usize].code_block.clone();
                            
                            // Save caller's frame
                            let saved_registers = self.gp_registers;
                            let old_ip = self.ip;

                            self.ip = 0;
                            self.exec_bytecode(code_block);

                            // Capture return value before restoring
                            let return_value = self.gp_registers[0];

                            // Restore caller's frame
                            self.gp_registers = saved_registers;
                            self.ip = old_ip;

                            // Write return value into the restored frame
                            self.gp_registers[0] = return_value;
                        }
                    }
                }
                Instruction::Return => {
                    self.ip = bytecode.len();
                }
                Instruction::Add(r1, r2, r3) => {
                    let a = self.gp_registers[*r2] as i64;
                    let b = self.gp_registers[*r3] as i64;
                    self.gp_registers[*r1] = (a + b) as u64;
                }
                Instruction::Sub(r1, r2, r3) => {
                    let a = self.gp_registers[*r2] as i64;
                    let b = self.gp_registers[*r3] as i64;
                    self.gp_registers[*r1] = (a - b) as u64;
                }
                Instruction::Mul(r1, r2, r3) => {
                    let a = self.gp_registers[*r2] as i64;
                    let b = self.gp_registers[*r3] as i64;
                    self.gp_registers[*r1] = (a * b) as u64;
                }
                Instruction::Div(r1, r2, r3) => {
                    let a = self.gp_registers[*r2] as i64;
                    let b = self.gp_registers[*r3] as i64;
                    self.gp_registers[*r1] = (a / b) as u64;
                }
                Instruction::Mod(r1, r2, r3) => {
                    let a = self.gp_registers[*r2] as i64;
                    let b = self.gp_registers[*r3] as i64;
                    self.gp_registers[*r1] = (a % b) as u64;
                }
                Instruction::Eq(r1, r2, r3) => {
                    let a = self.gp_registers[*r2] as i64;
                    let b = self.gp_registers[*r3] as i64;
                    self.gp_registers[*r1] = (a == b) as u64;
                }
                Instruction::NEq(r1, r2, r3) => {
                    let a = self.gp_registers[*r2] as i64;
                    let b = self.gp_registers[*r3] as i64;
                    self.gp_registers[*r1] = (a != b) as u64;
                }
                Instruction::GT(r1, r2, r3) => {
                    let a = self.gp_registers[*r2] as i64;
                    let b = self.gp_registers[*r3] as i64;
                    self.gp_registers[*r1] = (a > b) as u64;
                }
                Instruction::LT(r1, r2, r3) => {
                    let a = self.gp_registers[*r2] as i64;
                    let b = self.gp_registers[*r3] as i64;
                    self.gp_registers[*r1] = (a < b) as u64;
                }
                Instruction::GEq(r1, r2, r3) => {
                    let a = self.gp_registers[*r2] as i64;
                    let b = self.gp_registers[*r3] as i64;
                    self.gp_registers[*r1] = (a >= b) as u64;
                }
                Instruction::LEq(r1, r2, r3) => {
                    let a = self.gp_registers[*r2] as i64;
                    let b = self.gp_registers[*r3] as i64;
                    self.gp_registers[*r1] = (a <= b) as u64;
                }
                _ => {

                }
            }
            self.ip += 1;
        }

        1
    }
}