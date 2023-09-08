use crate::{cpu::CPU, memory::Memory};

impl CPU {
    pub(in crate::cpu) fn execute_dec_word_register(&mut self, opcode: u8) {
        let register_index = (opcode & 0x0F) - 8;
        let value = self.get_16bit_register_by_index(register_index);
        let (value, overflow) = value.overflowing_sub(1);
        self.set_16bit_register_by_index(register_index, value);
        self.zero_flag = value == 0;
        self.negative_flag = value & 0x8000 != 0;
        self.overflow_flag = overflow;
        self.auxiliary_carry_flag = value.count_ones() % 2 == 0;
        self.carry_flag = overflow;
    }

    pub(in crate::cpu) fn execute_dec_register_byte(&mut self, mem: &mut Memory) {
        let opcode = self.consume_instruction(mem);
        let register_index = (opcode & 0x0F) - 8;
        let value = self.get_8bit_register_by_index(register_index);
        let (value, _) = value.overflowing_sub(1);
        self.set_8bit_register_by_index(register_index, value);
    }
}

#[cfg(test)]
mod test_16bit_dec {
    use crate::{cpu::CPU, generate_test, memory::Memory};

    generate_test!(
        dec_ax,
        (|cpu: &mut CPU, mem: &mut Memory| {
            cpu.ax = 0x0001;
            cpu.write_instructions(mem, &[0x48]);
        }),
        (|cpu: &CPU, _: &Memory| {
            assert_eq!(cpu.ax, 0x0000);
            assert!(cpu.zero_flag);
            assert!(!cpu.negative_flag);
            assert!(!cpu.overflow_flag);
            assert!(cpu.auxiliary_carry_flag);
            assert!(!cpu.carry_flag);
        })
    );

    // dec bx and test for overflow
    generate_test!(
        dec_bx_overflow,
        (|cpu: &mut CPU, mem: &mut Memory| {
            cpu.bx = 0xFFFF;
            cpu.write_instructions(mem, &[0x4B]);
        }),
        (|cpu: &CPU, _: &Memory| {
            assert_eq!(cpu.bx, 0xFFFE);
            assert!(!cpu.zero_flag);
            assert!(cpu.negative_flag);
            assert!(!cpu.overflow_flag);
            assert!(!cpu.auxiliary_carry_flag);
            assert!(!cpu.carry_flag);
        })
    );
}

#[cfg(test)]
mod test_8bit_dec {
    use crate::{cpu::CPU, generate_test, memory::Memory};

    generate_test!(
        dec_al,
        (|cpu: &mut CPU, mem: &mut Memory| {
            cpu.ax = 0x0001;
            cpu.write_instructions(mem, &[0xFE, 0xC8]);
        }),
        (|cpu: &CPU, _: &Memory| {
            assert_eq!(cpu.ax, 0x0000);
        })
    );

    generate_test!(
        dec_dh,
        (|cpu: &mut CPU, mem: &mut Memory| {
            cpu.dx = 0x0100;
            cpu.write_instructions(mem, &[0xFE, 0xCE]);
        }),
        (|cpu: &CPU, _: &Memory| {
            assert_eq!(cpu.dx, 0x0000);
        })
    );
}
