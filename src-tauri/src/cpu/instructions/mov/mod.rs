use crate::{cpu::CPU, memory::Memory};

pub mod direct_addressing;
pub mod immediate_addressing;
pub(super) mod indexed_addressing;
pub mod register_addressing;

impl CPU {
    pub(in crate::cpu) fn execute_mov_register_word(&mut self, mem: &mut Memory) {
        let instruction = self.consume_instruction(mem);
        match instruction {
            0x00..=0x3F => {
                self.mov_16bit_register_indexed_registers_without_offset(mem, instruction)
            }
            0x40..=0x7f => {
                self.mov_16bit_register_indexed_registers_with_8bit_offset(mem, instruction)
            }
            0x80..=0xBF => {
                self.mov_16bit_register_indexed_registers_with_16bit_offset(mem, instruction)
            }
            0xC0..=0xFF => {
                self.mov_16bit_register_addressing(instruction);
            }
        }
    }
}
