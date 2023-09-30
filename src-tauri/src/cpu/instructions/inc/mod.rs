use crate::{cpu::CPU, memory::Memory};

impl CPU {

    pub(in crate::cpu) fn execute_inc_word_register(&mut self, opcode: u8) {
        let register_index = opcode & 0x07;
        let value = self.get_16bit_register_by_index(register_index);
        let value = self.inc_from_16bitvalue_and_set_flags(value);
        self.set_16bit_register_by_index(register_index, value);
    }

    pub(in crate::cpu) fn execute_inc_register_byte(&mut self, mem: &mut Memory) {
        let opcode = self.consume_instruction(mem);
        let register_index = opcode & 0x07;
        let value = self.get_8bit_register_by_index(register_index);
        let value = self.inc_from_8bitvalue_and_set_flags(value);
        self.set_8bit_register_by_index(register_index, value);
    }

    pub(in crate::cpu) fn execute_inc_address_16bit(&mut self, mem: &mut Memory) {
        self.consume_instruction(mem); // 0x06
        let address = self.consume_word(mem);
        let value = self.read_word_from_pointer(mem, address);
        let value = self.inc_from_16bitvalue_and_set_flags(value);
        self.write_word_from_pointer(mem, address, value);
    }

    pub(in crate::cpu) fn execute_inc_address_8bit(&mut self, mem: &mut Memory) {
        self.consume_instruction(mem); // 0x06
        let address = self.consume_word(mem);
        let value = self.read_byte_from_pointer(mem, address);
        let value = self.inc_from_8bitvalue_and_set_flags(value);
        self.write_byte_from_pointer(mem, address, value);
    }
}

#[cfg(test)]
mod test_16bit_inc {
    use crate::{
        cpu::{instructions::test_macro::compile_and_test_str, CPU},
        generate_test,
        memory::Memory,
    };

    generate_test!(
        inc_ax,
        (|cpu: &mut CPU, mem: &mut Memory| {
            cpu.ax = 0x0001;
            cpu.write_instructions(mem, &[0x40]);
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
            cpu.write_instructions(mem, &[0x43]);
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
            cpu.write_instructions(mem, &[0x41]);
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
            cpu.write_instructions(mem, &[0x44]);
        }),
        (|cpu: &CPU, _: &Memory| {
            assert_eq!(cpu.stack_pointer, 0x1000);
            assert_eq!(cpu.get_flags_as_binary(), 0b00110000)
        })
    );

    #[test]
    fn test_inc_address_16bit() {
        compile_and_test_str(
            "
            org 0x100
            .data
            var dw 0xFFFF
            code: 
            inc var
            ",
            2,
            |cpu: &CPU, mem: &Memory| {
                assert_eq!(cpu.read_word_from_pointer(mem, 0x102), 0x0000);
                assert_eq!(cpu.get_flags_as_binary(), 0b0011_0010);
            },
        );
    }
}

#[cfg(test)]
mod test_8bit_inc {
    use crate::{
        cpu::{instructions::test_macro::compile_and_test_str, CPU},
        generate_test,
        memory::Memory,
    };

    generate_test!(
        inc_al,
        (|cpu: &mut CPU, mem: &mut Memory| {
            cpu.ax = 0x0001;
            cpu.write_instructions(mem, &[0xFE, 0xC0]);
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
            cpu.write_instructions(mem, &[0xFE, 0xC6]);
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
            cpu.write_instructions(mem, &[0xFE, 0xC1]);
        }),
        (|cpu: &CPU, _: &Memory| {
            assert_eq!(cpu.cx, 0x0003);
            assert_eq!(cpu.get_flags_as_binary(), 0b00010000);
        })
    );

    #[test]
    fn test_dec_address_8bit() {
        compile_and_test_str(
            "
            org 0x100
            .data
            var db 0x01
            code: 
            inc var
            ",
            2,
            |cpu: &CPU, mem: &Memory| {
                assert_eq!(cpu.read_byte_from_pointer(mem, 0x102), 0x02);
                assert_eq!(cpu.get_flags_as_binary(), 0x00);
            },
        );
    }
}
