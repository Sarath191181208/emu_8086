use crate::{
    cpu::CPU, generate_16bit_reg_8bit_reg_indexed_and_byte_indexed_addressing_as_first_ins_methods,
    generate_execute_ins_16bit_reg_and_number, generate_execute_ins_8bit_reg_and_number,
    generate_execute_ins_byte_addr_and_number, generate_execute_ins_word_addr_and_number,
    generate_ins_al_and_num, generate_ins_ax_and_num, memory::Memory,
};

fn or_16bit_and_set_flags(cpu: &mut CPU, val1: u16, val2: u16) -> Option<u16> {
    let res = val1 | val2;
    cpu.set_or_ins_flags_from_16bit_res(res);
    Some(res)
}

fn or_8bit_and_set_flags(cpu: &mut CPU, val1: u8, val2: u8) -> Option<u8> {
    let res = val1 | val2;
    cpu.set_or_ins_flags_from_8bit_res(res, val1, val2);
    Some(res)
}

impl CPU {
    generate_16bit_reg_8bit_reg_indexed_and_byte_indexed_addressing_as_first_ins_methods!(
        or,
        &or_16bit_and_set_flags,
        &or_8bit_and_set_flags
    );

    generate_ins_al_and_num!(or, &or_8bit_and_set_flags);

    generate_ins_ax_and_num!(or, &or_16bit_and_set_flags);

    generate_execute_ins_16bit_reg_and_number!(or, 0x83, &or_16bit_and_set_flags);

    generate_execute_ins_8bit_reg_and_number!(or, &or_8bit_and_set_flags);

    generate_execute_ins_word_addr_and_number!(or, 0x83, &or_16bit_and_set_flags);

    generate_execute_ins_byte_addr_and_number!(or, &or_8bit_and_set_flags);

    fn set_or_ins_flags_from_16bit_res(&mut self, res: u16) {
        self.carry_flag = false;
        self.overflow_flag = false;
        self.zero_flag = res == 0;
        self.set_pairity_flag_from_16bit_res(res);
        self.set_negative_flag_from_16bit_res(res);
    }

    fn set_or_ins_flags_from_8bit_res(&mut self, res: u8, _val1: u8, _val2: u8) {
        self.carry_flag = false;
        self.overflow_flag = false;
        self.zero_flag = res == 0;
        self.set_negative_flag_from_8bit_res(res);
        self.set_pairity_flag_from_8bit_res(res);
        // self.set_auxillary_flag_from_nums(val1, val2);
    }
}

#[cfg(test)]
mod or_execution_tests {
    use crate::cpu::instructions::test_macro::execute_code;

    #[test]
    fn test_or_ax_and_num() {
        let code = "
        mov ax, 0x101 
        or ax, 0x010
        ";
        let (cpu, _) = execute_code(code);
        assert_eq!(cpu.ax, 0x111);
        assert_eq!(cpu.get_flags_as_binary(), 0b0001_0000)
    }

    #[test]
    fn test_or_al_and_num_with_aux_overflow() {
        let code = "
        mov al, 0xF0
        or al, 0x0F
        ";
        let (cpu, _) = execute_code(code);
        assert_eq!(cpu.get_ax_low(), 0xFF);
        assert_eq!(cpu.get_flags_as_binary(), 0b0001_0100);
    }

    #[test]
    fn test_or_reg_and_num() {
        let code = "
        mov ax, 0x101
        or ax, 0x010
        ";
        let (cpu, _) = execute_code(code);
        assert_eq!(cpu.ax, 0x111);
        assert_eq!(cpu.get_flags_as_binary(), 0b0001_0000);
    }

    #[test]
    fn test_8bit_reg_and_num() {
        let code = "
        mov al, 0x01
        or al, 0x10
        ";
        let (cpu, _) = execute_code(code);
        assert_eq!(cpu.get_ax_low(), 0x11);
        assert_eq!(cpu.get_flags_as_binary(), 0b0001_0000);
    }

    #[test]
    fn test_or_reg_reg() {
        let code = "
        mov sp, 0x1010
        mov bp, 0x0101
        or sp, bp
        ";
        let (cpu, _) = execute_code(code);
        assert_eq!(cpu.stack_pointer, 0x1111);
        assert_eq!(cpu.get_flags_as_binary(), 0b0001_0000);
    }

    #[test]
    fn test_or_reg_mem() {
        let code = "
        mov ax, 0x1010
        mov [0x100], 0x1100
        or ax, [0x100]
        ";
        let (cpu, _) = execute_code(code);
        assert_eq!(cpu.ax, 0x1110);
        assert_eq!(cpu.get_flags_as_binary(), 0b0000_0000);
    }

    #[test]
    fn test_or_mem_reg() {
        let code = "
        org 100h
        .data 
            var dw  0x0101
        code:
            mov ax, 0x1010
            mov [0x100], 0x1100
            or [0x100], ax

            mov bx, 0x100 
            or [bx+0x02], ax
        ";
        let (cpu, mem) = execute_code(code);
        assert_eq!(cpu.read_word_from_pointer(&mem, 0x100), 0x1110);
        assert_eq!(cpu.read_word_from_pointer(&mem, 0x102), 0x1111);
    }

    #[test]
    fn test_or_mem_num() {
        let code = "
        mov [0x100], 0x1100
        or [0x100], 0x0011
        ";
        let (cpu, mem) = execute_code(code);
        assert_eq!(cpu.read_word_from_pointer(&mem, 0x100), 0x1111);
        assert_eq!(cpu.get_flags_as_binary(), 0b0001_0000);
    }
}
