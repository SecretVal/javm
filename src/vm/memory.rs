use serde::{Deserialize, Serialize};

pub trait Mem {
    fn write(&mut self, pos: i64, value: u8) -> Result<(), &'static str>;
    fn read(&self, pos: i64) -> Result<u8, &'static str>;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Memory {
    pub(crate) bytes: Vec<u8>,
    pub(crate) size: i64,
}

impl Memory {
    pub(crate) fn new() -> Self {
        Self {
            bytes: vec![0; 2048],
            size: 2048,
        }
    }
}
impl Mem for Memory {
    fn write(&mut self, pos: i64, value: u8) -> Result<(), &'static str> {
        if self.size + 1 < pos {
            return Err("Memory overflow");
        }
        self.bytes[pos as usize] = value;
        Ok(())
    }

    fn read(&self, pos: i64) -> Result<u8, &'static str> {
        if self.size < pos {
            return Err("Memory overflow");
        }
        Ok(self.bytes[pos as usize])
    }
}
