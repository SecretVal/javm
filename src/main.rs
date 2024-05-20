mod vm;

use vm::{instructions::Instruction, vm::VM};

fn main() -> Result<(), &'static str> {
    let mut vm = VM::new(vec![
        Instruction::Push(1),
        Instruction::Push(2),
        Instruction::AddStack,
    ]);
    vm.step()?;
    println!("{}", vm.top()?);
    vm.step()?;
    println!("{}", vm.top()?);
    vm.step()?;
    println!("{}", vm.top()?);
    Ok(())
}
