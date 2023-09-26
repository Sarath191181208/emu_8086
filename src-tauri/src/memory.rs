use crate::consts::{Byte, Word};

#[derive(Debug)]
pub struct MemoryHistory {
    time: usize,
    index_old_new_values_pairs: Vec<(usize, Byte, Byte)>,
}

#[derive(Debug)]
pub struct Memory {
    // #[serde(serialize_with = "serialize")]
    mem: Vec<Byte>,
    history: Vec<MemoryHistory>,
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
    }
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            mem: vec![0; 0xFFFFF],
            history: vec![],
        }
    }

    fn get_addr(&self, segment: u16, address: u16) -> usize {
        (((segment * 0x10) as u32) + (address as u32)) as usize
    }

    pub fn reset(&mut self) {
        self.mem = vec![0; 0xFFFFF];
    }

    pub fn get_recent_new_bytes(&self) -> Vec<(usize, Byte)> {
        match self.history.last() {
            Some(history) => {
                let time = history.time;
                // collect all the values that match the time
                let history_vec_at_t = self
                    .history
                    .iter()
                    .filter(|h| h.time == time)
                    .collect::<Vec<&MemoryHistory>>();

                // reduce the 2d array into Vec<(usize, Byte)>
                history_vec_at_t
                    .iter()
                    .flat_map(|h| {
                        h.index_old_new_values_pairs
                            .iter()
                            .map(|(index, _, new_value)| (*index, *new_value))
                            .collect::<Vec<(usize, Byte)>>()
                    })
                    .collect::<Vec<(usize, Byte)>>()

            }
            None => { vec![] }
        }
    }

    pub fn read_byte(&self, segment: u16, offset: u16) -> Byte {
        let address = self.get_addr(segment, offset);
        self.mem[address as usize]
    }

    pub fn read_word(&self, segment: u16, offset: u16) -> Word {
        let addr_1 = self.get_addr(segment, offset);
        let addr_2 = self.get_addr(segment, offset + 1);
        let byte_1 = self.mem[addr_1 as usize];
        let byte_2 = self.mem[addr_2 as usize];
        ((byte_2 as u16) << 8) | (byte_1 as u16)
    }

    pub fn write_instructions(&mut self, segment: u16, offset: u16, data: &[Byte]) {
        for (i, byte) in data.iter().enumerate() {
            self.write_byte_with_a_time_stamp(segment, offset + i as u16, *byte, 0);
        }
    }

    fn write_byte_with_a_time_stamp(&mut self, segment: u16, offset: u16, data: Byte, time: usize) {
        let address = self.get_addr(segment, offset);
        self.history.push(MemoryHistory {
            time,
            index_old_new_values_pairs: vec![(address, self.mem[address], data)],
        });
        self.mem[address as usize] = data;
    }

    pub fn write_byte(&mut self, segment: u16, offset: u16, data: Byte) {
        self.write_byte_with_a_time_stamp(segment, offset, data, self.history.len());
    }

    pub fn write_word(&mut self, segment: u16, offset: u16, data: Word) {
        let address = self.get_addr(segment, offset);
        self.history.push(MemoryHistory {
            time: self.history.len(),
            index_old_new_values_pairs: vec![
                (address, self.mem[address], (data & 0xFF) as Byte),
                (
                    (address + 1) as usize,
                    self.mem[address + 1],
                    ((data >> 8) & 0xFF) as Byte,
                ),
            ],
        });
        self.mem[address as usize] = (data & 0xFF) as Byte;
        self.mem[(address + 1) as usize] = ((data >> 8) & 0xFF) as Byte;
    }
}
