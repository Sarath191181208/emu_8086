use crate::{ cpu::CPU, memory::Memory};

pub mod direct_addressing;
pub mod immediate_addressing;
pub mod register_addressing;

impl CPU {
    // add bx, _
    pub(in crate::cpu) fn execute_add(&mut self, mem: &mut Memory) {
        let instruction = self.consume_instruction(&mem);
        match instruction {
            0x06..=0x3E => {
                let is_direct_addressing = ((instruction - 0x06) % 8) == 0;
                if is_direct_addressing {
                    self.add_16bit_reg_direct_address(mem, instruction);
                }
            }
            0xC0..=0xFF => {
                self.add_16bit_register_addressing(instruction);
            }
            x => unimplemented!("ADD instruction not implemented! for {}", x),
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
