use crate::{cpu::CPU, memory::Memory};

impl CPU {
    pub(in crate::cpu) fn execute_test_16bit_reg(&mut self, mem: &mut Memory) {
        let ins = self.consume_instruction(mem);
        match ins {
            0x06 | 0x0E | 0x16 | 0x1E | 0x26 | 0x2E | 0x36 | 0x3E => {
                self.execute_test_16bit_reg_and_indexed_addr_variable(mem, ins / 0x06);
            }
            0x00..=0x3F => {
                self.execute_test_16bit_reg_and_indexed_addr_no_offset(mem, ins);
            }
            0x40..=0x7F => {
                self.execute_test_16bit_reg_and_indexed_addr_with_8bitoffset(mem, ins);
            }
            0x80..=0xBF => {
                self.execute_test_16bit_reg_and_indexed_addr_with_16bitoffset(mem, ins);
            }
            0xC0..=0xFF => {
                self.execute_test_16bit_reg_and_16bit_reg(ins);
            }
        }
    }

    pub(in crate::cpu) fn execute_test_8bit_reg(&mut self, mem: &mut Memory) {
        let ins = self.consume_instruction(mem);
        match ins {
            0x06 | 0x0E | 0x16 | 0x1E | 0x26 | 0x2E | 0x36 | 0x3E => {
                self.execute_test_8bit_reg_and_indexed_addr_variable(mem, ins / 0x06);
            }
            0x00..=0x3F => {
                self.execute_test_8bit_reg_and_indexed_addr_no_offset(mem, ins);
            }
            0x40..=0x7F => {
                self.execute_test_8bit_reg_and_indexed_addr_with_8bitoffset(mem, ins);
            }
            0x80..=0xBF => {
                self.execute_test_8bit_reg_and_indexed_addr_with_16bitoffset(mem, ins);
            }
            0xC0..=0xFF => {
                self.execute_test_8bit_reg_and_8bit_reg(ins);
            }
        }
    }

    pub(in crate::cpu) fn execute_test_ax_and_number(&mut self, mem: &mut Memory) {
        let val = self.consume_word(mem);
        let res = self.ax & val;
        self.set_test_ins_flags_from_16bit_res(res);
    }

    pub(in crate::cpu) fn execute_test_al_and_number(&mut self, mem: &mut Memory) {
        let val = self.consume_byte(mem);
        let res = self.get_ax_low() & val;
        self.set_test_ins_flags_from_8bit_res(res);
    }

    pub(in crate::cpu) fn execute_test_16bit_reg_and_number(&mut self, mem: &mut Memory) {
        let ins = self.consume_instruction(mem);
        let reg = ins - 0xC0;
        let num = self.consume_word(mem);
        let res = self.get_16bit_register_by_index(reg) & num;
        self.set_test_ins_flags_from_16bit_res(res);
    }

    pub(in crate::cpu) fn execute_test_8bit_reg_and_number(&mut self, mem: &mut Memory) {
        let ins = self.consume_instruction(mem);
        let reg = ins - 0xC0;
        let num = self.consume_byte(mem);
        let res = self.get_8bit_register_by_index(reg) & num;
        self.set_test_ins_flags_from_8bit_res(res);
    }

    pub(in crate::cpu) fn execute_test_word_indexed_addressing_and_number(&mut self, mem: &mut Memory) {
        self.consume_instruction(mem); // 0x06
        let addr = self.consume_word(mem);
        let addr_val = self.read_word_from_pointer(mem, addr);
        let num = self.consume_word(mem);
        let res = addr_val & num;
        self.set_test_ins_flags_from_16bit_res(res);
    }

    pub(in crate::cpu) fn execute_test_byte_indexed_addressing_and_number(&mut self, mem: &mut Memory) {
        self.consume_instruction(mem); // 0x06
        let addr = self.consume_word(mem);
        let addr_val = self.read_byte_from_pointer(mem, addr);
        let num = self.consume_byte(mem);
        let res = addr_val & num;
        self.set_test_ins_flags_from_8bit_res(res);
    }

    fn execute_test_16bit_reg_and_16bit_reg(&mut self, ins: u8) {
        let (low_reg, high_reg) = self.get_index_from_c0_ff_pattern(ins);

        let high_val = self.get_16bit_register_by_index(high_reg);
        let low_val = self.get_16bit_register_by_index(low_reg);

        let res = high_val & low_val;
        self.set_test_ins_flags_from_16bit_res(res);
    }

    fn execute_test_8bit_reg_and_8bit_reg(&mut self, ins: u8) {
        let (low_reg, high_reg) = self.get_index_from_c0_ff_pattern(ins);

        let high_val = self.get_8bit_register_by_index(high_reg);
        let low_val = self.get_8bit_register_by_index(low_reg);

        let res = high_val & low_val;
        self.set_test_ins_flags_from_8bit_res(res);
    }

    fn execute_test_16bit_reg_and_indexed_addr_variable(&mut self, mem: &mut Memory, reg_idx: u8) {
        let addr = self.consume_word(mem);
        let val = self.read_word_from_pointer(mem, addr);
        let reg_val = self.get_16bit_register_by_index(reg_idx);
        let res = val & reg_val;
        self.set_test_ins_flags_from_16bit_res(res);
    }

    fn execute_test_8bit_reg_and_indexed_addr_variable(&mut self, mem: &mut Memory, reg_idx: u8) {
        let addr = self.consume_word(mem);
        let val = self.read_byte_from_pointer(mem, addr);
        let reg_val = self.get_8bit_register_by_index(reg_idx);
        let res = val & reg_val;
        self.set_test_ins_flags_from_8bit_res(res);
    }

    fn execute_test_16bit_reg_and_indexed_addr_no_offset(&mut self, mem: &mut Memory, ins: u8) {
        let (low_reg, high_reg) = self.get_index_from_0x00_0x3f_pattern(ins);

        let mem_addr = self.get_offset_from_index_of_indexed_registers(low_reg);
        let mem_val = self.read_word_from_u20(mem, mem_addr);
        let reg_val = self.get_16bit_register_by_index(high_reg);
        let res = mem_val & reg_val;
        self.set_test_ins_flags_from_16bit_res(res);
    }

    fn execute_test_8bit_reg_and_indexed_addr_no_offset(&mut self, mem: &mut Memory, ins: u8) {
        let (low_reg, high_reg) = self.get_index_from_0x00_0x3f_pattern(ins);

        let mem_addr = self.get_offset_from_index_of_indexed_registers(low_reg);
        let mem_val = self.read_byte_from_u20(mem, mem_addr);
        let reg_val = self.get_8bit_register_by_index(high_reg);
        let res = mem_val & reg_val;
        self.set_test_ins_flags_from_8bit_res(res);
    }

    fn execute_test_16bit_reg_and_indexed_addr_with_8bitoffset(&mut self, mem: &mut Memory, ins: u8) {
        let (low_reg, high_reg) = self.get_index_from_0x40_0x7f_pattern(ins);

        let mem_addr = self.consume_byte_and_get_cummulative_offset(mem, low_reg);
        let mem_val = self.read_word_from_u20(mem, mem_addr);

        let reg_val = self.get_16bit_register_by_index(high_reg);
        let res = mem_val & reg_val;
        self.set_test_ins_flags_from_16bit_res(res);
    }

    fn execute_test_8bit_reg_and_indexed_addr_with_8bitoffset(&mut self, mem: &mut Memory, ins: u8) {
        let (low_reg, high_reg) = self.get_index_from_0x40_0x7f_pattern(ins);

        let mem_addr = self.consume_byte_and_get_cummulative_offset(mem, low_reg);
        let mem_val = self.read_byte_from_u20(mem, mem_addr);

        let reg_val = self.get_8bit_register_by_index(high_reg);
        let res = mem_val & reg_val;
        self.set_test_ins_flags_from_8bit_res(res);
    }

    fn execute_test_16bit_reg_and_indexed_addr_with_16bitoffset(&mut self, mem: &mut Memory, ins: u8) {
        let (low_reg, high_reg) = self.get_index_from_0x80_0xbf_pattern(ins);

        let mem_addr = self.consume_word_and_get_cummulative_offset(mem, low_reg);
        let mem_val = self.read_word_from_u20(mem, mem_addr);

        let reg_val = self.get_16bit_register_by_index(high_reg);
        let res = mem_val & reg_val;
        self.set_test_ins_flags_from_16bit_res(res);
    }

    fn execute_test_8bit_reg_and_indexed_addr_with_16bitoffset(&mut self, mem: &mut Memory, ins: u8) {
        let (low_reg, high_reg) = self.get_index_from_0x80_0xbf_pattern(ins);

        let mem_addr = self.consume_word_and_get_cummulative_offset(mem, low_reg);
        let mem_val = self.read_byte_from_u20(mem, mem_addr);

        let reg_val = self.get_8bit_register_by_index(high_reg);
        let res = mem_val & reg_val;
        self.set_test_ins_flags_from_8bit_res(res);
    }

    fn set_test_ins_flags_from_16bit_res(&mut self, res: u16) {
        self.carry_flag = false;
        self.overflow_flag = false;
        self.zero_flag = res == 0;
        self.negative_flag = res & 0x8000 != 0;
        self.pairity_flag = res.count_ones() % 2 == 0;
    }

    fn set_test_ins_flags_from_8bit_res(&mut self, res: u8) {
        self.carry_flag = false;
        self.overflow_flag = false;
        self.zero_flag = res == 0;
        self.negative_flag = res & 0x80 != 0;
        self.pairity_flag = res.count_ones() % 2 == 0;
    }
}

#[cfg(test)]
mod test_ins_execution_tests{
    use crate::cpu::instructions::test_macro::run_code;
    use pretty_assertions::assert_eq;


    #[test]
    fn test_ax_and_number(){
        let code = "
            mov ax,  0x0F0F 
            test ax, 0x0F0F
        ";
        let (cpu, _) = run_code(code, 2);
        assert_eq!(cpu.zero_flag, false);
        assert_eq!(cpu.negative_flag, false);
        assert_eq!(cpu.pairity_flag, true);
    }

    #[test]
    fn test_al_and_number(){
        let code = "
            mov al,  0x0F 
            test al, 0x0F
        ";
        let (cpu, _) = run_code(code, 2);
        assert_eq!(cpu.zero_flag, false);
        assert_eq!(cpu.negative_flag, false);
        assert_eq!(cpu.pairity_flag, true);
    }

    #[test]
    fn test_16bit_reg_and_16bit_reg(){
        let code = "
            mov ax,  0x000F 
            mov bx,  0x0F0F 
            test ax, bx
        ";
        let (cpu, _) = run_code(code, 3);
        assert_eq!(cpu.zero_flag, false);
        assert_eq!(cpu.negative_flag, false);
        assert_eq!(cpu.pairity_flag, true);
    }

    #[test]
    fn test_8bit_reg_and_8bit_reg(){
        let code = "
            mov al,  0x0F 
            mov bl,  0x0F 
            test al, bl
        ";
        let (cpu, _) = run_code(code, 3);
        assert_eq!(cpu.zero_flag, false);
        assert_eq!(cpu.negative_flag, false);
        assert_eq!(cpu.pairity_flag, true);
    }

    // execute_test_16bit_reg_and_indexed_addr_variable
    #[test]
    fn test_reg_and_var(){
        let code = "
        org 100h 
        .data 
        var dw 0x20 
        code: 
            mov ax, 0x0F0F 
            test ax, [var]
        ";
        let (cpu, _) = run_code(code, 4);
        assert_eq!(cpu.zero_flag, true);
        assert_eq!(cpu.pairity_flag, true);
        assert_eq!(cpu.negative_flag, false);
    }
    // execute_test_16bit_reg_and_indexed_addr_no_offset
    #[test]
    fn test_reg_and_no_offset(){
        let code = "
        org 100h 
        .data 
        var dw 0x91
        code: 
            mov bx, 0x102 ; The place where var is stored
            mov ax, 0x0F0F 
            test ax, [bx]
        ";
        let (cpu, _) = run_code(code, 5);
        assert_eq!(cpu.zero_flag, false);
        assert_eq!(cpu.pairity_flag, false);
        assert_eq!(cpu.negative_flag, false);
    }
    // execute_test_16bit_reg_and_indexed_addr_with_8bitoffset
    #[test]
    fn test_reg_and_8bit_offset(){
        let code = "
        org 100h 
        .data 
        var dw 0x91
        code: 
            mov bx, 0x101 ; The place where var is stored
            mov ax, 0x0F0F 
            test ax, [bx + 0x01]
        ";
        let (cpu, _) = run_code(code, 4);
        assert_eq!(cpu.zero_flag, false);
        assert_eq!(cpu.pairity_flag, false);
        assert_eq!(cpu.negative_flag, false);
    }
    // execute_test_16bit_reg_and_indexed_addr_with_16bitoffset
    #[test]
    fn test_reg_and_16bit_offset(){
        let code = "
        org 100h 
        .data 
        var dw 0x91
        code: 
            mov bx, 0x2 ; The place where var is stored
            mov ax, 0x0F0F 
            test ax, [bx + 0x100]
        ";
        let (cpu, _) = run_code(code, 5);
        assert_eq!(cpu.zero_flag, false);
        assert_eq!(cpu.pairity_flag, false);
        assert_eq!(cpu.negative_flag, false);
    }

    // execute_test_8bit_reg_and_indexed_addr_variable
    #[test]
    fn test_8bit_reg_and_var(){
        let code = "
        org 100h 
        .data 
        var db 0x20
        code: 
            mov al, 0x0F 
            test al, [var]
        ";
        let (cpu, _) = run_code(code, 4);
        assert_eq!(cpu.zero_flag, true);
        assert_eq!(cpu.pairity_flag, true);
        assert_eq!(cpu.negative_flag, false);
    }

    // execute_test_8bit_reg_and_indexed_addr_no_offset
    #[test]
    fn test_8bit_reg_and_no_offset(){
        let code = "
        org 100h 
        .data 
        var db 0x91
        code: 
            mov bx, 0x102 ; The place where var is stored
            mov al, 0x0F 
            test al, [bx]
        ";
        let (cpu, _) = run_code(code, 5);
        assert_eq!(cpu.zero_flag, false);
        assert_eq!(cpu.pairity_flag, false);
        assert_eq!(cpu.negative_flag, false);
    }

    // execute_test_8bit_reg_and_indexed_addr_with_8bitoffset
    #[test]
    fn test_8bit_reg_and_8bit_offset(){
        let code = "
        org 100h 
        .data 
        var db 0x91
        code: 
            mov bx, 0x101 ; The place where var is stored
            mov al, 0x0F 
            test al, [bx + 0x01]
        ";
        let (cpu, _) = run_code(code, 4);
        assert_eq!(cpu.zero_flag, false);
        assert_eq!(cpu.pairity_flag, false);
        assert_eq!(cpu.negative_flag, false);
    }

    // execute_test_8bit_reg_and_indexed_addr_with_16bitoffset
    #[test]
    fn test_8bit_reg_and_16bit_offset(){
        let code = "
        org 100h 
        .data 
        var db 0x91
        code: 
            mov bx, 0x2 ; The place where var is stored
            mov al, 0x0F 
            test al, [bx + 0x100]
        ";
        let (cpu, _) = run_code(code, 5);
        assert_eq!(cpu.zero_flag, false);
        assert_eq!(cpu.pairity_flag, false);
        assert_eq!(cpu.negative_flag, false);
    }


}
