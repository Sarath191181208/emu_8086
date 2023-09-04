use crate::consts::{Byte, Word};
use serde::ser::SerializeTuple;
use serde::{Serialize, Serializer};

#[derive(Serialize)]
pub struct Memory {
    #[serde(serialize_with = "serialize")]
    mem: [Byte; 0xFFFF],
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}

impl Memory {
    pub fn new() -> Memory {
        Memory { mem: [0; 0xFFFF] }
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
