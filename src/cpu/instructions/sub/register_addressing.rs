use crate::{consts::Byte, cpu::CPU, memory::Memory};

// Register Addressing
impl CPU {
    fn sub_8bit_register_addressing(&mut self, instruction: Byte) {
        let (source_index, write_index) = self.get_index_from_c0_ff_pattern(instruction);
        let reg = self.get_8bit_register_by_index(source_index % 8);
        let write_reg = self.get_8bit_register_by_index(write_index);
        let (result, _) = self.sub_8bit_with_overflow_and_set_flags(write_reg, reg);
        self.set_8bit_register_by_index(write_index, result);
    }

    pub(in crate::cpu) fn execute_sub_register_byte(&mut self, mem: &Memory) {
        let instruction = self.consume_instruction(mem);
        match instruction {
            0xC0..=0xFF => {
                self.sub_8bit_register_addressing(instruction);
            }
            x => panic!("SUB instruction not implemented! for {}", x),
        }
    }

    fn sub_16bit_register_addressing(&mut self, instruction: Byte) {
        let (source_index, write_index) = self.get_index_from_c0_ff_pattern(instruction);
        let reg = self.get_16bit_register_by_index(source_index % 8);
        let write_reg = self.get_16bit_register_by_index(write_index);
        let (result, _) = self.sub_16bit_with_overflow_and_set_flags(write_reg, reg);
        self.set_16bit_register_by_index(write_index, result);
    }

    pub(in crate::cpu) fn execute_sub_register_word(&mut self, mem: &Memory) {
        let instruction = self.consume_instruction(mem);
        match instruction {
            0xC1..=0xFF => {
                self.sub_16bit_register_addressing(instruction);
            }
            x => panic!("SUB instruction not implemented! for {}", x),
        }
    }
}

mod sub_16bit_register_addressing {
    use crate::{cpu::CPU, generate_test, memory::Memory};

    // sub ax, cx
    generate_test!(
                test_sub_ax_cx,
        (|cpu: &mut CPU, mem: &mut Memory| {
            cpu.ax = 0x0f0f;
            cpu.cx = 0x0013;
            mem.write_byte(0xFFFC, 0x2B);
            mem.write_byte(0xFFFD, 0xC1);
        }),
        (|cpu: &CPU, _: &Memory| {
            assert_eq!(cpu.ax, 0x0EFC);
            assert_eq!(cpu.get_flags_as_binary(), 0b00010000)
        })
    );
    // sub bx, dx overflow 
    generate_test!(
        test_sub_bx_dx_overflow,
        (|cpu: &mut CPU, mem: &mut Memory| {
            cpu.bx = 0x0000;
            cpu.dx = 0x0013;
            mem.write_byte(0xFFFC, 0x2B);
            mem.write_byte(0xFFFD, 0xDA);
        }),
        (|cpu: &CPU, _: &Memory| {
            assert_eq!(cpu.bx, 0xFFED);
            assert_eq!(cpu.get_flags_as_binary(), 0b000110101)
        })   
    );
    // sub sp, bp 
    generate_test!(
        test_sub_sp_bp,
        (|cpu: &mut CPU, mem: &mut Memory| {
            cpu.stack_pointer = 0xF000;
            cpu.base_pointer = 0x0013;
            mem.write_byte(0xFFFC, 0x2B);
            mem.write_byte(0xFFFD, 0xE5);
        }),
        (|cpu: &CPU, _: &Memory| {
            assert_eq!(cpu.stack_pointer, 0xEFED);
            assert_eq!(cpu.get_flags_as_binary(), 0b00110100)
        })   
    );
}

mod sub_8bit_register_addressing{
    use crate::{cpu::CPU, generate_test, memory::Memory};

    // sub al, cl
    generate_test!(
        test_sub_al_cl,
        (|cpu: &mut CPU, mem: &mut Memory| {
            cpu.set_ax_low(0x0f);
            cpu.set_cx_low(0x13);
            mem.write_byte(0xFFFC, 0x2A);
            mem.write_byte(0xFFFD, 0xC1);
        }),
        (|cpu: &CPU, _: &Memory| {
            assert_eq!(cpu.get_ax_low(), 0xFC);
            assert_eq!(cpu.get_flags_as_binary(), 0b00010101)
        })
    );

    // sub bl, dh overflow
    generate_test!(
        test_sub_bl_dl_overflow,
        (|cpu: &mut CPU, mem: &mut Memory| {
            cpu.set_bx_low(0xf0);
            cpu.set_dx_high(0x13);
            mem.write_byte(0xFFFC, 0x2A);
            mem.write_byte(0xFFFD, 0xDE);
        }),
        (|cpu: &CPU, _: &Memory| {
            assert_eq!(cpu.get_bx_low(), 0xDD);
            assert_eq!(cpu.get_flags_as_binary(), 0b00110100)
        })
    );

}