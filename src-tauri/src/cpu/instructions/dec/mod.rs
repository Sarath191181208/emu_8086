use crate::{cpu::CPU, memory::Memory};

impl CPU {
    fn dec_from_16bitvalue_and_set_flags(&mut self, value: u16) -> u16 {
        let (val, _) = self.sub_16bit_with_overflow_and_set_flags(value, 1);
        val
    }

    fn dec_from_8bitvalue_and_set_flags(&mut self, value: u8) -> u8 {
        let (val, _) = self.sub_8bit_with_overflow_and_set_flags(value, 1);
        val
    }

    pub(in crate::cpu) fn execute_dec_word_register(&mut self, opcode: u8) {
        let register_index = (opcode & 0x0F) - 8;
        let value = self.get_16bit_register_by_index(register_index);
        let value = self.dec_from_16bitvalue_and_set_flags(value);
        self.set_16bit_register_by_index(register_index, value);
    }

    pub(in crate::cpu) fn execute_dec_register_byte(&mut self, mem: &mut Memory) {
        let opcode = self.consume_instruction(mem);
        let register_index = (opcode & 0x0F) - 8;
        let value = self.get_8bit_register_by_index(register_index);
        let value = self.dec_from_8bitvalue_and_set_flags(value);
        self.set_8bit_register_by_index(register_index, value);
    }

    pub(in crate::cpu) fn execute_dec_address_16bit(&mut self, mem: &mut Memory) {
        self.consume_instruction(mem); // 0x0E
        let address = self.consume_word(mem);
        let value = self.read_word_from_pointer(mem, address);
        let value = self.dec_from_16bitvalue_and_set_flags(value);
        self.write_word_from_pointer(mem, address, value);
    }

    pub(in crate::cpu) fn execute_dec_address_8bit(&mut self, mem: &mut Memory) {
        self.consume_instruction(mem); // 0x0E
        let address = self.consume_word(mem);
        let value = self.read_byte_from_pointer(mem, address);
        let value = self.dec_from_8bitvalue_and_set_flags(value);
        self.write_byte_from_pointer(mem, address, value);
    }
}

#[cfg(test)]
mod test_16bit_dec {
    use crate::{
        cpu::{instructions::test_macro::compile_and_test_str, CPU},
        generate_test,
        memory::Memory,
    };

    generate_test!(
        dec_ax,
        (|cpu: &mut CPU, mem: &mut Memory| {
            cpu.ax = 0x0001;
            cpu.write_instructions(mem, &[0x48]);
        }),
        (|cpu: &CPU, _: &Memory| {
            assert_eq!(cpu.ax, 0x0000);
            assert_eq!(cpu.get_flags_as_binary(), 0b0001_0010);
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
            assert_eq!(cpu.get_flags_as_binary(), 0b0000_0100);
        })
    );

    #[test]
    fn test_dec_address_16bit() {
        compile_and_test_str(
            "
            org 0x100
            .data
            var dw 0x0001
            code: 
            DEC var
            ",
            2,
            |cpu: &CPU, mem: &Memory| {
                assert_eq!(cpu.read_word_from_pointer(mem, 0x102), 0x0000);
                assert_eq!(cpu.get_flags_as_binary(), 0b0001_0010);
            },
        );
    }
}

#[cfg(test)]
mod test_8bit_dec {
    use crate::{
        cpu::{instructions::test_macro::compile_and_test_str, CPU},
        generate_test,
        memory::Memory,
    };

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

    #[test]
    fn test_dec_address_8bit() {
        compile_and_test_str(
            "
            org 0x100
            .data
            var db 0x01
            code: 
            DEC var
            ",
            2,
            |cpu: &CPU, mem: &Memory| {
                assert_eq!(cpu.read_byte_from_pointer(mem, 0x102), 0x00);
                assert_eq!(cpu.get_flags_as_binary(), 0b0001_0010);
            },
        );
    }
}
