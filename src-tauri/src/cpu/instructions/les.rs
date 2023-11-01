use crate::{consts::U20, cpu::CPU, memory::Memory};

impl CPU {
    pub(in crate::cpu) fn exec_les_16_bit_reg_mem(&mut self, mem: &mut Memory) {
        let ins = self.consume_instruction(mem);
        match ins {
            0x06 | 0x0E | 0x16 | 0x1E | 0x26 | 0x2E | 0x36 | 0x3E => {
                let reg_idx = ins >> 3;
                let addr = self.consume_word(mem);
                let word1 = self.read_word_from_pointer(mem, addr);
                let word2 = self.read_word_from_pointer(mem, addr + 2);

                self.set_16bit_register_by_index(reg_idx, word1);
                self.extra_segment = word2;
            }
            0x00..=0x3F => {
                let (indexed_addr_idx, reg_idx) = self.get_index_from_0x00_0x3f_pattern(ins);
                let mem_addr = self.get_offset_from_index_of_indexed_registers(indexed_addr_idx);
                let word1 = self.read_word_from_u20(mem, mem_addr.clone());
                let word2 = self.read_word_from_u20(mem, mem_addr + U20::from(2_u16));
                self.set_16bit_register_by_index(reg_idx, word1);
                self.extra_segment = word2;
            }
            0x40..=0x7F => {
                let (indexed_addr_idx, reg_idx) = self.get_index_from_0x40_0x7f_pattern(ins);
                let mem_addr = self.consume_byte_and_get_cummulative_offset(mem, indexed_addr_idx);
                let word1 = self.read_word_from_u20(mem, mem_addr.clone());
                let word2 = self.read_word_from_u20(mem, mem_addr + U20::from(2_u16));
                self.set_16bit_register_by_index(reg_idx, word1);
                self.extra_segment = word2;
            }
            0x80..=0xBF => {
                let (indexed_addr_idx, reg_idx) = self.get_index_from_0x80_0xbf_pattern(ins);
                let mem_addr = self.consume_word_and_get_cummulative_offset(mem, indexed_addr_idx);
                let word1 = self.read_word_from_u20(mem, mem_addr.clone());
                let word2 = self.read_word_from_u20(mem, mem_addr + U20::from(2_u16));
                self.set_16bit_register_by_index(reg_idx, word1);
                self.extra_segment = word2;
            }
            0xC0..=0xFF => {}
        }
    }
}

#[cfg(test)]
mod les_exec_tests {
    use crate::cpu::instructions::test_macro::run_code;

    #[test]
    fn test_reg_direct_mem_test() {
        let code = "
        org 100h 
        .data 
        var dw 0x100
        code: 
        les ax, var
        ";
        let (cpu, _) = run_code(code, 3);
        assert_eq!(cpu.ax, 0x100);
        assert_eq!(cpu.extra_segment, 0x06C4);
    }

    #[test]
    fn test_reg_mem_tests() {
        let code = "
        org 100h 
        code: 
        mov bx, 0x100
        mov si, 0x02 
        les ax, [bx+si]
        ";
        let (cpu, _) = run_code(code, 5);
        assert_eq!(cpu.ax, 0xBE01);
        assert_eq!(cpu.extra_segment, 0x0002)
    }

    #[test]
    fn test_reg_mem_tests_2() {
        let code = "
        org 100h 
        code: 
        mov bx, 0x100
        mov si, 0x02 
        les sp, [bx+si+0x01]
        ";
        let (cpu, _) = run_code(code, 3);
        assert_eq!(cpu.stack_pointer, 0x02BE);
        assert_eq!(cpu.extra_segment, 0xC400);
    }

    #[test]
    fn test_reg_mem_tests_3() {
        let code = "
        org 100h 
        code: 
        mov bx, 0x1
        mov si, 0x1
        les di, [bx+si+0x100]
        ";
        let (cpu, _) = run_code(code, 3);
        assert_eq!(cpu.destination_index, 0xBE00);
        assert_eq!(cpu.extra_segment, 0x0001)
    }
}
