use super::{
    instructions::*,
    memory::{Mem, Memory},
};

#[derive(Debug)]
pub struct VM {
    pub memory: Memory,
    pub program: Vec<Instruction>,
    pc: usize,
    sp: usize,
}

impl VM {
    pub fn new(p: Vec<Instruction>) -> Self {
        Self {
            memory: Memory::new(),
            program: p,
            pc: 0,
            sp: 0,
        }
    }

    fn push(&mut self, value: u8) -> Result<(), &'static str> {
        self.memory.write(self.sp, value)?;
        if self.sp <= 0 {
            self.sp += 1;
        }
        Ok(())
    }

    pub fn step(&mut self) -> Result<(), &'static str> {
        let inst = self.program[self.pc];
        match inst {
            Instruction::Push(num) => self.push(num)?,
            Instruction::AddStack => {
                let n1 = self.memory.read(self.sp)?;
                let n2 = self.memory.read(self.sp - 1)?;
                self.push(n1 + n2)?
            }
        };
        self.pc += 1;
        Ok(())
    }

    pub fn top(&self) -> Result<u8, &'static str> {
        self.memory.read(self.sp)
    }
}
