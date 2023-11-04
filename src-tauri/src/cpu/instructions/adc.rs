use crate::{
    cpu::CPU, generate_16bit_reg_8bit_reg_indexed_and_byte_indexed_addressing_as_first_ins_methods,
    generate_execute_ins_16bit_reg_and_number, generate_execute_ins_8bit_reg_and_number,
    generate_execute_ins_byte_addr_and_number, generate_execute_ins_word_addr_and_number,
    generate_ins_al_and_num, generate_ins_ax_and_num, memory::Memory,
};

fn adc_16bit_and_set_flags(cpu: &mut CPU, val1: u16, val2: u16) -> Option<u16> {
    let val2 = val2.wrapping_add(cpu.carry_flag as u16);
    let (res, _) = cpu.add_16bit_with_overflow_and_set_flags(val1, val2);
    Some(res)
}

fn adc_8bit_and_set_flags(cpu: &mut CPU, val1: u8, val2: u8) -> Option<u8> {
    let val2 = val2.wrapping_add(cpu.carry_flag as u8);
    let (res, _) = cpu.add_8bit_with_overflow_and_set_flags(val1, val2);
    Some(res)
}

impl CPU {
    generate_16bit_reg_8bit_reg_indexed_and_byte_indexed_addressing_as_first_ins_methods!(
        adc,
        &adc_16bit_and_set_flags,
        &adc_8bit_and_set_flags
    );

    generate_ins_al_and_num!(adc, &adc_8bit_and_set_flags);

    generate_ins_ax_and_num!(adc, &adc_16bit_and_set_flags);

    generate_execute_ins_16bit_reg_and_number!(adc, 0x83, &adc_16bit_and_set_flags);

    generate_execute_ins_8bit_reg_and_number!(adc, &adc_8bit_and_set_flags);

    generate_execute_ins_word_addr_and_number!(adc, 0x83, &adc_16bit_and_set_flags);

    generate_execute_ins_byte_addr_and_number!(adc, &adc_8bit_and_set_flags);
}

#[cfg(test)]
mod adc_tests {
    use crate::cpu::instructions::test_macro::execute_code;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_8bit_mem_and_reg() {
        let code = "
            MOV CH,      10101010b ; 170
            MOV [0x100], 01010101b ; 85
            ADC [0x100], CH
        ";
        let (cpu, mem) = execute_code(code);
        assert_eq!(cpu.read_byte_from_pointer(&mem, 0x100), 0xFF);
    }

    #[test]
    fn test_16bit_mem_and_reg() {
        let code = "
        MOV SP, 0x100
        MOV [0x100], 0x1010
        ADC [0x100], SP
        ";

        let (cpu, mem) = execute_code(code);
        assert_eq!(cpu.read_word_from_pointer(&mem, 0x100), 0x1110);
    }

    #[test]
    fn sbb_8bit_reg_and_16bit_mem_or_reg() {
        let code = "
        MOV CH, 0x10
        MOV b.[0x100], 0x10
        ADC CH, [0x100]

        MOV BL, 0x10
        MOV BH, 0x01 
        ADC BL, BH

        MOV AL, 0x10
        MOV BP, 0x100
        MOV [0x100], 0x10
        ADC AL, [BP]
        ";
        let (cpu, _) = execute_code(code);
        assert_eq!(cpu.get_cx_high(), 0x20);
        assert_eq!(cpu.bx, 0x0111);
        assert_eq!(cpu.get_ax_low(), 0x20);
        assert_eq!(cpu.get_flags_as_binary(), 0b00)
    }

    #[test]
    fn sbb_16bit_reg_and_16bit_mem_or_reg() {
        let code = "
        MOV SP, 0x100
        MOV [0x100], 0x1010
        ADC SP, [0x100]

        MOV BP, 0x10
        MOV [0xA0], 0x1010
        ADC BP, [BP+0x90]

        MOV BX, 0x01 
        MOV AX, 0x10
        ADC BX, AX
        ";
        let (cpu, _) = execute_code(code);
        assert_eq!(cpu.stack_pointer, 0x1110);
        assert_eq!(cpu.base_pointer, 0x1020);
        assert_eq!(cpu.get_flags_as_binary(), 0b0001_0000);
    }

    #[test]
    fn sbb_8bit_reg_and_num() {
        let code = "
        MOV AL, 0x10
        ADC AL, 0x10

        MOV BL, 0x10
        ADC BL, 0x01

        MOV [0x100], 0x10
        ADC b.[0x100], 0x11
        ";
        let (cpu, mem) = execute_code(code);
        assert_eq!(cpu.get_ax_low(), 0x20);
        assert_eq!(cpu.get_bx_low(), 0x11);
        assert_eq!(cpu.read_byte_from_pointer(&mem, 0x100), 0x21);
        assert_eq!(cpu.get_flags_as_binary(), 0b0001_0000);
    }

    #[test]
    fn sbb_16bit_reg_and_num() {
        let code = "
        MOV SP, 0x100
        ADC SP, 0x10

        MOV BP, 0x100
        ADC BP, 0x10

        MOV AX, 0x100
        ADC AX, 0x10
        ";
        let (cpu, _) = execute_code(code);
        assert_eq!(cpu.stack_pointer, 0x110);
        assert_eq!(cpu.base_pointer, 0x110);
        assert_eq!(cpu.ax, 0x0110);
        assert_eq!(cpu.get_flags_as_binary(), 0b0000_0000);
    }
}
