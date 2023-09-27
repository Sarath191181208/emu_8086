use crate::{cpu::CPU, memory::Memory};

pub mod direct_addressing;
pub mod immediate_addressing;
pub mod register_addressing;

impl CPU {
    pub(in crate::cpu) fn execute_sub_register_word(&mut self, mem: &mut Memory) {
        let instruction = self.consume_instruction(mem);
        match instruction {
            0x06..=0x3E => {
                self.sub_16bit_reg_direct_address(mem, instruction);
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
            0x06..=0x3E => {
                self.sub_8bit_register_direct_address(mem, instruction);
            }
            0xC0..=0xFF => {
                self.sub_8bit_register_addressing(instruction);
            }
            x => panic!("SUB instruction not implemented! for {}", x),
        }
    }
}
