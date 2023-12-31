use crate::{consts::Byte, cpu::CPU, memory::Memory};

impl CPU {
    pub(super) fn sub_16bit_reg_direct_address(&mut self, mem: &mut Memory, reg_index: u8) {
        // read the data from memeory
        let data_addr = self.consume_word(mem);
        let data = self.read_word_from_pointer(mem, data_addr);
        // read the value of the register
        let reg_val = self.get_16bit_register_by_index(reg_index);
        // sub the values with the overflows and set the flags
        let (result, _) = self.sub_16bit_with_overflow_and_set_flags(reg_val, data);
        // set the value in the respective register
        self.set_16bit_register_by_index(reg_index, result);
    }

    pub(super) fn sub_8bit_register_direct_address(&mut self, mem: &mut Memory, reg_index: u8) {
        // read the data from memeory
        let data_addr = self.consume_word(mem);
        let data = self.read_byte_from_pointer(mem, data_addr);
        // read the value of the register
        let reg_val = self.get_8bit_register_by_index(reg_index);
        // sub the values with the overflows and set the flags
        let (result, _) = self.sub_8bit_with_overflow_and_set_flags(reg_val, data);
        // set the value in the respective register
        self.set_8bit_register_by_index(reg_index, result);
    }

    pub(in crate::cpu) fn execute_sub_direct_addr_8bit_register(&mut self, mem: &mut Memory) {
        // get the register index i.e ax, bx, cx, dx -> 0, 1, 2, 3
        let ins = self.consume_instruction(mem);
        let reg_idx = self.get_index_from_06_e6_pattern(ins);
        let reg = self.get_8bit_register_by_index(reg_idx);
        // get the address of the data `
        let data_addr = self.consume_word(mem);
        // get the data from the memory
        let data = self.read_byte_from_pointer(mem, data_addr);
        // add the values with the overflows and set the flags
        let (result, _) = self.sub_8bit_with_overflow_and_set_flags(data, reg);
        // write the data to the memory
        self.write_byte_from_pointer(mem, data_addr, result);
    }

    pub(in crate::cpu) fn sub_direct_address_16bit_val_immediate_value(
        &mut self,
        mem: &mut Memory,
        ins: Byte,
    ) {
        let _ = self.consume_instruction(mem); // 0x06
        let data_address = self.consume_word(mem);
        match ins {
            0x81 => {
                // 16 bit add
                let data_from_mem = self.read_word_from_pointer(mem, data_address);
                let num: u16 = self.consume_word(mem);
                let (result, _) = self.sub_16bit_with_overflow_and_set_flags(data_from_mem, num);
                self.write_word_from_pointer(mem, data_address, result);
            }
            0x83 => {
                // 8bit add
                let data_from_mem = self.read_word_from_pointer(mem, data_address);
                let num: u16 = self.consume_byte(mem) as u16;
                let (result, _) = self.sub_16bit_with_overflow_and_set_flags(data_from_mem, num);
                self.write_word_from_pointer(mem, data_address, result);
            }
            _ => unimplemented!("ADD instruction not implemented! for {}", ins),
        }
    }
    pub(in crate::cpu) fn sub_direct_address_8bit_val_immediate_value(&mut self, mem: &mut Memory) {
        let _ = self.consume_instruction(mem); // 0x06
        let data_addr = self.consume_word(mem);
        let data_from_mem = self.read_byte_from_pointer(mem, data_addr);
        let num: u8 = self.consume_byte(mem);
        let (result, _) = self.sub_8bit_with_overflow_and_set_flags(data_from_mem, num);
        self.write_byte_from_pointer(mem, data_addr, result);
    }
}

#[cfg(test)]
mod test_add_direct_address {
    use crate::cpu::instructions::test_macro::execute_code;

    #[test]
    fn test_ax_var() {
        let code = "
        org 0x100
        .data 
        var dw 0x1234
        code: 
        SUB AX, var
    ";
        let (cpu, _) = execute_code(code);

        assert_eq!(cpu.ax, 0xEDCC);
        assert_eq!(cpu.get_flags_as_binary(), 0b0011_0101);
    }

    #[test]
    fn test_var_sp() {
        let code = "
        org 0x100
        .data 
        var dw 0x1234
        code: 
        mov sp, 0x01
        SUB VAR, SP
    ";
        let (cpu, mem) = execute_code(code);

        assert_eq!(cpu.read_word_from_pointer(&mem, 0x102), 0x1233);
        assert_eq!(cpu.get_flags_as_binary(), 0b0001_0000);
    }

    #[test]
    fn test_var_0x1000() {
        let code = "
        org 0x100
        .data 
        var dw 0x1234
        code: 
        SUB var, 0x1000
    ";
        let (cpu, mem) = execute_code(code);

        assert_eq!(cpu.read_word_from_pointer(&mem, 0x102), 0x0234);
        assert_eq!(cpu.get_flags_as_binary(), 0b0000_0000);
    }

    #[test]
    fn test_var_0x10() {
        let code = "
        org 0x100
        .data 
        var dw 0x1234
        code: 
        SUB var, 0x10
    ";
        let (cpu, mem) = execute_code(code);

        assert_eq!(cpu.read_word_from_pointer(&mem, 0x102), 0x1224);
        assert_eq!(cpu.get_flags_as_binary(), 0b0001_0000);
    }

    #[test]
    fn test_cl_var() {
        let code = "
        org 0x100
        .data 
        var db 0x12
        code: 
        MOV CL, 0x10
        SUB CL, var
    ";
        let (cpu, _) = execute_code(code);

        assert_eq!(cpu.cx, 0x00FE);
        assert_eq!(cpu.get_flags_as_binary(), 0b0010_0101);
    }

    #[test]
    fn test_var_dl() {
        let code = "
        org 0x100
        .data 
        var db 0x12
        code: 
        MOV DL, 0x10
        SUB var, DL
    ";
        let (cpu, mem) = execute_code(code);

        assert_eq!(cpu.read_byte_from_pointer(&mem, 0x102), 0x02);
        assert_eq!(cpu.get_flags_as_binary(), 0b0000_0000);
    }

    #[test]
    fn test_var_0x20() {
        let code = "
        org 0x100
        .data 
        var db 0x12
        code: 
        SUB var, 0x20
    ";
        let (cpu, mem) = execute_code(code);

        assert_eq!(cpu.read_byte_from_pointer(&mem, 0x102), 0xF2);
        assert_eq!(cpu.get_flags_as_binary(), 0b0000_0101);
    }
}
