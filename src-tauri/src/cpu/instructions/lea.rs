use crate::{cpu::CPU, memory::Memory, consts::U20};

impl CPU {
    pub(in crate::cpu) fn exec_lea_reg_mem(&mut self, mem: &mut Memory) {
        let ins = self.consume_instruction(mem);
        match ins {
            0x06 | 0x0E | 0x16 | 0x1E | 0x26 | 0x2E | 0x36 | 0x3E => {
                let reg_idx = ins >> 3;
                let addr = self.consume_word(mem);
                self.set_16bit_register_by_index(reg_idx, addr);
            }
            0x00..=0x3F => {
                let (indexed_addr_idx, reg_idx) = self.get_index_from_0x00_0x3f_pattern(ins);
                let mem_addr = self.get_absolute_offset_from_index_of_indexed_registers(indexed_addr_idx);
                let (_, mem_addr) = mem_addr.as_segment_offset();
                self.set_16bit_register_by_index(reg_idx, mem_addr);
            }
            0x40..=0x7F => {
                let (indexed_addr_idx, reg_idx) = self.get_index_from_0x40_0x7f_pattern(ins);
                let mem_addr = self.get_absolute_offset_from_index_of_indexed_registers(indexed_addr_idx);
                let offset_8bit = self.consume_byte(mem);
                let (_, mem_addr) = (mem_addr + U20::from(offset_8bit)).as_segment_offset();
                self.set_16bit_register_by_index(reg_idx, mem_addr);
            }
            0x80..=0xBF => {
                let (indexed_addr_idx, reg_idx) = self.get_index_from_0x80_0xbf_pattern(ins);
                let mem_addr = self.get_absolute_offset_from_index_of_indexed_registers(indexed_addr_idx);
                let offset_16bit = self.consume_word(mem);
                let (_, mem_addr) = (mem_addr + U20::from(offset_16bit)).as_segment_offset();
                self.set_16bit_register_by_index(reg_idx, mem_addr);
            }
            0xC0..=0xFF => {
                let (reg_idx_1, reg_idx_2) = self.get_index_from_c0_ff_pattern(ins);
                let reg_2 = self.get_16bit_register_by_index(reg_idx_2);
                self.set_16bit_register_by_index(reg_idx_1, reg_2);
            }
        }
    }
}

#[cfg(test)]
mod lea_exec_tests{
    use crate::cpu::instructions::test_macro::run_code;

    #[test]
    fn test_reg_mem_tests(){
        let code = "
        mov bx, 0x100
        mov si, 0x02 
        lea ax, [bx+si]
        ";
        let (cpu, _) = run_code(code, 3);
        assert_eq!(cpu.ax, 0x102);
    }

    #[test]
    fn test_reg_mem_tests_2(){
        let code = "
        mov bx, 0x100
        mov si, 0x02 
        lea sp, [bx+si+0x01]
        ";
        let (cpu, _) = run_code(code, 3);
        assert_eq!(cpu.stack_pointer, 0x103);
    }

    #[test]
    fn test_reg_mem_tests_3(){
        let code = "
        mov bx, 0xffff
        mov si, 0xffff
        lea di, [bx+si+0x01]
        ";
        let (cpu, _) = run_code(code, 3);
        assert_eq!(cpu.destination_index, 0xFFFF);
    }
}
