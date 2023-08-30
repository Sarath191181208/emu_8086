use crate::consts::{Word, Byte};

pub struct Memory{
    mem: [Byte; 0xFFFF],
}

impl Memory {
    pub fn new() -> Memory {
        Memory{
            mem: [0; 0xFFFF],
        }
    }

    pub fn reset(&mut self) {
        self.mem = [0; 0xFFFF];
    }

    pub fn read(&self, address: Word) -> Byte {
        self.mem[address as usize]
    }

    pub fn write_byte(&mut self, address: Word, data: Byte) {
        self.mem[address as usize] = data;
    }

    pub fn write_word(&mut self, address: Word, data: Word) {
        self.mem[address as usize] = (data & 0xFF) as Byte;
        self.mem[(address + 1) as usize] = ((data >> 8) & 0xFF) as Byte;
    }
}
