use crate::{
    consts::{Byte, Word},
    cpu::CPU,
    memory::Memory,
};

// Immediate Addressing
impl CPU {
    pub(in crate::cpu) fn sub_ax_in_immediate_addressing(&mut self, mem: &Memory) {
        let data_low = self.consume_instruction(mem);
        let data_high = self.consume_instruction(mem);
        let data = ((data_high as Word) << 8) | (data_low as Word);
        let (result, _) = self.sub_16bit_with_overflow_and_set_flags(self.ax, data);
        self.ax = result;
    }

    pub(in crate::cpu) fn sub_al_in_immediate_addressing(&mut self, mem: &Memory) {
        let data = self.consume_instruction(mem);
        let (result, _) = self.sub_8bit_with_overflow_and_set_flags(self.ax as u8, data);
        self.ax = (self.ax & 0xFF00) | (result as Word);
    }

    pub(in crate::cpu) fn execute_sub_immediate_byte(&mut self, mem: &Memory) {
        let instruction = self.consume_instruction(mem);
        let index = (instruction - 8) & 0x07;
        let data = self.consume_instruction(mem);
        let (result, _) =
            self.sub_8bit_with_overflow_and_set_flags(self.get_8bit_register_by_index(index), data);
        self.set_8bit_register_by_index(index, result);
    }

    fn get_data_sub(&mut self, mem: &Memory, instruction: Byte) -> Word {
        match instruction {
            0x81 => {
                let data_low = self.consume_instruction(mem);
                let data_high = self.consume_instruction(mem);
                ((data_high as Word) << 8) | (data_low as Word)
            }
            0x83 => {
                let data_low = self.consume_instruction(mem);
                0xFF << 8 | (data_low as Word)
            }
            x => panic!("SUB instruction not implemented! for {}", x),
        }
    }

    fn sub_immediate_word(&mut self, instruction: Byte, mem: &Memory) {
        let index = (self.consume_instruction(mem) - 8) & 0x07;
        let data = self.get_data_sub(mem, instruction);
        let (result, _) = self
            .sub_16bit_with_overflow_and_set_flags(self.get_16bit_register_by_index(index), data);
        self.set_16bit_register_by_index(index, result);
    }

    pub(in crate::cpu) fn execute_sub_immediate_word(&mut self, mem: &Memory, instruction: Byte) {
        match instruction {
            0x81 | 0x83 => self.sub_immediate_word(instruction, mem),
            x => panic!("SUB instruction not implemented! for {}", x),
        }
    }
}

#[cfg(test)]
mod sub_immediate_16bit_tests {
    use crate::{cpu::CPU, generate_test, memory::Memory};

    // test ax - 0x1234
    generate_test!(
        sub_ax_0x1234,
        (|cpu: &mut CPU, mem: &mut Memory| {
            cpu.instruction_pointer = 0xFFFB;
            cpu.write_instructions(mem, &[0x2D, 0x34, 0x12]);
            cpu.ax = 0x1235;
        }),
        (|cpu: &CPU, _: &Memory| {
            assert_eq!(cpu.ax, 0x0001);
            assert_eq!(cpu.get_flags_as_binary(), 0b00000000);
        })
    );

    // test bx - dx
    generate_test!(
        sub_bx_dx,
        (|cpu: &mut CPU, mem: &mut Memory| {
            cpu.instruction_pointer = 0xFFFB;
            cpu.write_instructions(mem, &[0x2B, 0xDA]);
            cpu.bx = 0x1235;
            cpu.dx = 0x1234;
        }),
        (|cpu: &CPU, _: &Memory| {
            assert_eq!(cpu.bx, 0x0001);
            assert_eq!(cpu.get_flags_as_binary(), 0b00000000);
        })
    );

    // test sp - cx and overflow
    generate_test!(
        sub_sp_cx_overflow,
        (|cpu: &mut CPU, mem: &mut Memory| {
            cpu.instruction_pointer = 0xFFFB;
            cpu.write_instructions(mem, &[0x2B, 0xE1]);
            cpu.stack_pointer = 0x1234;
            cpu.cx = 0x1235;
        }),
        (|cpu: &CPU, _: &Memory| {
            assert_eq!(cpu.stack_pointer, 0xFFFF);
            assert_eq!(cpu.get_flags_as_binary(), 0b00110101);
        })
    );
}

#[cfg(test)]
mod sub_immediate_8bit_tests {
    use crate::{
        cpu::{instructions::test_macro::compile_and_test_str, CPU},
        memory::Memory,
    };

    #[test]
    fn sub_al_0x12_overflow() {
        compile_and_test_str(
            "
            MOV AL, 0x01
            SUB AL, 0x12
            ",
            2,
            |cpu: &CPU, _: &Memory| {
                assert_eq!(cpu.ax, 0x00EF);
                assert_eq!(cpu.get_flags_as_binary(), 0b0010_0101);
            },
        );
    }

    #[test]
    fn sub_dl_0x12() {
        compile_and_test_str(
            "
            MOV DL, 0x21
            SUB DL, 0x12
            ",
            2,
            |cpu: &CPU, _: &Memory| {
                assert_eq!(cpu.dx, 0x000F);
                assert_eq!(cpu.get_flags_as_binary(), 0b0011_0000);
            },
        );
    }
}
