

#[derive(Debug)]
pub enum Instruction {
    MovConst(usize, u64),
    Mov(usize, usize),
    Print(usize),
    Halt,
    Jmp(usize),
    JmpCond(usize, usize),
    Call(usize),
    Return,

    Add(usize, usize, usize),
    Sub(usize, usize, usize),
    Mul(usize, usize, usize),
    Div(usize, usize, usize),
    Mod(usize, usize, usize),
    Eq(usize, usize, usize),
    GT(usize, usize, usize),
    LT(usize, usize, usize),
    GEq(usize, usize, usize),
    LEq(usize, usize, usize),
    NEq(usize, usize, usize),
}