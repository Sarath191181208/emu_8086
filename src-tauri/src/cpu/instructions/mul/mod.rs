use crate::{cpu::CPU, memory::Memory};

impl CPU {
    fn mul_16bit_set_flags_regs(&mut self, multplicator: u16, multiplicand: u16) {
        let result = (multplicator as u32) * (multiplicand as u32);
        self.ax = (result & 0x0000_FFFF) as u16;
        self.dx = ((result & 0xFFFF_0000) >> 16) as u16;
        self.carry_flag = result > 0xFFFF;
        self.overflow_flag = result > 0xFFFF;
    }

    fn mul_8bit_set_flags_regs(&mut self, multplicator: u8, multiplicand: u8) {
        let result = (multplicator as u16) * (multiplicand as u16);
        self.ax = result;
        self.carry_flag = result > 0xFF;
        self.overflow_flag = result > 0xFF;
    }

    pub(in crate::cpu) fn execute_mul_16bit(&mut self, mem: &mut Memory) {
        let opcode = self.consume_instruction(mem);
        let register_index = opcode & 0x07;
        let multplicator = self.get_16bit_register_by_index(register_index);
        self.mul_16bit_set_flags_regs(multplicator, self.ax);
    }
    pub(in crate::cpu) fn execute_mul_8bit(&mut self, mem: &mut Memory) {
        let opcode = self.consume_instruction(mem);
        let register_index = opcode & 0x07;
        let multplicator = self.get_8bit_register_by_index(register_index);
        let multiplicand = self.get_ax_low();
        self.mul_8bit_set_flags_regs(multplicator, multiplicand);
    }

    pub(in crate::cpu) fn execute_mul_address_8bit(&mut self, mem: &mut Memory) {
        self.consume_instruction(mem); // 0x26
        let address = self.consume_word(mem);
        let multiplicand = self.get_ax_low();
        let multiplicator = self.read_byte_from_pointer(mem, address);
        self.mul_8bit_set_flags_regs(multiplicator, multiplicand);
    }

    pub(in crate::cpu) fn execute_mul_address_16bit(&mut self, mem: &mut Memory) {
        self.consume_instruction(mem); // 0x26
        let address = self.consume_word(mem);
        let multiplicand = self.ax;
        let multiplicator = self.read_word_from_pointer(mem, address);
        self.mul_16bit_set_flags_regs(multiplicator, multiplicand);
    }
}

#[cfg(test)]
mod test_16bit_mul {
    use crate::{
        compiler::compile_lines,
        cpu::{instructions::test_macro::compile_and_test_str, CPU},
        generate_test_with_cycles,
        memory::Memory,
    };

    generate_test_with_cycles!(
        mul_ax_bx,
        (|cpu: &mut CPU, mem: &mut Memory| {
            let (compiled_bytes, _, _) = compile_lines(
                "
            MOV AX, 0x1111
            MOV BX, 0x1010
            MUL BX",
                false,
            )
            .unwrap();
            cpu.write_instructions(mem, &compiled_bytes);
        }),
        (|cpu: &CPU, _: &Memory| {
            assert_eq!(cpu.ax, 0x2110);
            assert_eq!(cpu.dx, 0x0112);
            assert_eq!(cpu.get_flags_as_binary(), 0b0000_1001);
        }),
        3
    );

    generate_test_with_cycles!(
        mul_ax_cx,
        (|cpu: &mut CPU, mem: &mut Memory| {
            let (compiled_bytes, _, _) = compile_lines(
                "
            MOV AX, 0x0011
            MOV CX, 0x0010
            MUL CX",
                false,
            )
            .unwrap();
            cpu.write_instructions(mem, &compiled_bytes);
        }),
        (|cpu: &CPU, _: &Memory| {
            assert_eq!(cpu.ax, 0x0110);
            assert_eq!(cpu.dx, 0x0000);
            assert_eq!(cpu.get_flags_as_binary(), 0b0000_0000);
        }),
        3
    );

    #[test]
    fn mul_var() {
        compile_and_test_str(
            "
                org 100h
                .data 
                var dw 0x0010
                code:
                mov ax, 0x0011
                MUL var
                ",
            3,
            |cpu: &CPU, _: &Memory| {
                assert_eq!(cpu.ax, 0x0110);
                assert_eq!(cpu.dx, 0x0000);
                assert_eq!(cpu.get_flags_as_binary(), 0b0000_0000);
            },
        );
    }
}

#[cfg(test)]
mod test_8bit_mul {
    use crate::{compiler::compile_lines, cpu::CPU, generate_test_with_cycles, memory::Memory};

    generate_test_with_cycles!(
        mul_ax_bl,
        (|cpu: &mut CPU, mem: &mut Memory| {
            let (compiled_bytes, _, _) = compile_lines(
                "
            MOV AX, 0x11
            MOV BL, 0x10
            MUL BL",
                false,
            )
            .unwrap();
            cpu.write_instructions(mem, &compiled_bytes);
        }),
        (|cpu: &CPU, _: &Memory| {
            assert_eq!(cpu.ax, 0x110);
            assert_eq!(cpu.get_flags_as_binary(), 0b0000_1001);
        }),
        3
    );

    generate_test_with_cycles!(
        mul_ax_cl,
        (|cpu: &mut CPU, mem: &mut Memory| {
            let (compiled_bytes, _, _) = compile_lines(
                "
            MOV AX, 0x01
            MOV CL, 0x00
            MUL CL",
                false,
            )
            .unwrap();
            cpu.write_instructions(mem, &compiled_bytes);
        }),
        (|cpu: &CPU, _: &Memory| {
            assert_eq!(cpu.ax, 0);
            assert_eq!(cpu.get_flags_as_binary(), 0b0000_0000);
        }),
        3
    );
}
