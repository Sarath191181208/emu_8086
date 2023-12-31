use crate::{consts::Byte, cpu::CPU};

// Register Addressing
impl CPU {
    pub(super) fn add_8bit_register_addressing(&mut self, instruction: Byte) {
        let (source_index, write_index) = self.get_index_from_c0_ff_pattern(instruction);
        let reg = self.get_8bit_register_by_index(source_index % 8);
        let write_reg = self.get_8bit_register_by_index(write_index);
        let (result, _) = self.add_8bit_with_overflow_and_set_flags(write_reg, reg);
        self.set_8bit_register_by_index(write_index, result);
    }

    pub(in crate::cpu::instructions::add) fn add_16bit_register_addressing(
        &mut self,
        instruction: Byte,
    ) {
        let (source_index, write_index) = self.get_index_from_c0_ff_pattern(instruction);
        let reg = self.get_16bit_register_by_index(source_index % 8);
        let write_reg = self.get_16bit_register_by_index(write_index);
        let (result, _) = self.add_16bit_with_overflow_and_set_flags(reg, write_reg);
        self.set_16bit_register_by_index(write_index, result);
    }
}

#[cfg(test)]
mod add_16bit_register_addressing_tests {
    use crate::{cpu::CPU, generate_test, memory::Memory};

    // test ax+ax
    generate_test!(
        add_ax_ax_no_overflow,
        (|cpu: &mut CPU, mem: &mut Memory| {
            cpu.write_instructions(mem, &[0x02, 0xC0]);
            cpu.ax = 0x0001;
        }),
        (|cpu: &CPU, _mem: &Memory| {
            assert_eq!(0x0002, cpu.ax);
            assert!(!cpu.overflow_flag);
            assert!(!cpu.zero_flag);
            assert!(!cpu.negative_flag);
        })
    );

    generate_test!(
        add_ax_ax_zero,
        (|cpu: &mut CPU, mem: &mut Memory| {
            cpu.write_instructions(mem, &[0x02, 0xC0]);
        }),
        (|cpu: &CPU, _mem: &Memory| {
            assert_eq!(0x0000, cpu.ax);
            assert!(!cpu.overflow_flag);
            assert!(cpu.zero_flag);
            assert!(!cpu.negative_flag);
        })
    );

    generate_test!(
        add_ax_ax_overflow,
        (|cpu: &mut CPU, mem: &mut Memory| {
            cpu.write_instructions(mem, &[0x02, 0xC0]);
            cpu.ax = 0xFFFF;
        }),
        (|cpu: &CPU, _mem: &Memory| {
            assert_eq!(0xFFFE, cpu.ax);
            assert_eq!(cpu.get_flags_as_binary(), 0b00100101);
        })
    );

    generate_test!(
        add_ax_bx,
        (|cpu: &mut CPU, mem: &mut Memory| {
            cpu.bx = 0x1100;
            cpu.ax = 0x0011;
            cpu.write_instructions(mem, &[0x03, 0xC3]);
        }),
        (|cpu: &CPU, _mem: &Memory| {
            assert_eq!(0x1111, cpu.ax);
            assert!(!cpu.overflow_flag);
            assert!(!cpu.zero_flag);
            assert!(!cpu.negative_flag);
        })
    );
}

#[cfg(test)]
mod add_8bit_register_addressing_tests {
    use crate::{cpu::CPU, generate_test, memory::Memory};

    // ADD AL, BL
    generate_test!(
        add_al_bl,
        (|cpu: &mut CPU, mem: &mut Memory| {
            cpu.set_ax_low(0x01);
            cpu.set_bx_low(0x01);
            cpu.write_instructions(mem, &[0x02, 0xC3]);
        }),
        (|cpu: &CPU, _mem: &Memory| {
            assert_eq!(0x02, cpu.get_ax_low());
            assert!(!cpu.overflow_flag);
            assert!(!cpu.zero_flag);
            assert!(!cpu.negative_flag);
        })
    );

    // ADD BL, CL zero
    generate_test!(
        add_bl_cl,
        (|cpu: &mut CPU, mem: &mut Memory| {
            cpu.set_bx_low(0x01);
            cpu.set_cx_low(0x01);
            cpu.write_instructions(mem, &[0x02, 0xD9]);
        }),
        (|cpu: &CPU, _mem: &Memory| {
            assert_eq!(0x02, cpu.get_bx_low());
            assert!(!cpu.overflow_flag);
            assert!(!cpu.zero_flag);
            assert!(!cpu.negative_flag);
        })
    );

    // ADD CH, BL
    generate_test!(
        add_ch_bl,
        (|cpu: &mut CPU, mem: &mut Memory| {
            cpu.set_bx_low(0x01);
            cpu.set_cx_high(0x01);
            cpu.write_instructions(mem, &[0x02, 0xEB]);
        }),
        (|cpu: &CPU, _mem: &Memory| {
            assert_eq!(0x02, cpu.get_cx_high());
            assert!(!cpu.overflow_flag);
            assert!(!cpu.zero_flag);
            assert!(!cpu.negative_flag);
        })
    );
}
