use crate::{
    consts::{Byte, Word},
    cpu::CPU,
    memory::Memory,
};

// Immediate Addressing
impl CPU {
    pub(in crate::cpu) fn add_ax_in_immediate_addressing(&mut self, mem: &Memory) {
        let data_low = self.consume_instruction(mem);
        let data_high = self.consume_instruction(mem);
        let data = ((data_high as Word) << 8) | (data_low as Word);
        let (result, _) = self.add_16bit_with_overflow_and_set_flags(self.ax, data);
        self.ax = result;
    }

    pub(in crate::cpu) fn add_al_in_immediate_addressing(&mut self, mem: &Memory) {
        let data = self.consume_instruction(mem);
        let (result, _) = self.add_8bit_with_overflow_and_set_flags(self.ax as u8, data);
        self.ax = (self.ax & 0xFF00) | (result as Word);
    }

    pub(in crate::cpu) fn execute_add_immediate_byte(&mut self, mem: &Memory) {
        let instruction = self.consume_instruction(mem);
        match instruction {
            0xC1..=0xC7 => {
                let index = instruction & 0x07;
                let data = self.consume_instruction(mem);
                let (result, _) = self.add_8bit_with_overflow_and_set_flags(
                    self.get_8bit_register_by_index(index),
                    data,
                );
                self.set_8bit_register_by_index(index, result);
            }
            x => panic!("ADD instruction not implemented! for {}", x),
        }
    }

    fn get_data_add(&mut self, mem: &Memory, instruction: Byte) -> Word {
        match instruction {
            0x81 => {
                self.consume_word(mem)
            }
            0x83 => {
                self.consume_byte(mem) as Word
            }
            x => panic!("ADD instruction not implemented! for {}", x),
        }
    }

    pub(in crate::cpu) fn execute_add_reg_immediate_word(&mut self, mem: &Memory, instruction: Byte) {
        let index = self.consume_instruction(mem) & 0x07;
        let data = self.get_data_add(mem, instruction);
        let (result, _) = self
            .add_16bit_with_overflow_and_set_flags(self.get_16bit_register_by_index(index), data);
        self.set_16bit_register_by_index(index, result);
    }
}

#[cfg(test)]
mod add_immediate_16bit_tests {
    use crate::{cpu::CPU, generate_test, memory::Memory};

    // test ax+ax
    generate_test!(
        add_ax_ax_no_overflow,
        (|cpu: &mut CPU, mem: &mut Memory| {
            cpu.write_instructions(mem, &[0x05, 0x34, 0x12]);
            cpu.ax = 0x1234;
        }),
        (|cpu: &CPU, _: &Memory| {
            assert_eq!(cpu.ax, 0x2468);
            assert_eq!(cpu.get_flags_as_binary(), 0b00000000);
        })
    );

    // test ax+ax overflow
    generate_test!(
        add_ax_ax_overflow,
        (|cpu: &mut CPU, mem: &mut Memory| {
            cpu.write_instructions(mem, &[0x05, 0xFF, 0xFF]);
            cpu.ax = 0xFFFF;
        }),
        (|cpu: &CPU, _: &Memory| {
            assert_eq!(cpu.ax, 0xFFFE);
            assert!(cpu.carry_flag);
            assert_eq!(cpu.get_flags_as_binary(), 0b00100101)
        })
    );

    // test ax+ax zero
    generate_test!(
        add_ax_ax_zero,
        (|cpu: &mut CPU, mem: &mut Memory| {
            cpu.write_instructions(mem, &[0x05, 0x00, 0x00]);
            cpu.ax = 0x0000;
        }),
        (|cpu: &CPU, _: &Memory| {
            assert_eq!(cpu.ax, 0x0000);
            assert!(cpu.zero_flag);
            assert_eq!(cpu.get_flags_as_binary(), 0b00010010);
        })
    );

    // test ax+ax negative
    generate_test!(
        add_ax_ax_negative,
        (|cpu: &mut CPU, mem: &mut Memory| {
            cpu.write_instructions(mem, &[0x05, 0xFF, 0xFF]);
            cpu.ax = 0x0001;
        }),
        (|cpu: &CPU, _: &Memory| {
            assert_eq!(cpu.ax, 0x0000);
            assert_eq!(cpu.get_flags_as_binary(), 0b00110011);
        })
    );

    // test bx + 0x1234
    generate_test!(
        add_bx_0x1234,
        (|cpu: &mut CPU, mem: &mut Memory| {
            cpu.instruction_pointer = 0xFFFB;
            cpu.write_instructions(mem, &[0x81, 0xC3, 0x34, 0x12]);
            cpu.bx = 0x0001;
        }),
        (|cpu: &CPU, _: &Memory| {
            assert_eq!(cpu.bx, 0x1235);
            assert_eq!(cpu.get_flags_as_binary(), 0b00010000);
        })
    );

    // test bx + 0x1234 overflow
    generate_test!(
        add_bx_0x1234_overflow,
        (|cpu: &mut CPU, mem: &mut Memory| {
            cpu.instruction_pointer = 0xFFFB;
            cpu.write_instructions(mem, &[0x81, 0xC3, 0xFF, 0xFF]);
            cpu.bx = 0xFFFF;
        }),
        (|cpu: &CPU, _: &Memory| {
            assert_eq!(cpu.bx, 0xFFFE);
            assert_eq!(cpu.get_flags_as_binary(), 0b00100101);
        })
    );

    // test bx + 0xFFEE
    generate_test!(
        add_bx_0xffee,
        (|cpu: &mut CPU, mem: &mut Memory| {
            cpu.instruction_pointer = 0xFFFB;
            cpu.write_instructions(mem, &[0x83, 0xC3, 0xEE]);
            cpu.bx = 0xFF01;
        }),
        (|cpu: &CPU, _: &Memory| {
            assert_eq!(cpu.bx, 0xFFEF);
            assert!(!cpu.overflow_flag);
            assert!(!cpu.carry_flag);
        })
    );
}

#[cfg(test)]
mod add_immediate_8bit_tests {
    use crate::{cpu::CPU, generate_test, memory::Memory};

    // test al+0x12
    generate_test!(
        add_al_0x12,
        (|cpu: &mut CPU, mem: &mut Memory| {
            cpu.instruction_pointer = 0xFFFB;
            cpu.write_instructions(mem, &[0x04, 0x12]);
            cpu.ax = 0x0001;
        }),
        (|cpu: &CPU, _: &Memory| {
            assert_eq!(cpu.ax, 0x0013);
            assert_eq!(cpu.get_flags_as_binary(), 0b00000000)
        })
    );

    // test al+0xFF overflow
    generate_test!(
        add_al_0x12_overflow,
        (|cpu: &mut CPU, mem: &mut Memory| {
            cpu.instruction_pointer = 0xFFFB;
            cpu.write_instructions(mem, &[0x04, 0xFF]);
            cpu.ax = 0x00FE;
        }),
        (|cpu: &CPU, _: &Memory| {
            assert_eq!(cpu.ax, 0x00FD);
            assert_eq!(cpu.get_flags_as_binary(), 0b00100101);
        })
    );

    // add cl + 0x12
    generate_test!(
        add_cl_0x12,
        (|cpu: &mut CPU, mem: &mut Memory| {
            cpu.instruction_pointer = 0xFFFB;
            cpu.write_instructions(mem, &[0x80, 0xC1, 0x12]);
            cpu.cx = 0x0001;
        }),
        (|cpu: &CPU, _: &Memory| {
            assert_eq!(cpu.cx, 0x0013);
            assert_eq!(cpu.get_flags_as_binary(), 0b00000000);
        })
    );

    // add cl + 0xFF overflow
    generate_test!(
        add_cl_0xff_overflow,
        (|cpu: &mut CPU, mem: &mut Memory| {
            cpu.instruction_pointer = 0xFFFB;
            cpu.write_instructions(mem, &[0x80, 0xC1, 0xFF]);
            cpu.cx = 0x00FE;
        }),
        (|cpu: &CPU, _: &Memory| {
            assert_eq!(cpu.cx, 0x00FD);
            assert_eq!(cpu.get_flags_as_binary(), 0b00100101);
        })
    );

    // add bh + 0x12
    generate_test!(
        add_bh_0x12,
        (|cpu: &mut CPU, mem: &mut Memory| {
            cpu.instruction_pointer = 0xFFFB;
            cpu.write_instructions(mem, &[0x80, 0xC7, 0x12]);
            cpu.bx = 0xFF01;
        }),
        (|cpu: &CPU, _: &Memory| {
            assert_eq!(cpu.bx, 0x1101);
            assert_eq!(cpu.get_flags_as_binary(), 0b00110001);
        })
    );

    // add bh + 0xFF overflow
    generate_test!(
        add_bh_0xff_overflow,
        (|cpu: &mut CPU, mem: &mut Memory| {
            cpu.instruction_pointer = 0xFFFB;
            cpu.write_instructions(mem, &[0x80, 0xC7, 0xFF]);
            cpu.bx = 0x00FE;
        }),
        (|cpu: &CPU, _: &Memory| {
            assert_eq!(cpu.bx, 0xFFFE);
            assert_eq!(cpu.get_flags_as_binary(), 0b00010100);
        })
    );
}
