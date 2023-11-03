use crate::{cpu::CPU, memory::Memory};

pub mod direct_addressing;
pub mod immediate_addressing;
pub mod indexed_addressing;
pub mod register_addressing;

impl CPU {
    pub(in crate::cpu) fn execute_sub_register_word(&mut self, mem: &mut Memory) {
        let instruction = self.consume_instruction(mem);
        match instruction {
            0x00..=0x3F => {
                self.sub_16bit_register_indexed_registers_without_offset(mem, instruction)
            }
            0x40..=0x7F => {
                self.sub_16bit_register_indexed_registers_with_8bit_offset(mem, instruction)
            }
            0x80..=0xBF => {
                self.sub_16bit_register_indexed_registers_with_16bit_offset(mem, instruction)
            }
            0xC1..=0xFF => {
                self.sub_16bit_register_addressing(instruction);
            }
            x => panic!("SUB instruction not implemented! for {}", x),
        }
    }

    pub(in crate::cpu) fn execute_sub_register_byte(&mut self, mem: &mut Memory) {
        let instruction = self.consume_instruction(mem);
        match instruction {
            0x00..=0x3F => {
                self.sub_8bit_register_indexed_registers_without_offset(mem, instruction)
            }
            0x40..=0x7F => {
                self.sub_8bit_register_indexed_registers_with_8bit_offset(mem, instruction)
            }
            0x80..=0xBF => {
                self.sub_8bit_register_indexed_registers_with_16bit_offset(mem, instruction)
            }
            0xC0..=0xFF => {
                self.sub_8bit_register_addressing(instruction);
            }
        }
    }

    pub(in crate::cpu) fn execute_sub_indexed_addr_16bit_register(&mut self, mem: &mut Memory) {
        let exec_fn = |cpu: &mut CPU, val1: u16, val2: u16| -> Option<u16> {
            let (result, _) = cpu.sub_16bit_with_overflow_and_set_flags(val1, val2);
            Some(result)
        };
        self.consume_bytes_and_parse_mem_as_first_arg_double_ins(mem, &exec_fn);
    }
}

#[cfg(test)]
mod sub_mem_reg_tests {
    use crate::cpu::instructions::test_macro::execute_code;

    #[test]
    fn test_sub_mem_reg() {
        let code = "
        mov ax, 0x1234
        mov bx, 0x100
        mov cx, 0xF0F

        mov [bx + 0x02], cx
        sub [bx + 0x02], ax
        ";
        let (cpu, mem) = execute_code(code);
        assert_eq!(cpu.read_word_from_pointer(&mem, 0x102), 0xFCDB);
    }

    #[test]
    fn test_sub_mem_reg_8bit() {
        let code = "
        mov ax, 0x1234
        mov si, 0x02
        mov cx, 0xF0F

        mov [si + 0x100], cx
        sub [si + 0x100], ax
        ";
        let (cpu, mem) = execute_code(code);
        assert_eq!(cpu.read_word_from_pointer(&mem, 0x102), 0xFCDB);
    }
}
