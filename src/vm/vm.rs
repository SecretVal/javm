use serde::{Deserialize, Serialize};

use super::{
    instructions::*,
    memory::{Mem, Memory},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct VM {
    pub memory: Memory,
    pub program: Vec<Instruction>,
    pub pc: usize,
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

    fn push(&mut self, value: u64) -> Result<(), &'static str> {
        self.sp += 1;
        self.memory.write(self.sp, value)?;
        Ok(())
    }

    pub fn step(&mut self) -> Result<(), &'static str> {
        let inst = self.program[self.pc];
        match inst {
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
            Instruction::Jmp(dest) => {
                self.pc = dest;
            }
            Instruction::JmpZero(dest) => {
                if self.top()? == 0 {
                    self.pc = dest;
                }
            }
            Instruction::JmpEquals(dest) => {
                if self.top()? == self.memory.read(self.sp - 1)? {
                    self.pc = dest;
                }
            }
            Instruction::JmpLess(dest) => {
                if self.top()? < self.memory.read(self.sp - 1)? {
                    self.pc = dest;
                }
            }
            Instruction::JmpGreater(dest) => {
                if self.top()? > self.memory.read(self.sp - 1)? {
                    self.pc = dest;
                }
            }
            Instruction::Nop => {}
        };
        match inst {
            Instruction::Jmp(_) => {}
            Instruction::JmpZero(_) => {
                if self.top()? != 0 {
                    self.pc += 1;
                }
            }
            Instruction::JmpEquals(_) => {
                if self.top()? != self.memory.read(self.sp - 1)? {
                    self.pc += 1;
                }
            }
            Instruction::JmpGreater(_) => {
                if !(self.top()? > self.memory.read(self.sp - 1)?) {
                    self.pc += 1;
                }
            }
            Instruction::JmpLess(_) => {
                if !(self.top()? < self.memory.read(self.sp - 1)?) {
                    self.pc += 1;
                }
            }
            _ => self.pc += 1,
        };
        Ok(())
    }

    pub fn top(&self) -> Result<u64, &'static str> {
        self.memory.read(self.sp)
    }
}
