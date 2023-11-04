use crate::{
    cpu::CPU, generate_16bit_reg_8bit_reg_indexed_and_byte_indexed_addressing_as_first_ins_methods,
    generate_execute_ins_16bit_reg_and_number, generate_execute_ins_8bit_reg_and_number,
    generate_execute_ins_byte_addr_and_number, generate_execute_ins_word_addr_and_number,
    generate_ins_al_and_num, generate_ins_ax_and_num, memory::Memory,
};

fn cmp_16bit_and_set_flags(cpu: &mut CPU, val1: u16, val2: u16) -> Option<u16> {
    cpu.sub_16bit_with_overflow_and_set_flags(val1, val2);
    None
}

fn cmp_8bit_and_set_flags(cpu: &mut CPU, val1: u8, val2: u8) -> Option<u8> {
    cpu.sub_8bit_with_overflow_and_set_flags(val1, val2);
    None
}

impl CPU {
    generate_16bit_reg_8bit_reg_indexed_and_byte_indexed_addressing_as_first_ins_methods!(
        cmp,
        &cmp_16bit_and_set_flags,
        &cmp_8bit_and_set_flags
    );

    generate_ins_al_and_num!(cmp, &cmp_8bit_and_set_flags);

    generate_ins_ax_and_num!(cmp, &cmp_16bit_and_set_flags);

    generate_execute_ins_16bit_reg_and_number!(cmp, 0x83, &cmp_16bit_and_set_flags);

    generate_execute_ins_8bit_reg_and_number!(cmp, &cmp_8bit_and_set_flags);

    generate_execute_ins_word_addr_and_number!(cmp, 0x83, &cmp_16bit_and_set_flags);

    generate_execute_ins_byte_addr_and_number!(cmp, &cmp_8bit_and_set_flags);
}

#[cfg(test)]
mod cmp_tests {
    use crate::cpu::instructions::test_macro::execute_code;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_8bit_mem_and_reg() {
        let code = "
            MOV CH,      10101010b ; 170
            MOV [0x100], 01010101b ; 85
            CMP [0x100], CH
        ";
        let (cpu, _) = execute_code(code);
        assert_eq!(cpu.get_flags_as_binary(), 0b0010_1101);
    }

    #[test]
    fn test_16bit_mem_and_reg() {
        let code = "
        MOV SP, 0x100
        MOV [0x100], 0x1010
        CMP [0x100], SP
        ";

        let (cpu, _) = execute_code(code);
        assert_eq!(cpu.get_flags_as_binary(), 0b00);
    }

    #[test]
    fn cmp_8bit_reg_and_16bit_mem_or_reg() {
        let code = "
        MOV CH, 0x10
        MOV b.[0x100], 0x10
        CMP CH, [0x100]

        MOV BL, 0x10
        MOV BH, 0x01 
        CMP BL, BH

        MOV AL, 0x10
        MOV BP, 0x100
        MOV [0x100], 0x10
        CMP AL, [BP]
        ";
        let (cpu, _) = execute_code(code);
        assert_eq!(cpu.get_flags_as_binary(), 0b0001_0010);
    }

    #[test]
    fn cmp_16bit_reg_and_16bit_mem_or_reg() {
        let code = "
        MOV SP, 0x100
        MOV [0x100], 0x1010
        CMP SP, [0x100]

        MOV BP, 0x10
        MOV [0xA0], 0x1010
        CMP BP, [BP+0x90]

        MOV BX, 0x01 
        MOV AX, 0x10
        CMP BX, AX
        ";
        let (cpu, _) = execute_code(code);
        assert_eq!(cpu.get_flags_as_binary(), 0b0000_0101);
    }

    #[test]
    fn cmp_8bit_reg_and_num() {
        let code = "
        MOV AL, 0x10
        CMP AL, 0x10

        MOV BL, 0x10
        CMP BL, 0x01

        MOV [0x100], 0x10
        CMP b.[0x100], 0x11
        ";
        let (cpu, _) = execute_code(code);
        assert_eq!(cpu.get_flags_as_binary(), 0b0011_0101);
    }

    #[test]
    fn cmp_16bit_reg_and_num() {
        let code = "
        MOV SP, 0x100
        CMP SP, 0x10

        MOV BP, 0x100
        CMP BP, 0x10

        MOV AX, 0x100
        CMP AX, 0x10
        ";
        let (cpu, _) = execute_code(code);
        assert_eq!(cpu.get_flags_as_binary(), 0b0001_0000);
    }
}
