use crate::{cpu::CPU, memory::Memory};

impl CPU {
    pub(in crate::cpu) fn execute_loop_8bit(&mut self, mem: &mut Memory) {
        let offset = self.consume_instruction(mem);
        match offset {
            0x80..=0xFF => {
                let offset = 0xFF - offset + 1;
                let ip = self.instruction_pointer;
                self.set_instruction_pointer(ip.wrapping_sub(offset as u16));
            }
            0x00..=0x7F => {
                let ip = self.instruction_pointer;
                self.set_instruction_pointer(ip.wrapping_add(offset as u16));
            }
        }
    }
}