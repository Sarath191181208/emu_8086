use crate::{cpu::CPU, memory::Memory};

impl CPU {
    pub(in crate::cpu) fn execute_jmp_if_cx_zero_8bit(&mut self, mem: &mut Memory) {
        let offset = self.consume_byte(mem);
        if self.cx == 0 {
            self.execute_8bit_offset_jmp(offset);
        }
    }
}
