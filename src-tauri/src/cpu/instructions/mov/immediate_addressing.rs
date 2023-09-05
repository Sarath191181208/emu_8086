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
mod mov_16bit_register_addressing_tests {
    use super::CPU;
    use crate::memory::Memory;

    macro_rules! generate_test {
        ($test_name:ident, $instructions:expr, $expected:expr, $compare: expr) => {
            paste::item! {
                #[test]
                fn [<test_ $test_name>]() {
                    let mut cpu = CPU::new();
                    let mut mem = Memory::new();
                    cpu.reset(&mut mem);

                    $instructions(&mut mem);

                    cpu.execute(&mut mem);

                    assert_eq!($expected, $compare(&cpu));
                }
            }
        };
    }

    // test al
    generate_test!(
        mov_al_0x12,
        (|mem: &mut Memory| {
            mem.write_byte(0xFFFC, 0xB0);
            mem.write_byte(0xFFFD, 0x12);
        }),
        0x12,
        (|cpu: &CPU| cpu.get_ax_low())
    );

    // test bl
    generate_test!(
        mov_bl_0x12,
        (|mem: &mut Memory| {
            mem.write_byte(0xFFFC, 0xB3);
            mem.write_byte(0xFFFD, 0x12);
        }),
        0x12,
        (|cpu: &CPU| cpu.get_bx_low())
    );

    // test cl
    generate_test!(
        mov_cl_0x12,
        (|mem: &mut Memory| {
            mem.write_byte(0xFFFC, 0xB1);
            mem.write_byte(0xFFFD, 0x12);
        }),
        0x12,
        (|cpu: &CPU| cpu.get_cx_low())
    );

    // test ah
    generate_test!(
        mov_ah_0x12,
        (|mem: &mut Memory| {
            mem.write_byte(0xFFFC, 0xB4);
            mem.write_byte(0xFFFD, 0x12);
        }),
        0x12,
        (|cpu: &CPU| cpu.get_ax_high())
    );

    // test for bh
    generate_test!(
        mov_bh_0x12,
        (|mem: &mut Memory| {
            mem.write_byte(0xFFFC, 0xB7);
            mem.write_byte(0xFFFD, 0x12);
        }),
        0x12,
        (|cpu: &CPU| cpu.get_bx_high())
    );

    //test for ch
    generate_test!(
        mov_ch_0x12,
        (|mem: &mut Memory| {
            mem.write_byte(0xFFFC, 0xB5);
            mem.write_byte(0xFFFD, 0x12);
        }),
        0x12,
        (|cpu: &CPU| cpu.get_cx_high())
    );

    // test for ax
    generate_test!(
        mov_ax_0x1234,
        (|mem: &mut Memory| {
            mem.write_byte(0xFFFC, 0xB8);
            mem.write_byte(0xFFFD, 0x12);
            mem.write_byte(0xFFFE, 0x34);
        }),
        0x3412,
        (|cpu: &CPU| cpu.ax)
    );

    // test for bx
    generate_test!(
        mov_bx_0x1234,
        (|mem: &mut Memory| {
            mem.write_byte(0xFFFC, 0xBB);
            mem.write_byte(0xFFFD, 0x12);
            mem.write_byte(0xFFFE, 0x34);
        }),
        0x3412,
        (|cpu: &CPU| cpu.bx)
    );

    // test for cx
    generate_test!(
        mov_cx_0x1234,
        (|mem: &mut Memory| {
            mem.write_byte(0xFFFC, 0xB9);
            mem.write_byte(0xFFFD, 0x12);
            mem.write_byte(0xFFFE, 0x34);
        }),
        0x3412,
        (|cpu: &CPU| cpu.cx)
    );

    // test for dx
    generate_test!(
        mov_dx_0x1234,
        (|mem: &mut Memory| {
            mem.write_byte(0xFFFC, 0xBA);
            mem.write_byte(0xFFFD, 0x12);
            mem.write_byte(0xFFFE, 0x34);
        }),
        0x3412,
        (|cpu: &CPU| cpu.dx)
    );

    // test for sp
    generate_test!(
        mov_sp_0x1234,
        (|mem: &mut Memory| {
            mem.write_byte(0xFFFC, 0xBC);
            mem.write_byte(0xFFFD, 0x12);
            mem.write_byte(0xFFFE, 0x34);
        }),
        0x3412,
        (|cpu: &CPU| cpu.stack_pointer)
    );

    // test for bp
    generate_test!(
        mov_bp_0x1234,
        (|mem: &mut Memory| {
            mem.write_byte(0xFFFC, 0xBD);
            mem.write_byte(0xFFFD, 0x12);
            mem.write_byte(0xFFFE, 0x34);
        }),
        0x3412,
        (|cpu: &CPU| cpu.base_pointer)
    );

    // test for si
    generate_test!(
        mov_si_0x1234,
        (|mem: &mut Memory| {
            mem.write_byte(0xFFFC, 0xBE);
            mem.write_byte(0xFFFD, 0x12);
            mem.write_byte(0xFFFE, 0x34);
        }),
        0x3412,
        (|cpu: &CPU| cpu.source_index)
    );

    // test for di
    generate_test!(
        mov_di_0x1234,
        (|mem: &mut Memory| {
            mem.write_byte(0xFFFC, 0xBF);
            mem.write_byte(0xFFFD, 0x12);
            mem.write_byte(0xFFFE, 0x34);
        }),
        0x3412,
        (|cpu: &CPU| cpu.destination_index)
    );
}