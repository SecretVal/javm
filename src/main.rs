mod asm;
mod vm;

use std::fs;

use asm::{generator::Generator, parser::Parser};
use vm::vm::VM;

use clap::Parser as ClapParser;

#[derive(ClapParser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    asssemble: bool,
    #[arg(short, long)]
    execute: bool,
    #[arg()]
    file: String,
}

fn main() -> Result<(), &'static str> {
    let args = Args::parse();
    let filename = args.file;
    let file = fs::read_to_string(&filename).unwrap();
    if args.asssemble {
        let mut parser = Parser::new(asm::tokenize(file));
        let mut expressions = vec![];
        while let Some(expr) = parser.parse_expression() {
            expressions.push(expr);
        }
        let mut generator = Generator::new(expressions);
        while let Some(_) = generator.generate_expression() {}
        let vm = VM::new(generator.output);
        let _ = fs::write(
            format!("{}.jab", filename.split(".").collect::<Vec<_>>()[0]),
            ron::to_string(&vm).unwrap(),
        );
    } else if args.execute {
        let mut vm: VM = ron::from_str(&file).unwrap();
        while !vm.halt {
            vm.step()?;
        }
    }

    Ok(())
}
