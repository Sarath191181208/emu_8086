use crate::{cpu::CPU, memory::Memory};

pub mod direct_addressing;
pub mod immediate_addressing;
pub mod register_addressing;
pub mod indexed_addressing;

impl CPU {
    // add bx, _
    pub(in crate::cpu) fn execute_add(&mut self, mem: &mut Memory) {
        let instruction = self.consume_instruction(mem);
        match instruction {
            0x00..=0x3F => {
                self.add_16bit_register_indexed_registers_without_offset(mem, instruction)
            }
            0x40..=0x7F => {
                self.add_16bit_register_indexed_registers_with_8bit_offset(mem, instruction)
            }
            0x80..=0xBF => {
                self.add_16bit_register_indexed_registers_with_16bit_offset(mem, instruction)
            }
            0xC0..=0xFF => {
                self.add_16bit_register_addressing(instruction);
            }
        }
    }

    // add bl, _
    pub(in crate::cpu) fn execute_add_register(&mut self, mem: &mut Memory) {
        let instruction = self.consume_instruction(mem);
        match instruction {
            0x06..=0x3E => {
                let is_direct_addressing = ((instruction - 0x06) % 8) == 0;
                if is_direct_addressing {
                    self.add_8bit_register_direct_address(mem, instruction);
                }
            }
            0xC0..=0xFF => {
                self.add_8bit_register_addressing(instruction);
            }
            x => unimplemented!("ADD instruction not implemented! for {}", x),
        }
    }
}
