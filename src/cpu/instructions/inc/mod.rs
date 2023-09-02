use crate::{cpu::CPU, memory::Memory};

impl CPU {
    pub(in crate::cpu) fn execute_inc_word_register(&mut self, opcode: u8) {
        let register_index = opcode & 0x07;
        let value = self.get_16bit_register_by_index(register_index);
        let (value, overflow) = value.overflowing_add(1);
        self.set_16bit_register_by_index(register_index, value);
        self.zero_flag = value == 0;
        self.negative_flag = value & 0x8000 != 0;
        self.overflow_flag = overflow;
        self.auxiliary_carry_flag = value.count_ones() % 2 == 0;
        self.carry_flag = overflow;
    }

    pub(in crate::cpu) fn execute_inc_register_byte(&mut self, mem: &mut Memory) {
        let opcode = self.consume_instruction(mem);
        let register_index = opcode & 0x07;
        let value = self.get_8bit_register_by_index(register_index);
        let (value, _) = value.overflowing_add(1);
        self.set_8bit_register_by_index(register_index, value);
    }
}

#[cfg(test)]
mod test_16bit_inc {
    use crate::{cpu::CPU, generate_test, memory::Memory};

    generate_test!(
        inc_ax,
        (|cpu: &mut CPU, mem: &mut Memory| {
            cpu.ax = 0x0001;
            mem.write_byte(0xFFFC, 0x40);
        }),
        (|cpu: &CPU, _: &Memory| {
            assert_eq!(cpu.ax, 0x0002);
            assert_eq!(cpu.zero_flag, false);
            assert_eq!(cpu.negative_flag, false);
            assert_eq!(cpu.overflow_flag, false);
            assert_eq!(cpu.auxiliary_carry_flag, false);
            assert_eq!(cpu.carry_flag, false);
        })
    );

    // inc bx and test for overflow
    generate_test!(
        inc_bx_overflow,
        (|cpu: &mut CPU, mem: &mut Memory| {
            cpu.bx = 0xFFFF;
            mem.write_byte(0xFFFC, 0x43);
        }),
        (|cpu: &CPU, _: &Memory| {
            assert_eq!(cpu.bx, 0x0000);
            assert_eq!(cpu.zero_flag, true);
            assert_eq!(cpu.negative_flag, false);
            assert_eq!(cpu.overflow_flag, true);
            assert_eq!(cpu.auxiliary_carry_flag, true);
            assert_eq!(cpu.carry_flag, true);
        })
    );

    // inc cx and test for negative
    generate_test!(
        inc_cx_negative,
        (|cpu: &mut CPU, mem: &mut Memory| {
            cpu.cx = 0x7FFF;
            mem.write_byte(0xFFFC, 0x41);
        }),
        (|cpu: &CPU, _: &Memory| {
            assert_eq!(cpu.cx, 0x8000);
            assert_eq!(cpu.zero_flag, false);
            assert_eq!(cpu.negative_flag, true);
            assert_eq!(cpu.overflow_flag, false);
            assert_eq!(cpu.auxiliary_carry_flag, false);
            assert_eq!(cpu.carry_flag, false);
        })
    );

    generate_test!(
        inc_sp_auxiliary_carry,
        (|cpu: &mut CPU, mem: &mut Memory| {
            cpu.stack_pointer = 0x0FFF;
            mem.write_byte(0xFFFC, 0x44);
        }),
        (|cpu: &CPU, _: &Memory| {
            assert_eq!(cpu.stack_pointer, 0x1000);
            assert_eq!(cpu.zero_flag, false);
            assert_eq!(cpu.negative_flag, false);
            assert_eq!(cpu.overflow_flag, false);
            assert_eq!(cpu.auxiliary_carry_flag, false);
            assert_eq!(cpu.carry_flag, false);
        })
    );
}

#[cfg(test)]
mod test_8bit_inc {
    use crate::{cpu::CPU, generate_test, memory::Memory};

    generate_test!(
        inc_al,
        (|cpu: &mut CPU, mem: &mut Memory| {
            cpu.ax = 0x0001;
            mem.write_byte(0xFFFC, 0xFE);
            mem.write_byte(0xFFFD, 0xC0);
        }),
        (|cpu: &CPU, _: &Memory| {
            assert_eq!(cpu.ax, 0x0002);
        })
    );

    generate_test!(
        inc_dh,
        (|cpu: &mut CPU, mem: &mut Memory| {
            cpu.dx = 0x0100;
            mem.write_byte(0xFFFC, 0xFE);
            mem.write_byte(0xFFFD, 0xC6);
        }),
        (|cpu: &CPU, _: &Memory| {
            assert_eq!(cpu.dx, 0x0200);
        })
    );

    generate_test!(
        inc_cl,
        (|cpu: &mut CPU, mem: &mut Memory| {
            cpu.cx = 0x0001;
            mem.write_byte(0xFFFC, 0xFE);
            mem.write_byte(0xFFFD, 0xC1);
        }),
        (|cpu: &CPU, _: &Memory| {
            assert_eq!(cpu.cx, 0x0002);
        })
    );
}
