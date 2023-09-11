use crate::{cpu::CPU, memory::Memory};

impl CPU {
    pub(in crate::cpu) fn execute_mul_16bit(&mut self, mem: &mut Memory) {
        let opcode = self.consume_instruction(mem);
        let register_index = opcode & 0x07;
        let multplicator = self.get_16bit_register_by_index(register_index);
        let multiplicand = self.ax;
        let result = (multplicator as u32) * (multiplicand as u32);
        self.ax = (result & 0x0000_FFFF) as u16;
        self.dx = ((result & 0xFFFF_0000) >> 16) as u16;
        self.carry_flag = result > 0xFFFF;
        self.overflow_flag = result > 0xFFFF;

    }
    pub(in crate::cpu) fn execute_mul_8bit(&mut self, mem: &mut Memory) {
        let opcode = self.consume_instruction(mem);
        let register_index = opcode & 0x07;
        let multplicator = self.get_8bit_register_by_index(register_index);
        let multiplicand = self.get_ax_low();
        let result = (multplicator as u16) * (multiplicand as u16);
        self.ax = result;
        self.carry_flag = result > 0xFF;
        self.overflow_flag = result > 0xFF;

    }
}
