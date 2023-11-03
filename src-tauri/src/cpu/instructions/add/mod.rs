use crate::{cpu::CPU, memory::Memory};

pub mod direct_addressing;
pub mod immediate_addressing;
pub mod indexed_addressing;
pub mod register_addressing;

impl CPU {
    // add bx, _
    pub(in crate::cpu) fn execute_add(&mut self, mem: &mut Memory) {
        let instruction = self.consume_instruction(mem);
        match instruction {
            0x00..=0x3F => {
                self.add_16bit_register_indexed_registers_without_offset(mem, instruction)
            }
            0x40..=0x7F => {
                self.add_16bit_register_indexed_registers_with_8bit_offset(mem, instruction)
            }
            0x80..=0xBF => {
                self.add_16bit_register_indexed_registers_with_16bit_offset(mem, instruction)
            }
            0xC0..=0xFF => {
                self.add_16bit_register_addressing(instruction);
            }
        }
    }

    // add bl, _
    pub(in crate::cpu) fn execute_add_register(&mut self, mem: &mut Memory) {
        let instruction = self.consume_instruction(mem);
        match instruction {
            0x00..=0x3F => {
                self.add_8bit_register_indexed_registers_without_offset(mem, instruction)
            }
            0x40..=0x7F => {
                self.add_8bit_register_indexed_registers_with_8bit_offset(mem, instruction)
            }
            0x80..=0xBF => {
                self.add_8bit_register_indexed_registers_with_16bit_offset(mem, instruction)
            }
            0xC0..=0xFF => {
                self.add_8bit_register_addressing(instruction);
            }
        }
    }

    pub(in crate::cpu) fn execute_add_indexed_addr_16bit_register(&mut self, mem: &mut Memory) {
        let exec_fn = |cpu: &mut CPU, val1: u16, val2: u16| -> Option<u16> {
            let (result, _) = cpu.add_16bit_with_overflow_and_set_flags(val1, val2);
            Some(result)
        };
        self.consume_bytes_and_parse_mem_as_first_arg_double_ins(mem, &exec_fn);
    }
}

#[cfg(test)]
mod add_test_mem_reg {
    use crate::cpu::instructions::test_macro::execute_code;

    #[test]
    fn test_add_mem_reg() {
        let code = "
            mov ax, 0x1234
            mov cx, 0x1236
            mov dx, 0x1238

            mov w.[0x1234], 0x00 
            mov w.[0x1236], 0x00
            mov w.[0x1238], 0x00

            add [0x1234], ax 
            
            mov bx, 0x1234
            add [bx+0x02] , cx

            mov bx, 0x04
            add [0x1234+bx] , dx
        ";
        let (cpu, mem) = execute_code(code);
        assert_eq!(cpu.read_word_from_pointer(&mem, 0x1234), 0x1234);
        assert_eq!(cpu.read_word_from_pointer(&mem, 0x1236), 0x1236);
        assert_eq!(cpu.read_word_from_pointer(&mem, 0x1238), 0x1238);
    }
}
