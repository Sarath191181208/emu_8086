use crate::{consts::Byte, cpu::CPU};

// Register Addressing
impl CPU {
    pub(crate) fn sub_8bit_register_addressing(&mut self, instruction: Byte) {
        let (source_index, write_index) = self.get_index_from_c0_ff_pattern(instruction);
        let reg = self.get_8bit_register_by_index(source_index % 8);
        let write_reg = self.get_8bit_register_by_index(write_index);
        let (result, _) = self.sub_8bit_with_overflow_and_set_flags(write_reg, reg);
        self.set_8bit_register_by_index(write_index, result);
    }

    pub(super) fn sub_16bit_register_addressing(&mut self, instruction: Byte) {
        let (source_index, write_index) = self.get_index_from_c0_ff_pattern(instruction);
        let reg = self.get_16bit_register_by_index(source_index % 8);
        let write_reg = self.get_16bit_register_by_index(write_index);
        let (result, _) = self.sub_16bit_with_overflow_and_set_flags(write_reg, reg);
        self.set_16bit_register_by_index(write_index, result);
    }
}

#[cfg(test)]
mod sub_16bit_register_addressing {
    use crate::{
        cpu::{instructions::test_macro::run_code, CPU},
        generate_test,
        memory::Memory,
    };

    // sub ax, cx
    #[test]
    fn test_sub_ax_cx() {
        let code = "
            MOV AX, 0x0F0F
            MOV CX, 0x0013
            SUB AX, CX
            ";
        let (cpu, _) = run_code(code, 3);
        assert_eq!(cpu.ax, 0x0EFC);
        assert_eq!(cpu.get_flags_as_binary(), 0b00010000)
    }

    #[test]
    fn test_sub_bx_dx_overflow(){
        let code = "
        MOV BX, 0x00
        mov DX, 0x13 
        SUB BX, DX 
        ";
        let (cpu, _) = run_code(code, 3);
        assert_eq!(cpu.bx, 0xFFED);
        assert_eq!(cpu.get_flags_as_binary(), 0b000110101)
    }

    #[test]
    fn test_sub_sp_bp(){
        let code = "
        MOV SP, 0xF000
        mov BP, 0x0013
        SUB SP, BP
        ";
        let (cpu, _) = run_code(code, 3);
        assert_eq!(cpu.stack_pointer, 0xEFED);
        assert_eq!(cpu.get_flags_as_binary(), 0b00110100)
    }
}

#[cfg(test)]
mod sub_8bit_register_addressing {
    use crate::{cpu::{CPU, instructions::test_macro::run_code}, generate_test, memory::Memory};


    #[test]
    fn test_sub_al_cl(){
        let code = "
        MOV AL, 0x0F
        MOV CL, 0x13
        SUB AL, CL
        ";
        let (cpu, _) = run_code(code, 3);
        assert_eq!(cpu.ax, 0xFC);
        assert_eq!(cpu.get_flags_as_binary(), 0b00010101)
    }

    #[test]
    fn test_sub_bl_dl_overflow(){
        let code = "
        MOV BL, 0xF0
        MOV DL, 0x13
        SUB BL, DL
        ";
        let (cpu, _) = run_code(code, 3);
        assert_eq!(cpu.get_bx_low(), 0xDD);
        assert_eq!(cpu.get_flags_as_binary(), 0b00110100)
    }
}
