mod vm;

use vm::{instructions::Instruction, vm::VM};

fn main() -> Result<(), &'static str> {
    let mut vm = VM::new(vec![
        Instruction::Push(1),
        Instruction::Push(2),
        Instruction::AddStack,
        Instruction::Print,
        Instruction::Halt,
    ]);
    while !vm.halt {
        vm.step()?;
    }
    Ok(())
}
