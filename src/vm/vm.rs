use super::{
    instructions::*,
    memory::{Mem, Memory},
};

#[derive(Debug)]
pub struct VM {
    pub memory: Memory,
    pub program: Vec<Instruction>,
    pc: usize,
    sp: i64,
    pub halt: bool,
}

impl VM {
    pub fn new(p: Vec<Instruction>) -> Self {
        Self {
            memory: Memory::new(),
            program: p,
            pc: 0,
            sp: -1,
            halt: false,
        }
    }

    fn push(&mut self, value: u8) -> Result<(), &'static str> {
        self.sp += 1;
        self.memory.write(self.sp, value)?;
        Ok(())
    }

    pub fn step(&mut self) -> Result<(), &'static str> {
        match self.program[self.pc] {
            Instruction::Push(num) => self.push(num)?,
            Instruction::AddStack => {
                let n1 = self.memory.read(self.sp - 1)?;
                let n2 = self.memory.read(self.sp)?;
                self.push(n1 + n2)?
            }
            Instruction::SubStack => {
                let n1 = self.memory.read(self.sp - 1)?;
                let n2 = self.memory.read(self.sp)?;
                self.push(n1 - n2)?
            }
            Instruction::MulStack => {
                let n1 = self.memory.read(self.sp - 1)?;
                let n2 = self.memory.read(self.sp)?;
                self.push(n1 * n2)?
            }
            Instruction::DivStack => {
                let n1 = self.memory.read(self.sp - 1)?;
                let n2 = self.memory.read(self.sp)?;
                self.push(n1 / n2)?
            }
            Instruction::Halt => {
                self.halt = true;
            }
            Instruction::Print => {
                println!("{}", self.top()?);
            }
        };
        self.pc += 1;
        Ok(())
    }

    pub fn top(&self) -> Result<u8, &'static str> {
        self.memory.read(self.sp)
    }
}
