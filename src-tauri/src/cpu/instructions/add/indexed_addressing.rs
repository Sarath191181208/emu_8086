use crate::{consts::U20, cpu::CPU, memory::Memory};

impl CPU {
    fn add_8_bit_register_and_mem_offset(&mut self, mem: &mut Memory, reg_idx: u8, offset: U20) {
        // read the data from memory ex:
        let data = self.read_byte_from_u20(mem, offset);
        // read the value of the register
        let reg_val = self.get_8bit_register_by_index(reg_idx);
        // sub the values with the overflows and set the flags
        let (result, _) = self.add_8bit_with_overflow_and_set_flags(reg_val, data);
        // set the value in the respective register
        self.set_8bit_register_by_index(reg_idx, result);
    }

    pub(super) fn add_16bit_register_indexed_registers_without_offset(
        &mut self,
        mem: &mut Memory,
        ins: u8,
    ) {
        let (low_reg_idx, high_reg_idx) = self.get_index_from_0x00_0x3f_pattern(ins);
        match low_reg_idx {
            0x06 => {
                // add reg, [0x1234]
                self.add_16bit_reg_direct_address(mem, high_reg_idx);
            }
            _ => {
                // get offset
                let memory_offset = self.get_offset_from_index_of_indexed_registers(low_reg_idx);
                // read the data from memeory
                let data = self.read_word_from_u20(mem, memory_offset);
                // read the value of the register
                let reg_val = self.get_16bit_register_by_index(high_reg_idx);
                // add the values with the overflows and set the flags
                let (result, _) = self.add_16bit_with_overflow_and_set_flags(reg_val, data);
                // set the value in the respective register
                self.set_16bit_register_by_index(high_reg_idx, result);
            }
        }
    }

    pub(super) fn add_8bit_register_indexed_registers_without_offset(
        &mut self,
        mem: &mut Memory,
        ins: u8,
    ) {
        let (low_reg_idx, high_reg_idx) = self.get_index_from_0x00_0x3f_pattern(ins);
        match low_reg_idx {
            0x06 => {
                // add reg, [0x1234]
                self.add_8bit_register_direct_address(mem, high_reg_idx);
            }
            _ => {
                // get offset
                let memory_offset = self.get_offset_from_index_of_indexed_registers(low_reg_idx);
                self.add_8_bit_register_and_mem_offset(mem, high_reg_idx, memory_offset);
            }
        }
    }

    pub(super) fn add_16bit_register_indexed_registers_with_8bit_offset(
        &mut self,
        mem: &mut Memory,
        ins: u8,
    ) {
        let (low_reg_idx, high_reg_idx) = self.get_index_from_0x40_0x7f_pattern(ins);
        let offset = U20::from(self.consume_byte(mem));
        let memory_offset = self.get_offset_from_index_of_indexed_registers(low_reg_idx);
        let data = self.read_word_from_u20(mem, memory_offset + offset);

        let reg_val = self.get_16bit_register_by_index(high_reg_idx);
        let (res, _) = self.add_16bit_with_overflow_and_set_flags(data, reg_val);
        self.set_16bit_register_by_index(high_reg_idx, res);
    }

    pub(super) fn add_8bit_register_indexed_registers_with_8bit_offset(
        &mut self,
        mem: &mut Memory,
        ins: u8,
    ) {
        let (low_reg_idx, high_reg_idx) = self.get_index_from_0x40_0x7f_pattern(ins);
        // getting the offset defined in ins i.e 0x20
        let offset = U20::from(self.consume_byte(mem));
        // getting the offset from the index of indexed registers i.e from [bx+si] | [bx]
        let memory_offset = self.get_offset_from_index_of_indexed_registers(low_reg_idx);
        // perform SUB AL..BH, [BX+SI]...[Bx] + 8bit-Offset
        self.add_8_bit_register_and_mem_offset(mem, high_reg_idx, memory_offset + offset);
    }

    pub(super) fn add_16bit_register_indexed_registers_with_16bit_offset(
        &mut self,
        mem: &mut Memory,
        ins: u8,
    ) {
        let (low_reg_idx, high_reg_idx) = self.get_index_from_0x80_0xbf_pattern(ins);
        let offset = U20::from(self.consume_word(mem));
        let memory_offset = self.get_offset_from_index_of_indexed_registers(low_reg_idx);
        let data = self.read_word_from_u20(mem, memory_offset + offset);
        self.set_16bit_register_by_index(high_reg_idx, data);
    }

    pub(super) fn add_8bit_register_indexed_registers_with_16bit_offset(
        &mut self,
        mem: &mut Memory,
        ins: u8,
    ) {
        // getting the register index from the ins go to def to understand more
        let (low_reg_idx, high_reg_idx) = self.get_index_from_0x80_0xbf_pattern(ins);
        // getting the offset defined in ins i.e 0x100
        let offset = U20::from(self.consume_word(mem));
        // getting the offset from the index of indexed registers i.e from [bx+si] | [bx]
        let memory_offset: U20 = self.get_offset_from_index_of_indexed_registers(low_reg_idx);
        // perform SUB AL..DH, [BX+SI]...[Bx] + 16bit-Offset
        self.add_8_bit_register_and_mem_offset(mem, high_reg_idx, memory_offset + offset);
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::instructions::test_macro::run_code;

    #[test]
    fn no_offset_indexed_add() {
        let code = "
        org 100h
        .data 
        var dw 0x1234
        code: 
        mov bx, 0x100 
        mov si, 0x02
        add ax, [bx+si]
        ";
        let (cpu, _) = run_code(code, 4);
        assert_eq!(cpu.ax, 0x1234);
    }

    #[test]
    fn only_offset_indexed_add() {
        let code = " 
        add ax, [0x1000]
        ";
        let (cpu, _) = run_code(code, 1);
        assert_eq!(cpu.ax, 0x9090);
    }

    #[test]
    fn offset_8bit_index_add() {
        let code = "
        org 100h
        .data 
        _var db 0x20
        var dw 0x1234
        code: 
        mov bx, 0x100 
        mov si, 0x05
        add ax, [bx+si-0x02]
        ";
        let (cpu, _) = run_code(code, 4);
        assert_eq!(cpu.ax, 0x1234);
    }

    #[test]
    fn offset_16bit_index_add() {
        let code = "
        org 100h
        .data 
        _var dw 0x20
        var dw 0x1234
        code: 
        mov bx, 0x02
        mov si, 0x02
        add ax, [bx+si+0x100]
        ";
        let (cpu, _) = run_code(code, 4);
        assert_eq!(cpu.ax, 0x1234);
    }
}

#[cfg(test)]
mod tests_8bit {
    use crate::cpu::instructions::test_macro::run_code;

    #[test]
    fn offset_8bit_register_and_offset() {
        let code = "
        org 100h
        .data
        _var db 0x20
        var db 0x12
        code:
        add al, [0x102]
        ";
        let (cpu, _) = run_code(code, 4);
        assert_eq!(cpu.get_ax_low(), 0x20);
    }

    #[test]
    fn offset_8bit_index_add_8bit() {
        let code = "
        org 100h
        .data
        _var db 0x20
        var db 0x12
        code:
        mov bx, 0x100
        mov si, 0x05
        add al, [bx+si-0x02]
        ";
        let (cpu, _) = run_code(code, 4);
        assert_eq!(cpu.get_ax_low(), 0x12);
    }

    #[test]
    fn offset_16bit_index_add_8bit() {
        let code = "
        org 100h
        .data
        _var dw 0x20
        var db 0x12
        code:
        mov bx, 0x02
        mov si, 0x02
        add al, [bx+si+0x100]
        ";
        let (cpu, _) = run_code(code, 4);
        assert_eq!(cpu.get_ax_low(), 0x12);
    }
}
