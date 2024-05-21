use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Instruction {
    Push(u8),
    AddStack,
    SubStack,
    MulStack,
    DivStack,
    Halt,
    Print,
}
