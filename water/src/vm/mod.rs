use crate::bytecode::Instruction;
use crate::codegen::CompiledFunction;
const NR_OF_REGISTERS: usize = 100;

struct VM {
    pub ip: usize,
    pub gp_registers: [i64; NR_OF_REGISTERS],
    pub fp_registers: [f64; NR_OF_REGISTERS],
    pub status_register: StatusRegister,
    pub stack: Vec<i64>,
}

pub struct StatusRegister {
        zf: bool,
        cf: bool,
        sf: bool,
        of: bool,
}

impl VM {
    fn new () -> Self {
        Self {
            ip: 0,
            gp_registers: [0; NR_OF_REGISTERS],
            fp_registers: [0.0; NR_OF_REGISTERS],
            status_register: StatusRegister {
                zf: false,
                cf: false,
                sf: false,
                of: false,
            },
            stack: Vec::new(),
        }
    }
}

pub fn exec(bytecode: &Vec<Instruction>, functions: &Vec<CompiledFunction>) -> i32 {
    let mut vm = VM::new();

    vm.exec_bytecode(bytecode, functions)
}

impl VM {
    pub fn exec_bytecode(&mut self, bytecode: &Vec<Instruction>, functions: &Vec<CompiledFunction>) -> i32 {

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
                            let function = &functions[index as usize];
                            
                            // Save caller's frame
                            let saved_registers = self.gp_registers;
                            let old_ip = self.ip;

                            self.ip = 0;
                            self.exec_bytecode(&function.code_block, functions);

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
                    self.gp_registers[*r1] = self.gp_registers[*r2] + self.gp_registers[*r3];
                }
                Instruction::Sub(r1, r2, r3) => {
                    self.gp_registers[*r1] = self.gp_registers[*r2] - self.gp_registers[*r3];
                }
                Instruction::Mul(r1, r2, r3) => {
                    self.gp_registers[*r1] = self.gp_registers[*r2] * self.gp_registers[*r3];
                }
                Instruction::Div(r1, r2, r3) => {
                    self.gp_registers[*r1] = self.gp_registers[*r2] / self.gp_registers[*r3];
                }
                Instruction::Mod(r1, r2, r3) => {
                    self.gp_registers[*r1] = self.gp_registers[*r2] % self.gp_registers[*r3];
                }
                Instruction::Eq(r1, r2, r3) => {
                    self.gp_registers[*r1] = (self.gp_registers[*r2] == self.gp_registers[*r3]) as i64;
                }
                Instruction::NEq(r1, r2, r3) => {
                    self.gp_registers[*r1] = (self.gp_registers[*r2] != self.gp_registers[*r3]) as i64;
                }
                Instruction::GT(r1, r2, r3) => {
                    self.gp_registers[*r1] = (self.gp_registers[*r2] > self.gp_registers[*r3]) as i64;
                }
                Instruction::LT(r1, r2, r3) => {
                    self.gp_registers[*r1] = (self.gp_registers[*r2] < self.gp_registers[*r3]) as i64;
                }
                Instruction::GEq(r1, r2, r3) => {
                    self.gp_registers[*r1] = (self.gp_registers[*r2] >= self.gp_registers[*r3]) as i64;
                }
                Instruction::LEq(r1, r2, r3) => {
                    self.gp_registers[*r1] = (self.gp_registers[*r2] <= self.gp_registers[*r3]) as i64;
                }
                _ => {

                }
            }
            self.ip += 1;
        }

        1
    }
}