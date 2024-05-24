use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Instruction {
    Nop,
    Push(u64),
    AddStack,
    SubStack,
    MulStack,
    DivStack,
    Halt,
    Print,
    Jmp(usize),
    JmpZero(usize),
    JmpEquals(usize),
    JmpGreater(usize),
    JmpLess(usize),
}
