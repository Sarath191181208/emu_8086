use crate::{consts::Byte, cpu::CPU, memory::Memory};

impl CPU {
    fn add_8bit_register_addressing(&mut self, instruction: Byte) {
        let (source_index, write_index) = self.get_index_from_c0_ff_pattern(instruction);
        let reg = self.get_8bit_register_by_index(source_index % 8);
        let write_reg = self.get_8bit_register_by_index(write_index);
        let (result, overflow) = reg.overflowing_add(write_reg);
        self.set_8bit_register_by_index(write_index, result);
        self.overflow_flag = overflow;
        self.zero_flag = result == 0;
        self.negative_flag = result & 0x80 != 0;
    }

    pub(in crate::cpu) fn execute_add_register_byte(&mut self, mem: &Memory) {
        let instruction = self.consume_instruction(mem);
        match instruction {
            0xC0..=0xFF => {
                self.add_8bit_register_addressing(instruction);
            }
            x => panic!("ADD instruction not implemented! for {}", x),
        }
    }

    fn add_16bit_register_addressing(&mut self, instruction: Byte) {
        let (source_index, write_index) = self.get_index_from_c0_ff_pattern(instruction);
        let reg = self.get_16bit_register_by_index(source_index % 8);
        let write_reg = self.get_16bit_register_by_index(write_index);
        let (result, overflow) = reg.overflowing_add(write_reg);
        self.set_16bit_register_by_index(write_index, result);
        self.overflow_flag = overflow;
        self.zero_flag = result == 0;
        self.negative_flag = result & 0x8000 != 0;
    }

    pub(in crate::cpu) fn execute_add_register_word(&mut self, mem: &Memory) {
        let instruction = self.consume_instruction(mem);
        match instruction {
            0xC1..=0xFF => {
                self.add_16bit_register_addressing(instruction);
            }
            x => panic!("ADD instruction not implemented! for {}", x),
        }
    }
}

#[cfg(test)]
mod add_16bit_register_addressing_tests {
    use crate::cpu::CPU;
    use crate::memory::Memory;

    macro_rules! generate_test {
        ($test_name:ident, $instructions:expr, $compare: expr) => {
            paste::item! {
                #[test]
                fn [<test_ $test_name>]() {
                    let mut cpu = CPU::new();
                    let mut mem = Memory::new();
                    cpu.reset(&mut mem);

                    $instructions(&mut cpu, &mut mem);

                    cpu.execute(&mut mem);

                    $compare(&cpu, &mem);
                }
            }
        };
    }

    // test ax+ax
    generate_test!(
        add_ax_ax_no_overflow,
        (|cpu: &mut CPU, mem: &mut Memory| {
            mem.write_byte(0xFFFC, 0x02);
            mem.write_byte(0xFFFD, 0xC0);
            cpu.ax = 0x0001;
        }),
        (|cpu: &CPU, _mem: &Memory| {
            assert_eq!(0x0002, cpu.ax);
            assert_eq!(false, cpu.overflow_flag);
            assert_eq!(false, cpu.zero_flag);
            assert_eq!(false, cpu.negative_flag);
        })
    );

    generate_test!(
        add_ax_ax_zero,
        (|_: &mut CPU, mem: &mut Memory| {
            mem.write_byte(0xFFFC, 0x02);
            mem.write_byte(0xFFFD, 0xC0);
        }),
        (|cpu: &CPU, _mem: &Memory| {
            assert_eq!(0x0000, cpu.ax);
            assert_eq!(false, cpu.overflow_flag);
            assert_eq!(true, cpu.zero_flag);
            assert_eq!(false, cpu.negative_flag);
        })
    );

    generate_test!(
        add_ax_ax_overflow,
        (|cpu: &mut CPU, mem: &mut Memory| {
            mem.write_byte(0xFFFC, 0x02);
            mem.write_byte(0xFFFD, 0xC0);
            cpu.ax = 0xFFFF;
        }),
        (|cpu: &CPU, _mem: &Memory| {
            assert_eq!(0xFFFE, cpu.ax);
            assert_eq!(true, cpu.overflow_flag);
            assert_eq!(false, cpu.zero_flag);
            assert_eq!(true, cpu.negative_flag);
        })
    );

    generate_test!(
        add_ax_bx,
        (|cpu: &mut CPU, mem: &mut Memory| {
            cpu.bx = 0x1100;
            cpu.ax = 0x0011;
            mem.write_byte(0xFFFC, 0x03);
            mem.write_byte(0xFFFD, 0xC3);
        }),
        (|cpu: &CPU, _mem: &Memory| {
            assert_eq!(0x1111, cpu.ax);
            assert_eq!(false, cpu.overflow_flag);
            assert_eq!(false, cpu.zero_flag);
            assert_eq!(false, cpu.negative_flag);
        })
    );
}
