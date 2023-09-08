use crate::{consts::Byte, cpu::CPU, memory::Memory};

impl CPU {
    pub(in crate::cpu) fn execute_direct_mov_byte(&mut self, mem: &mut Memory, opcode: u8) {
        // read the data to be written i.e one byte as it's for 8bit register
        let write_data: Byte = self.consume_instruction(mem);
        // As the opcode is from 0xB0 to 0xB7, we can subtract 0xB0 from the opcode to get the
        // index to the register we need to write to
        let index: u8 = opcode - 0xB0; // opcode = 0xB0 (to) 0xB7
        self.set_8bit_register_by_index(index, write_data);
    }

    pub(in crate::cpu) fn execute_direct_mov_word(&mut self, mem: &mut Memory, opcode: u8) {
        // read the data to be written i.e two bytes
        let write_byte_low: Byte = self.consume_instruction(mem);
        let write_byte_high: Byte = self.consume_instruction(mem);
        let write_data: u16 = (write_byte_high as u16) << 8 | (write_byte_low as u16);
        // As the opcode is from 0xB8 to 0xBF, we can subtract 0xB8 from the opcode to get the index
        let index: u8 = opcode - 0xB8; // opcode = 0xB8 (to) 0xBF
        self.set_16bit_register_by_index(index, write_data);
    }
}

#[cfg(test)]
mod mov_registers_tests {
    use super::CPU;
    use crate::{generate_test, memory::Memory};

    // test al
    generate_test!(
        mov_al_0x12,
        (|cpu: &mut CPU, mem: &mut Memory| {
            cpu.write_instructions(mem, &[0xB0, 0x12]);
        }),
        (|cpu: &CPU, _| {
            assert_eq!(0x12, cpu.get_ax_low());
        })
    );

    // test cl
    generate_test!(
        mov_cl_0x12,
        (|cpu: &mut CPU, mem: &mut Memory| {
            cpu.write_instructions(mem, &[0xB1, 0x12]);
        }),
        (|cpu: &CPU, _| {
            assert_eq!(0x12, cpu.get_cx_low());
        })
    );

    //test for ch
    generate_test!(
        mov_ch_0x12,
        (|cpu: &mut CPU, mem: &mut Memory| {
            cpu.write_instructions(mem, &[0xB5, 0x12]);
        }),
        (|cpu: &CPU, _| {
            assert_eq!(0x12, cpu.get_cx_high());
        })
    );

    // test for ax
    generate_test!(
        mov_ax_0x1234,
        (|cpu: &mut CPU, mem: &mut Memory| {
            cpu.write_instructions(mem, &[0xB8, 0x12, 0x34]);
        }),
        (|cpu: &CPU, _| {
            assert_eq!(0x3412, cpu.ax);
        })
    );

    // test for bp
    generate_test!(
        mov_bp_0x1234,
        (|cpu: &mut CPU, mem: &mut Memory| {
            cpu.write_instructions(mem, &[0xBD, 0x12, 0x34]);
        }),
        (|cpu: &CPU, _| {
            assert_eq!(0x3412, cpu.base_pointer);
        })
    );
}
