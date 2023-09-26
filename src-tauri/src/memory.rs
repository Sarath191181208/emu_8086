use crate::consts::{Byte, Word};
use serde::ser::SerializeTuple;
use serde::{Serialize, Serializer};


#[derive(Serialize, Clone)]
pub struct Memory {
    // #[serde(serialize_with = "serialize")]
    mem: Vec<Byte>,
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}

impl Memory {
    pub fn new() -> Memory {
        Memory { mem: vec![0; 0xFFFFF] }
    }

    fn get_addr(&self, segment: u16, address: u16) -> usize {
        (((segment*0x10) as u32) + (address as u32))  as usize
    }

    pub fn reset(&mut self) {
        self.mem = vec![0; 0xFFFFF];
    }

    pub fn read(&self, segment: u16, offset: u16) -> Byte {
        let address = self.get_addr(segment, offset);
        self.mem[address as usize]
    }

    pub fn write_byte(&mut self, segment: u16, offset: u16, data: Byte) {
        let address = self.get_addr(segment, offset);
        self.mem[address as usize] = data;
    }

    pub fn write_word(&mut self, segment: u16, offset: u16, data: Word) {
        let address = self.get_addr(segment, offset);
        self.mem[address as usize] = (data & 0xFF) as Byte;
        self.mem[(address + 1) as usize] = ((data >> 8) & 0xFF) as Byte;
    }
}

fn serialize<const N: usize, S, T>(t: &[T; N], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Serialize,
{
    let mut ser_tuple = serializer.serialize_tuple(N)?;
    for elem in t {
        ser_tuple.serialize_element(elem)?;
    }
    ser_tuple.end()
}
