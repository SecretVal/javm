#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    Push(u8),
    AddStack,
    SubStack,
    MulStack,
    DivStack,
    Halt,
    Print,
}
