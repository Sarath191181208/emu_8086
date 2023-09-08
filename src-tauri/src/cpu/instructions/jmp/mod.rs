use crate::{cpu::CPU, memory::Memory};

impl CPU {
    pub(in crate::cpu) fn execute_jmp_8bit(&mut self, mem: &mut Memory) {
        let offset = self.consume_instruction(mem);
        match offset {
            0x80..=0xFF => {
                let offset = 0xFF - offset + 1;
                self.instruction_pointer = self.instruction_pointer.wrapping_sub(offset as u16);
            }
            0x00..=0x7F => {
                self.instruction_pointer = self.instruction_pointer.wrapping_add(offset as u16);
            }
        }
    }

    pub(in crate::cpu) fn execute_jmp_16bit(&mut self, mem: &mut Memory) {
        let offset_low = self.consume_instruction(mem);
        let offset_high = self.consume_instruction(mem);

        let offset = (offset_high as u16) << 8 | offset_low as u16;
        match offset {
            0x8000..=0xFFFF => {
                print!("offset: {:x}", offset);
                let offset = 0xFFFF - offset;
                self.instruction_pointer = self.instruction_pointer.wrapping_sub(offset);
            }
            0x0000..=0x7FFF => {
                self.instruction_pointer = self.instruction_pointer.wrapping_add(offset);
            }
        }
    }
}

#[cfg(test)]
mod test_8bit_jmp {
    use crate::{compiler::compile_lines, cpu::CPU, generate_test_jmp, memory::Memory};

    generate_test_jmp!(
        jmp_8bit_positive,
        (|cpu: &mut CPU, mem: &mut Memory| {
            let (compiled_bytes, _) = compile_lines(
                "
            label:
                INC AX
            jmp label
            ",
                false,
            )
            .unwrap();
            cpu.write_instructions(mem, &compiled_bytes);
        }),
        (|cpu: &CPU, _: &Memory| {
            assert_eq!(cpu.instruction_pointer, 0x0101);
            assert_eq!(cpu.ax, 0x0003);
        }),
        5
    );

    generate_test_jmp!(
        jmp_8bit_negative,
        (|cpu: &mut CPU, mem: &mut Memory| {
            let (compiled_bytes, _) = compile_lines(
                "
                INC AX
                JMP label
                DEC AX
                label:
                INC AX  
            ",
                false,
            )
            .unwrap();
            cpu.write_instructions(mem, &compiled_bytes);
        }),
        (|cpu: &CPU, _: &Memory| {
            assert_eq!(cpu.instruction_pointer, 0x0105);
            assert_eq!(cpu.ax, 0x0002);
        }),
        3
    );
}

#[cfg(test)]
mod test_16_bit_jmp {
    use crate::{compiler::compile_lines, cpu::CPU, generate_test_jmp, memory::Memory};

    fn generate_0x80_long_ins() -> String {
        let mut ins = String::new();
        for _ in 0..0x80 {
            ins.push_str("INC AX\n");
        }
        ins
    }

    generate_test_jmp!(
        jmp_16bit_positive,
        (|cpu: &mut CPU, mem: &mut Memory| {
            let (compiled_bytes, _) = compile_lines(
                format!(
                    "
            label:
                {}
            jmp label
            ",
                    generate_0x80_long_ins()
                )
                .as_str(),
                false,
            )
            .unwrap();
            cpu.write_instructions(mem, &compiled_bytes);
        }),
        (|cpu: &CPU, _: &Memory| {
            assert_eq!(cpu.instruction_pointer, 0x0102);
            assert_eq!(cpu.ax, 0x0081);
        }),
        0x82
    );

    generate_test_jmp!(
        jmp_16bit_negative,
        (|cpu: &mut CPU, mem: &mut Memory| {
            let (compiled_bytes, _) = compile_lines(
                format!(
                    "
                INC AX
                JMP label
                {}
                label:
                INC AX
            ",
                    generate_0x80_long_ins()
                )
                .as_str(),
                false,
            )
            .unwrap();
            cpu.write_instructions(mem, &compiled_bytes);
        }),
        (|cpu: &CPU, _: &Memory| {
            assert_eq!(cpu.instruction_pointer, 0x0185);
            assert_eq!(cpu.ax, 0x0002);
        }),
        0x3
    );
}
