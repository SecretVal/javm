mod asm;
mod vm;

use std::{env, fs};

use asm::{generator::Generator, parser::Parser};
use vm::vm::VM;

fn main() -> Result<(), &'static str> {
    let args: Vec<_> = env::args().collect();
    let filename = &args[1];
    let file = fs::read_to_string(filename).unwrap();
    let mut parser = Parser::new(asm::tokenize(file));
    let mut expressions = vec![];
    while let Some(expr) = parser.parse_expression() {
        expressions.push(expr);
    }
    let mut generator = Generator::new(expressions);
    while let Some(_) = generator.generate_expression() {}
    println!("{:?}", generator.output);
    let mut vm = VM::new(generator.output);
    while !vm.halt {
        vm.step()?;
    }
    Ok(())
}
