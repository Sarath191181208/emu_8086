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

#[cfg(test)]
mod test_16bit_mul {
    use crate::{compiler::compile_lines, cpu::CPU, generate_test_with_cycles, memory::Memory};

    generate_test_with_cycles!(
        mul_ax_bx,
        (|cpu: &mut CPU, mem: &mut Memory| {
            let (compiled_bytes, _) = compile_lines(
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
            let (compiled_bytes, _) = compile_lines(
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
}

#[cfg(test)]
mod test_8bit_mul {
    use crate::{compiler::compile_lines, cpu::CPU, generate_test_with_cycles, memory::Memory};

    generate_test_with_cycles!(
        mul_ax_bl,
        (|cpu: &mut CPU, mem: &mut Memory| {
            let (compiled_bytes, _) = compile_lines(
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
            let (compiled_bytes, _) = compile_lines(
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
