use crate::vm::lang_bytecode::Instruction;
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

pub fn exec(bytecode: &Vec<Instruction>) -> i32 {
    let mut vm = VM::new();
    while vm.ip < bytecode.len() {
        let fetched_instr = &bytecode[vm.ip];
        match fetched_instr {
            Instruction::Mov(r1, r2) => {
                vm.gp_registers[*r1] = vm.gp_registers[*r2];
            }
            Instruction::MovConst(r1, val) => {
                vm.gp_registers[*r1] = *val;
            }
            Instruction::Halt => {

            }
            Instruction::Print(r1) => {
                println!("{:?}", vm.gp_registers[*r1 as usize]);
            }
            Instruction::Jmp(r1) => {
                continue;
            }
            Instruction::Add(r1, r2) => {
                vm.gp_registers[*r1] = vm.gp_registers[*r1] + vm.gp_registers[*r2];
            }
            Instruction::Sub(r1, r2) => {
                vm.gp_registers[*r1] = vm.gp_registers[*r1] - vm.gp_registers[*r2];
            }
            Instruction::Mul(r1, r2) => {
                vm.gp_registers[*r1] = vm.gp_registers[*r1] * vm.gp_registers[*r2];
            }
            Instruction::Div(r1, r2) => {
                vm.gp_registers[*r1] = vm.gp_registers[*r1] / vm.gp_registers[*r2];
            }
            Instruction::Mod(r1, r2) => {
                vm.gp_registers[*r1] = vm.gp_registers[*r1] % vm.gp_registers[*r2];
            }
        }
        vm.ip += 1;
    }

    1
}