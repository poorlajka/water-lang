pub mod value;

#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum Opcode {
    Mov = 0,
    MovConst = 1,
    Add = 2,
    Sub = 3,
    Mul = 4,
    Div = 5,
    Mod = 6,
    Eq = 7,
    NEq = 8,
    GT = 9,
    LT = 10,
    GEq = 11,
    LEq = 12,
    Jmp = 13,
    JmpCond = 14,
    Call = 15,
    Return = 16,
    LoadString = 17,
    Halt = 18,
    CreateArray = 19,
    LoadIndex = 20,
    StoreIndex = 21,
    Not = 22,
    COUNT,
}

#[derive(Clone, Copy, Debug)]
pub struct Instruction {
    pub opcode: Opcode,
    pub op0: u64,
    pub op1: u64,
    pub op2: u64,
}

impl Instruction {
    pub fn new(opcode: Opcode, op0: u64, op1: u64, op2: u64) -> Self {
        Self { opcode, op0, op1, op2 }
    }

    pub fn mov(dst: usize, src: usize) -> Self {
        Self::new(Opcode::Mov, dst as u64, src as u64, 0)
    }

    pub fn mov_const(dst: usize, val: u64) -> Self {
        Self::new(Opcode::MovConst, dst as u64, val, 0)
    }

    pub fn add(dst: usize, lhs: usize, rhs: usize) -> Self {
        Self::new(Opcode::Add, dst as u64, lhs as u64, rhs as u64)
    }

    pub fn sub(dst: usize, lhs: usize, rhs: usize) -> Self {
        Self::new(Opcode::Sub, dst as u64, lhs as u64, rhs as u64)
    }

    pub fn mul(dst: usize, lhs: usize, rhs: usize) -> Self {
        Self::new(Opcode::Mul, dst as u64, lhs as u64, rhs as u64)
    }

    pub fn div(dst: usize, lhs: usize, rhs: usize) -> Self {
        Self::new(Opcode::Div, dst as u64, lhs as u64, rhs as u64)
    }

    pub fn mod_(dst: usize, lhs: usize, rhs: usize) -> Self {
        Self::new(Opcode::Mod, dst as u64, lhs as u64, rhs as u64)
    }

    pub fn eq(dst: usize, lhs: usize, rhs: usize) -> Self {
        Self::new(Opcode::Eq, dst as u64, lhs as u64, rhs as u64)
    }

    pub fn neq(dst: usize, lhs: usize, rhs: usize) -> Self {
        Self::new(Opcode::NEq, dst as u64, lhs as u64, rhs as u64)
    }

    pub fn gt(dst: usize, lhs: usize, rhs: usize) -> Self {
        Self::new(Opcode::GT, dst as u64, lhs as u64, rhs as u64)
    }

    pub fn lt(dst: usize, lhs: usize, rhs: usize) -> Self {
        Self::new(Opcode::LT, dst as u64, lhs as u64, rhs as u64)
    }

    pub fn geq(dst: usize, lhs: usize, rhs: usize) -> Self {
        Self::new(Opcode::GEq, dst as u64, lhs as u64, rhs as u64)
    }

    pub fn leq(dst: usize, lhs: usize, rhs: usize) -> Self {
        Self::new(Opcode::LEq, dst as u64, lhs as u64, rhs as u64)
    }

    pub fn jmp(offset: usize) -> Self {
        Self::new(Opcode::Jmp, offset as u64, 0, 0)
    }

    pub fn jmp_cond(offset: usize, cond_reg: usize) -> Self {
        Self::new(Opcode::JmpCond, offset as u64, cond_reg as u64, 0)
    }

    pub fn call(reg: usize) -> Self {
        Self::new(Opcode::Call, reg as u64, 0, 0)
    }

    pub fn return_() -> Self {
        Self::new(Opcode::Return, 0, 0, 0)
    }

    pub fn load_string(dst: usize, string_index: usize) -> Self {
        Self::new(Opcode::LoadString, dst as u64, string_index as u64, 0)
    }

    pub fn halt() -> Self {
        Self::new(Opcode::Halt, 0, 0, 0)
    }

    pub fn create_array(dst: usize, size: usize) -> Self {
    Self::new(Opcode::CreateArray, dst as u64, size as u64, 0)
    }

    pub fn load_index(dst: usize, arr: usize, idx: usize) -> Self {
        Self::new(Opcode::LoadIndex, dst as u64, arr as u64, idx as u64)
    }

    pub fn store_index(arr: usize, idx: usize, src: usize) -> Self {
        Self::new(Opcode::StoreIndex, arr as u64, idx as u64, src as u64)
    }

    pub fn not(dst: usize, src: usize) -> Self {
        Self::new(Opcode::Not, dst as u64, src as u64, 0)
    }
}

pub struct CompiledFunction {
    pub code_block: Vec<Instruction>,
}

pub struct Program {
    pub main: Vec<Instruction>,
    pub functions: Vec<CompiledFunction>,
    pub strings: Vec<String>,
}