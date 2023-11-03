use crate::{
    consts::{Byte, Word},
    cpu::CPU,
    memory::Memory,
};

// Immediate Addressing
impl CPU {
    pub(in crate::cpu) fn add_ax_in_immediate_addressing(&mut self, mem: &Memory) {
        let data_low = self.consume_instruction(mem);
        let data_high = self.consume_instruction(mem);
        let data = ((data_high as Word) << 8) | (data_low as Word);
        let (result, _) = self.add_16bit_with_overflow_and_set_flags(self.ax, data);
        self.ax = result;
    }

    pub(in crate::cpu) fn add_al_in_immediate_addressing(&mut self, mem: &Memory) {
        let data = self.consume_instruction(mem);
        let (result, _) = self.add_8bit_with_overflow_and_set_flags(self.ax as u8, data);
        self.ax = (self.ax & 0xFF00) | (result as Word);
    }

    pub(in crate::cpu) fn execute_add_immediate_byte(&mut self, mem: &Memory) {
        let instruction = self.consume_instruction(mem);
        match instruction {
            0xC1..=0xC7 => {
                let index = instruction & 0x07;
                let data = self.consume_instruction(mem);
                let (result, _) = self.add_8bit_with_overflow_and_set_flags(
                    self.get_8bit_register_by_index(index),
                    data,
                );
                self.set_8bit_register_by_index(index, result);
            }
            x => panic!("ADD instruction not implemented! for {}", x),
        }
    }

    fn get_data_add(&mut self, mem: &Memory, instruction: Byte) -> Word {
        match instruction {
            0x81 => self.consume_word(mem),
            0x83 => self.consume_byte(mem) as Word,
            x => panic!("ADD instruction not implemented! for {}", x),
        }
    }

    pub(in crate::cpu) fn execute_add_reg_immediate_word(
        &mut self,
        mem: &Memory,
        instruction: Byte,
    ) {
        let index = self.consume_instruction(mem) & 0x07;
        let data = self.get_data_add(mem, instruction);
        let (result, _) = self
            .add_16bit_with_overflow_and_set_flags(self.get_16bit_register_by_index(index), data);
        self.set_16bit_register_by_index(index, result);
    }
}

#[cfg(test)]
mod add_immediate_16bit_tests {
    use crate::cpu::instructions::test_macro::execute_code;

    #[test]
    fn add_ax_ax_no_overflow() {
        let code = "MOV AX, 0x1234 \n ADD AX, 0x1234";
        let (cpu, _) = execute_code(code);
        assert_eq!(cpu.ax, 0x2468);
        assert_eq!(cpu.get_flags_as_binary(), 0b0000_0000);
    }

    #[test]
    fn add_ax_ax_overflow() {
        let code = "MOV AX, 0xFFFF \n ADD AX, 0xFFFF";
        let (cpu, _) = execute_code(code);
        assert_eq!(cpu.ax, 0xFFFE);
        assert_eq!(cpu.get_flags_as_binary(), 0b0010_0101);
    }

    #[test]
    fn add_ax_ax_zero() {
        let code = "MOV AX, 0x0000 \n ADD AX, 0x0000";
        let (cpu, _) = execute_code(code);
        assert_eq!(cpu.ax, 0x0000);
        assert_eq!(cpu.get_flags_as_binary(), 0b0001_0010);
    }

    #[test]
    fn add_ax_ax_negative() {
        let code = "MOV AX, 0x0001 \n ADD AX, 0xFFFF";
        let (cpu, _) = execute_code(code);
        assert_eq!(cpu.ax, 0x0000);
        assert_eq!(cpu.get_flags_as_binary(), 0b0011_0011);
    }

    #[test]
    fn add_bx_0x1234() {
        let code = "MOV BX, 0x0001 \n ADD BX, 0x1234";
        let (cpu, _) = execute_code(code);
        assert_eq!(cpu.bx, 0x1235);
        assert_eq!(cpu.get_flags_as_binary(), 0b0001_0000);
    }

    #[test]
    fn add_bx_0xffff_overflow() {
        let code = "MOV BX, 0xFFFF \n ADD BX, 0xFFFF";
        let (cpu, _) = execute_code(code);
        assert_eq!(cpu.bx, 0xFFFE);
        assert_eq!(cpu.get_flags_as_binary(), 0b0010_0101);
    }

    #[test]
    fn add_bx_0xffee() {
        let code = "MOV BX, 0xFF01 \n ADD BX, 0xFFEE";
        let (cpu, _) = execute_code(code);
        assert_eq!(cpu.bx, 0xFEEF);
        assert_eq!(cpu.get_flags_as_binary(), 0b0000_0101);
    }
}

#[cfg(test)]
mod add_immediate_8bit_tests {
    use crate::cpu::instructions::test_macro::execute_code;

    #[test]
    fn add_al_0x12() {
        let code = "MOV AL, 0x01 \n ADD AL, 0x12";
        let (cpu, _) = execute_code(code);
        assert_eq!(cpu.ax, 0x13);
        assert_eq!(cpu.get_flags_as_binary(), 0b0000_0000);
    }

    #[test]
    fn add_al_0xff_overflow() {
        let code = "MOV AL, 0xFE \n ADD AL, 0xFF";
        let (cpu, _) = execute_code(code);
        assert_eq!(cpu.ax, 0xFD);
        assert_eq!(cpu.get_flags_as_binary(), 0b0010_0101);
    }

    #[test]
    fn add_cl_0x12() {
        let code = "MOV CL, 0x01 \n ADD CL, 0x12";
        let (cpu, _) = execute_code(code);
        assert_eq!(cpu.cx, 0x13);
        assert_eq!(cpu.get_flags_as_binary(), 0b0000_0000);
    }

    #[test]
    fn add_cl_0xff_overflow() {
        let code = "MOV CL, 0xFE \n ADD CL, 0xFF";
        let (cpu, _) = execute_code(code);
        assert_eq!(cpu.cx, 0xFD);
        assert_eq!(cpu.get_flags_as_binary(), 0b0010_0101);
    }

    #[test]
    fn add_bh_0x12() {
        let code = "MOV BX, 0xFF01 \n ADD BH, 0x12";
        let (cpu, _) = execute_code(code);
        assert_eq!(cpu.bx, 0x1101);
        assert_eq!(cpu.get_flags_as_binary(), 0b0011_0001);
    }

    #[test]
    fn add_bh_0xff_overflow() {
        let code = "MOV BX, 0xFF01 \n ADD BH, 0xFF";
        let (cpu, _) = execute_code(code);
        assert_eq!(cpu.bx, 0xFE01);
        assert_eq!(cpu.get_flags_as_binary(), 0b0010_0101);
    }
}
