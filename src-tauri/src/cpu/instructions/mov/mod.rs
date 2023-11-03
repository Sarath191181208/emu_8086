use crate::{cpu::CPU, memory::Memory};

pub mod direct_addressing;
pub mod immediate_addressing;
pub(super) mod indexed_addressing;
pub mod register_addressing;

impl CPU {
    pub(in crate::cpu) fn execute_mov_register_word(&mut self, mem: &mut Memory) {
        let instruction = self.consume_instruction(mem);
        match instruction {
            0x00..=0x3F => {
                self.mov_16bit_register_indexed_registers_without_offset(mem, instruction)
            }
            0x40..=0x7f => {
                self.mov_16bit_register_indexed_registers_with_8bit_offset(mem, instruction)
            }
            0x80..=0xBF => {
                self.mov_16bit_register_indexed_registers_with_16bit_offset(mem, instruction)
            }
            0xC0..=0xFF => {
                self.mov_16bit_register_addressing(instruction);
            }
        }
    }

    pub(in crate::cpu) fn execute_mov_register_byte(&mut self, mem: &mut Memory) {
        let instruction = self.consume_instruction(mem);
        match instruction {
            0x00..=0x3F => {
                self.mov_8bit_register_indexed_registers_without_offset(mem, instruction)
            }
            0x40..=0x7f => {
                self.mov_8bit_register_indexed_registers_with_8bit_offset(mem, instruction)
            }
            0x80..=0xBF => {
                self.mov_8bit_register_indexed_registers_with_16bit_offset(mem, instruction)
            }
            0xC0..=0xFF => {
                self.mov_8bit_register_addressing(instruction);
            }
        }
    }

    pub(in crate::cpu) fn execute_mov_indexed_addr_16bit_register(&mut self, mem: &mut Memory) {
        let exec_fn = |_: &mut CPU, _: u16, val: u16| -> Option<u16> { Some(val) };
        self.consume_bytes_and_parse_mem_as_first_arg_double_ins(mem, &exec_fn);
    }
}

#[cfg(test)]
mod mov_indexed_addr_tests {
    use crate::cpu::instructions::test_macro::execute_code;

    #[test]
    fn test_mov_idx_addr_reg() {
        let code = "
        mov ax, 0x100
        mov bx, 0x200
        mov cx, 0x300

        mov [0x1234], ax
        mov [0x1236], bx  

        mov bx, 0x1230 
        mov [bx + 0x08], cx
        ";
        let (cpu, mem) = execute_code(code);
        assert_eq!(cpu.read_word_from_pointer(&mem, 0x1234), 0x100);
        assert_eq!(cpu.read_word_from_pointer(&mem, 0x1236), 0x200);
        assert_eq!(cpu.read_word_from_pointer(&mem, 0x1238), 0x300);
    }
}
