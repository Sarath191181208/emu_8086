use crate::{cpu::CPU, memory::Memory};

pub mod direct_addressing;
pub mod immediate_addressing;
pub mod indexed_addressing;
pub mod register_addressing;

impl CPU {
    pub(in crate::cpu) fn execute_sub_register_word(&mut self, mem: &mut Memory) {
        let instruction = self.consume_instruction(mem);
        match instruction {
            0x00..=0x3F => {
                self.sub_16bit_register_indexed_registers_without_offset(mem, instruction)
            }
            0x40..=0x7F => {
                self.sub_16bit_register_indexed_registers_with_8bit_offset(mem, instruction)
            }
            0x80..=0xBF => {
                self.sub_16bit_register_indexed_registers_with_16bit_offset(mem, instruction)
            }
            0xC1..=0xFF => {
                self.sub_16bit_register_addressing(instruction);
            }
            x => panic!("SUB instruction not implemented! for {}", x),
        }
    }

    pub(in crate::cpu) fn execute_sub_register_byte(&mut self, mem: &mut Memory) {
        let instruction = self.consume_instruction(mem);
        match instruction {
            0x00..=0x3F => {
                self.sub_8bit_register_indexed_registers_without_offset(mem, instruction)
            }
            0x40..=0x7F => {
                self.sub_8bit_register_indexed_registers_with_8bit_offset(mem, instruction)
            }
            0x80..=0xBF => {
                self.sub_8bit_register_indexed_registers_with_16bit_offset(mem, instruction)
            }
            0xC0..=0xFF => {
                self.sub_8bit_register_addressing(instruction);
            }
        }
    }
}
