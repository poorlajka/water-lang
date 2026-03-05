#[derive(Debug)]
pub enum Instruction {
    MovConst(usize, i64),
    Mov(usize, usize),
    Print(usize),
    Halt,
    Jmp(usize),
    Add(usize, usize),
    Sub(usize, usize),
    Mul(usize, usize),
    Div(usize, usize),
    Mod(usize, usize),
}