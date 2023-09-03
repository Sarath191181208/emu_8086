use crate::{cpu::CPU, memory::Memory};

impl CPU {
    pub(in crate::cpu) fn execute_inc_word_register(&mut self, opcode: u8) {
        let register_index = opcode & 0x07;
        let value = self.get_16bit_register_by_index(register_index);
        let (value, _) = self.add_16bit_with_overflow_and_set_flags(value, 0x0001);
        self.set_16bit_register_by_index(register_index, value);
    }

    pub(in crate::cpu) fn execute_inc_register_byte(&mut self, mem: &mut Memory) {
        let opcode = self.consume_instruction(mem);
        let register_index = opcode & 0x07;
        let value = self.get_8bit_register_by_index(register_index);
        let (value, _) = self.add_8bit_with_overflow_and_set_flags(value, 0x01);
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
            assert_eq!(cpu.get_flags_as_binary(), 0b00000000);
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
            assert!(cpu.zero_flag);
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
            assert_eq!(cpu.get_flags_as_binary(), 0b00111100);
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
            assert_eq!(cpu.get_flags_as_binary(), 0b00110000)
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
            assert_eq!(cpu.get_flags_as_binary(), 0b00000000);
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
            assert_eq!(cpu.get_flags_as_binary(), 0b00000000);
        })
    );

    generate_test!(
        inc_cl,
        (|cpu: &mut CPU, mem: &mut Memory| {
            cpu.cx = 0x0002;
            mem.write_byte(0xFFFC, 0xFE);
            mem.write_byte(0xFFFD, 0xC1);
        }),
        (|cpu: &CPU, _: &Memory| {
            assert_eq!(cpu.cx, 0x0003);
            assert_eq!(cpu.get_flags_as_binary(), 0b00010000);
        })
    );
}
