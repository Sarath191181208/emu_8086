use crate::{consts::Byte, cpu::CPU, memory::Memory};

impl CPU {
    pub(super) fn add_16bit_reg_direct_address(&mut self, mem: &mut Memory, reg_index: u8) {
        // read the data from memeory
        let data_addr = self.consume_word(mem);
        let data = self.read_word_from_pointer(mem, data_addr);
        // read the value of the register
        let reg_val = self.get_16bit_register_by_index(reg_index);
        // add the values with the overflows and set the flags
        let (result, _) = self.add_16bit_with_overflow_and_set_flags(reg_val, data);
        // set the value in the respective register
        self.set_16bit_register_by_index(reg_index, result);
    }

    pub(super) fn add_8bit_register_direct_address(&mut self, mem: &mut Memory, reg_index: u8) {
        // read the data from memeory
        let data_addr = self.consume_word(mem);
        let data = self.read_byte_from_pointer(mem, data_addr);
        // read the value of the register
        let reg_val = self.get_8bit_register_by_index(reg_index);
        // add the values with the overflows and set the flags
        let (result, _) = self.add_8bit_with_overflow_and_set_flags(reg_val, data);
        // set the value in the respective register
        self.set_8bit_register_by_index(reg_index, result);
    }

    pub(in crate::cpu) fn execute_add_address_and_16bit_register(&mut self, mem: &mut Memory) {
        // get the register index i.e ax, bx, cx, dx -> 0, 1, 2, 3
        let ins = self.consume_instruction(mem);
        let reg_idx = self.get_index_from_06_e6_pattern(ins);
        let reg = self.get_16bit_register_by_index(reg_idx);
        // get the address of the data `
        let data_addr = self.consume_word(mem);
        // get the data from the memory
        let data = self.read_word_from_pointer(mem, data_addr);
        // add the values with the overflows and set the flags
        let (result, _) = self.add_16bit_with_overflow_and_set_flags(reg, data);
        // write the data to the memory
        self.write_word_from_pointer(mem, data_addr, result);
    }

    pub(in crate::cpu) fn execute_add_address_and_8bit_register(&mut self, mem: &mut Memory) {
        // get the register index i.e ax, bx, cx, dx -> 0, 1, 2, 3
        let ins = self.consume_instruction(mem);
        let reg_idx = self.get_index_from_06_e6_pattern(ins);
        let reg = self.get_8bit_register_by_index(reg_idx);
        // get the address of the data `
        let data_addr = self.consume_word(mem);
        // get the data from the memory
        let data = self.read_byte_from_pointer(mem, data_addr);
        // add the values with the overflows and set the flags
        let (result, _) = self.add_8bit_with_overflow_and_set_flags(reg, data);
        // write the data to the memory
        self.write_byte_from_pointer(mem, data_addr, result);
    }

    pub(in crate::cpu) fn add_direct_address_16bit_val_immediate_value(
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
                let (result, _) = self.add_16bit_with_overflow_and_set_flags(data_from_mem, num);
                self.write_word_from_pointer(mem, data_address, result);
            }
            0x83 => {
                // 8bit add
                let data_from_mem = self.read_word_from_pointer(mem, data_address);
                let num: u16 = self.consume_byte(mem) as u16;
                let (result, _) = self.add_16bit_with_overflow_and_set_flags(data_from_mem, num);
                self.write_word_from_pointer(mem, data_address, result);
            }
            _ => unimplemented!("ADD instruction not implemented! for {}", ins),
        }
    }

    pub(in crate::cpu) fn add_direct_address_8bit_val_immediate_value(&mut self, mem: &mut Memory) {
        let _ = self.consume_instruction(mem); // 0x06
        let data_addr = self.consume_word(mem);
        let data_from_mem = self.read_byte_from_pointer(mem, data_addr);
        let num: u8 = self.consume_byte(mem);
        let (result, _) = self.add_8bit_with_overflow_and_set_flags(data_from_mem, num);
        self.write_byte_from_pointer(mem, data_addr, result);
    }
}

#[cfg(test)]
mod test_add_direct_address {
    use crate::cpu::instructions::test_macro::run_code;


    #[test]
    fn test_ax_var() {
        let code = "
            org 0x100
            .data 
            var dw 0x1234
            code: 
            ADD AX, var
            ";
        let (cpu, _) = run_code(code, 2);
        assert_eq!(cpu.ax, 0x1234);
    }

    #[test]
    fn test_al_var() {
        let code = "
            org 0x100
            .data 
            var db 0x12
            code: 
            ADD AL, var
            ";
        let (cpu, _) = run_code(code, 2);
        assert_eq!(cpu.ax, 0x0012);
    }

    #[test]
    fn test_var_bx() {
        let code = "
            org 0x100
            .data 
            var dw 0x1234
            code: 
            mov bx, 0x1111
            ADD var, BX
            ";
        let (cpu, mem) = run_code(code, 3);
        assert_eq!(cpu.read_word_from_pointer(&mem, 0x102), 0x2345);
    }

    #[test]
    fn test_var_cl() {
        let code = "
            org 0x100
            .data 
            var db 0x12
            code: 
            mov cl, 0x11
            ADD var, cl
            ";
        let (cpu, mem) = run_code(code, 3);
        assert_eq!(cpu.read_byte_from_pointer(&mem, 0x102), 0x23);
    }

    #[test]
    fn test_var_16bit_immediate() {
        let code = "
            org 0x100
            .data 
            var dw 0x1234
            code: 
            ADD var, 0x1111
            ";
        let (cpu, mem) = run_code(code, 3);
        assert_eq!(cpu.read_word_from_pointer(&mem, 0x102), 0x2345);
    }

    #[test]
    fn test_var_word_8bit_immediate() {
        let code = "
            org 0x100
            .data 
            var dw 0x1234
            code: 
            ADD var, 0x11
            ";
        let (cpu, mem) = run_code(code, 3);
        assert_eq!(cpu.read_byte_from_pointer(&mem, 0x102), 0x45);
    }

    #[test]
    fn test_var_byte_8bit_immediate() {
        let code = "
            org 0x100
            .data 
            var db 0x10
            code: 
            ADD var, 0x20
            ";
        let (cpu, mem) = run_code(code, 3);
        assert_eq!(cpu.read_byte_from_pointer(&mem, 0x102), 0x30);
    }
}
