use crate::{consts::U20, cpu::CPU, memory::Memory};

impl CPU {
    fn sub_16_bit_register_and_mem_offset(&mut self, mem: &mut Memory, reg_idx: u8, offset: U20) {
        // read the data from memory ex:
        let data = self.read_word_from_u20(mem, offset);
        // read the value of the register
        let reg_val = self.get_16bit_register_by_index(reg_idx);
        // sub the values with the overflows and set the flags
        let (result, _) = self.sub_16bit_with_overflow_and_set_flags(reg_val, data);
        // set the value in the respective register
        self.set_16bit_register_by_index(reg_idx, result);
    }

    fn sub_8_bit_register_and_mem_offset(&mut self, mem: &mut Memory, reg_idx: u8, offset: U20) {
        // read the data from memory ex:
        let data = self.read_byte_from_u20(mem, offset);
        // read the value of the register
        let reg_val = self.get_8bit_register_by_index(reg_idx);
        // sub the values with the overflows and set the flags
        let (result, _) = self.sub_8bit_with_overflow_and_set_flags(reg_val, data);
        // set the value in the respective register
        self.set_8bit_register_by_index(reg_idx, result);
    }

    pub(super) fn sub_16bit_register_indexed_registers_without_offset(
        &mut self,
        mem: &mut Memory,
        ins: u8,
    ) {
        let (low_reg_idx, high_reg_idx) = self.get_index_from_0x00_0x3f_pattern(ins);
        match low_reg_idx {
            0x06 => {
                // sub reg, [0x1234]
                self.sub_16bit_reg_direct_address(mem, high_reg_idx);
            }
            _ => {
                // get offset [bx+si] -> Offset(bx) + Offset(si) + Offset(data_segment)
                let memory_offset = self.get_offset_from_index_of_indexed_registers(low_reg_idx);
                // perform SUB AX..DI, [BX+SI]...[Bx]
                self.sub_16_bit_register_and_mem_offset(mem, high_reg_idx, memory_offset);
            }
        }
    }

    pub(super) fn sub_8bit_register_indexed_registers_without_offset(
        &mut self,
        mem: &mut Memory,
        ins: u8,
    ) {
        let (low_reg_idx, high_reg_idx) = self.get_index_from_0x00_0x3f_pattern(ins);
        match low_reg_idx {
            0x06 => {
                // sub reg, [0x1234]
                self.sub_8bit_register_direct_address(mem, high_reg_idx);
            }
            _ => {
                // get offset [bx+si] -> Offset(bx) + Offset(si) + Offset(data_segment)
                let memory_offset = self.get_offset_from_index_of_indexed_registers(low_reg_idx);
                // perform SUB AL..DH, [BX+SI]...[Bx]
                self.sub_8_bit_register_and_mem_offset(mem, high_reg_idx, memory_offset);
            }
        }
    }

    pub(super) fn sub_16bit_register_indexed_registers_with_8bit_offset(
        &mut self,
        mem: &mut Memory,
        ins: u8,
    ) {
        let (low_reg_idx, high_reg_idx) = self.get_index_from_0x40_0x7f_pattern(ins);
        // the method consumes and adds the offset to the memory offset
        let mem_offset_plus_8bit_offset =
            self.consume_byte_and_get_cummulative_offset(mem, low_reg_idx);
        // perform SUB AX..DI, [BX+SI]...[Bx] + 8bit-Offset
        self.sub_16_bit_register_and_mem_offset(mem, high_reg_idx, mem_offset_plus_8bit_offset);
    }

    pub(super) fn sub_8bit_register_indexed_registers_with_8bit_offset(
        &mut self,
        mem: &mut Memory,
        ins: u8,
    ) {
        let (low_reg_idx, high_reg_idx) = self.get_index_from_0x40_0x7f_pattern(ins);
        // the method consumes and adds the offset to the memory offset
        let mem_offset_plus_8bit_offset =
            self.consume_byte_and_get_cummulative_offset(mem, low_reg_idx);
        // perform SUB AL..DH, [BX+SI]...[Bx] + 8bit-Offset
        self.sub_8_bit_register_and_mem_offset(mem, high_reg_idx, mem_offset_plus_8bit_offset);
    }

    pub(super) fn sub_16bit_register_indexed_registers_with_16bit_offset(
        &mut self,
        mem: &mut Memory,
        ins: u8,
    ) {
        // getting the register index from the ins go to def to understand more
        let (low_reg_idx, high_reg_idx) = self.get_index_from_0x80_0xbf_pattern(ins);
        // getting the offset defined in ins + 0x100
        let mem_offset_plus_16bit_offset =
            self.consume_word_and_get_cummulative_offset(mem, low_reg_idx);
        // perform SUB AX..DI, [BX+SI]...[Bx] + 16bit-Offset
        self.sub_16_bit_register_and_mem_offset(mem, high_reg_idx, mem_offset_plus_16bit_offset);
    }

    pub(super) fn sub_8bit_register_indexed_registers_with_16bit_offset(
        &mut self,
        mem: &mut Memory,
        ins: u8,
    ) {
        // getting the register index from the ins go to def to understand more
        let (low_reg_idx, high_reg_idx) = self.get_index_from_0x80_0xbf_pattern(ins);
        // getting the offset defined in ins + 0x100
        let mem_offset_plus_16bit_offset =
            self.consume_word_and_get_cummulative_offset(mem, low_reg_idx);
        // perform SUB AX..DI, [BX+SI]...[Bx] + 16bit-Offset
        self.sub_8_bit_register_and_mem_offset(mem, high_reg_idx, mem_offset_plus_16bit_offset);
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        cpu::{instructions::test_macro::compile_and_test_str, CPU},
        memory::Memory,
    };

    #[test]
    fn no_offset_indexed_sub() {
        compile_and_test_str(
            "
            org 100h
            .data 
            var dw 0x1234
            code: 
            mov bx, 0x100 
            mov si, 0x02
            sub ax, [bx+si]
            ",
            4,
            |cpu: &CPU, _: &Memory| {
                assert_eq!(cpu.ax, 0xEDCC);
            },
        );
    }

    #[test]
    fn only_offset_indexed_sub() {
        compile_and_test_str(
            " 
            sub dx, [0x1000]
            ",
            1,
            |cpu: &CPU, _: &Memory| {
                assert_eq!(cpu.dx, 0x6F70);
            },
        );
    }

    #[test]
    fn offset_8bit_index_sub() {
        compile_and_test_str(
            "
            org 100h
            .data 
            _var db 0x20
            var dw 0x1234
            code: 
            mov bx, 0x100 
            mov si, 0x05
            sub ax, [bx+si-0x02]
            ",
            4,
            |cpu: &CPU, _: &Memory| {
                assert_eq!(cpu.ax, 0xEDCC);
            },
        );
    }

    #[test]
    fn offset_16bit_index_sub() {
        compile_and_test_str(
            "
            org 100h
            .data 
            _var dw 0x20
            var dw 0x1234
            code: 
            mov bx, 0x02
            mov si, 0x02
            sub ax, [bx+si+0x100]
            ",
            4,
            |cpu: &CPU, _: &Memory| {
                assert_eq!(cpu.ax, 0xEDCC);
            },
        );
    }
}

#[cfg(test)]
mod tests_8bit {
    use crate::{cpu::{CPU, instructions::test_macro::compile_and_test_str}, memory::Memory};

    #[test]
    fn offset_8bit_index_sub_8bit() {
        compile_and_test_str(
            "
    org 100h
    .data
    _var db 0x20
    var db 0x12
    code:
    mov bx, 0x100
    mov si, 0x05
    sub al, [bx+si-0x02]
    ",
            4,
            |cpu: &CPU, _: &Memory| {
                assert_eq!(cpu.get_ax_low(), 0xEE);
            },
        );
    }

    #[test]
    fn offset_16bit_index_sub_8bit() {
        compile_and_test_str(
            "
    org 100h
    .data
    _var dw 0x20
    var db 0x12
    code:
    mov bx, 0x02
    mov si, 0x02
    sub al, [bx+si+0x100]
    ",
            4,
            |cpu: &CPU, _: &Memory| {
                assert_eq!(cpu.get_ax_low(), 0xEE);
            },
        );
    }
}
