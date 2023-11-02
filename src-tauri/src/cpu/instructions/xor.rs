use crate::{
    cpu::CPU, generate_16bit_reg_8bit_reg_indexed_and_byte_indexed_addressing_as_first_ins_methods,
    generate_execute_ins_16bit_reg_and_number, generate_execute_ins_8bit_reg_and_number,
    generate_execute_ins_byte_addr_and_number, generate_execute_ins_word_addr_and_number,
    generate_ins_al_and_num, generate_ins_ax_and_num, memory::Memory,
};

fn xor_16bit_and_set_flags(cpu: &mut CPU, val1: u16, val2: u16) -> Option<u16> {
    let res = val1 ^ val2;
    cpu.set_xor_ins_flags_from_16bit_res(res);
    Some(res)
}

fn xor_8bit_and_set_flags(cpu: &mut CPU, val1: u8, val2: u8) -> Option<u8> {
    let res = val1 ^ val2;
    cpu.set_xor_ins_flags_from_8bit_res(res, val1, val2);
    Some(res)
}

impl CPU {
    generate_16bit_reg_8bit_reg_indexed_and_byte_indexed_addressing_as_first_ins_methods!(
        xor,
        &xor_16bit_and_set_flags,
        &xor_8bit_and_set_flags
    );

    generate_ins_al_and_num!(xor, &xor_8bit_and_set_flags);

    generate_ins_ax_and_num!(xor, &xor_16bit_and_set_flags);

    generate_execute_ins_16bit_reg_and_number!(xor, 0x83, &xor_16bit_and_set_flags);

    generate_execute_ins_8bit_reg_and_number!(xor, &xor_8bit_and_set_flags);

    generate_execute_ins_word_addr_and_number!(xor, 0x83, &xor_16bit_and_set_flags);

    generate_execute_ins_byte_addr_and_number!(xor, &xor_8bit_and_set_flags);

    fn set_xor_ins_flags_from_16bit_res(&mut self, res: u16) {
        self.carry_flag = false;
        self.overflow_flag = false;
        self.zero_flag = res == 0;
        self.set_pairity_flag_from_16bit_res(res);
        self.set_negative_flag_from_16bit_res(res);
    }

    fn set_xor_ins_flags_from_8bit_res(&mut self, res: u8, _val1: u8, _val2: u8) {
        self.carry_flag = false;
        self.overflow_flag = false;
        self.zero_flag = res == 0;
        self.set_negative_flag_from_8bit_res(res);
        self.set_pairity_flag_from_8bit_res(res);
        // self.set_auxillary_flag_from_nums(val1, val2);
    }
}

#[cfg(test)]
mod xor_tests {
    use crate::cpu::instructions::test_macro::run_code;

    #[test]
    fn test_8bit_mem_and_reg() {
        let code = "
            MOV CH,      10101010b
            MOV [0x100], 01010101b
            XOR [0x100], CH
        ";
        let (cpu, mem) = run_code(code, 3);
        assert_eq!(cpu.read_byte_from_pointer(&mem, 0x100), 0b11111111);
    }

    #[test]
    fn test_16bit_mem_and_reg() {
        let code = "
        MOV SP, 0x100
        MOV [0x100], 0x1010
        XOR [0x100], SP
        ";

        let (cpu, mem) = run_code(code, 3);
        assert_eq!(cpu.read_word_from_pointer(&mem, 0x100), 0x1110);
    }

    #[test]
    fn xor_8bit_reg_and_16bit_mem_or_reg() {
        let code = "
        MOV CH, 0x10
        MOV b.[0x100], 0x10
        XOR CH, [0x100]

        MOV BL, 0x10
        MOV BH, 0x01 
        XOR BL, BH

        MOV AL, 0x10
        MOV BP, 0x100
        MOV [0x100], 0x10
        XOR AL, [BP]
        ";
        let (cpu, _) = run_code(code, 10);
        assert_eq!(cpu.get_cx_high(), 0x00);
        assert_eq!(cpu.get_bx_low(), 0x11);
        assert_eq!(cpu.get_ax_low(), 0x00);
    }
}
