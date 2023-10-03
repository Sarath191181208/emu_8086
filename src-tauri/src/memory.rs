use crate::consts::{Byte, Word, U20};

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
            mem: vec![0x90; 0xFFFFF],
            history: vec![],
        }
    }

    fn get_addr(&self, segment: u16, address: u16) -> usize {
        (((segment * 0x10) as u32) + (address as u32)) as usize
    }

    fn push_history(&mut self, time: usize, index_old_new_values_pairs: Vec<(usize, Byte, Byte)>) {
        let time_already_exists_index = self.history.iter().position(|h| h.time == time);
        match time_already_exists_index {
            Some(index) => {
                // if it exists, then add the new values to the existing time
                let mut new_index_old_new_values_pairs =
                    self.history[index].index_old_new_values_pairs.clone();
                new_index_old_new_values_pairs.extend(index_old_new_values_pairs);
                self.history[index].index_old_new_values_pairs = new_index_old_new_values_pairs;
            }
            None => {
                // if it doesn't exist, then add a new time
                self.history.push(MemoryHistory {
                    time,
                    index_old_new_values_pairs,
                });
            }
        }
    }

    pub fn reset(&mut self) {
        self.mem = vec![0x90; 0xFFFFF];
    }

    pub fn read_byte(&self, segment: u16, offset: u16) -> Byte {
        let address = self.get_addr(segment, offset);
        self.mem[address]
    }

    pub fn read_word(&self, segment: u16, offset: u16) -> Word {
        let addr_1 = self.get_addr(segment, offset);
        let addr_2 = self.get_addr(segment, offset + 1);
        let byte_1 = self.mem[addr_1];
        let byte_2 = self.mem[addr_2];
        ((byte_2 as u16) << 8) | (byte_1 as u16)
    }

    pub fn write_instructions(&mut self, segment: u16, offset: u16, data: &[Byte]) {
        for (i, byte) in data.iter().enumerate() {
            self.write_byte_with_a_time_stamp(segment, offset + i as u16, *byte, 0);
        }
    }

    fn write_byte_with_a_time_stamp(
        &mut self,
        segment: u16,
        offset: u16,
        new_data: Byte,
        time: usize,
    ) {
        let address = self.get_addr(segment, offset);
        let prev_data = self.mem[address];
        self.push_history(time, vec![(address, prev_data, new_data)]);
        self.mem[address] = new_data;
    }

    pub fn write_byte(&mut self, segment: u16, offset: u16, data: Byte) {
        self.write_byte_with_a_time_stamp(segment, offset, data, self.history.len());
    }

    pub fn write_word(&mut self, segment: u16, offset: u16, data: Word) {
        let time = self.history.len();
        let low_byte = (data & 0xFF) as Byte;
        let high_byte = ((data >> 8) & 0xFF) as Byte;
        self.write_byte_with_a_time_stamp(segment, offset, low_byte, time);
        self.write_byte_with_a_time_stamp(segment, offset + 1, high_byte, time);
    }

    pub fn read_word_with_u20(&self, offset: U20) -> Word {
        let (segment, offset) = offset.as_segment_offset();
        self.read_word(segment, offset)
    }

    pub fn read_byte_with_u20(&self, offset: U20) -> Byte {
        let (segment, offset) = offset.as_segment_offset();
        self.read_byte(segment, offset)
    }

    pub fn write_word_with_u20(&mut self, offset: U20, data: Word) {
        let (segment, offset) = offset.as_segment_offset();
        self.write_word(segment, offset, data);
    }

    pub fn write_byte_with_u20(&mut self, offset: U20, data: Byte) {
        let (segment, offset) = offset.as_segment_offset();
        self.write_byte(segment, offset, data);
    }
}

impl Memory {
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
            None => {
                vec![]
            }
        }
    }
}
