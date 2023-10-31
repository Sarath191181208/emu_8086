use crate::{cpu::CPU, memory::Memory};

impl CPU {
    pub(in crate::cpu) fn execute_and_16bit_reg(&mut self, mem: &mut Memory) {
        self.consume_bytes_and_parse_16bit_reg_as_first_arg_double_ins(
            mem,
            &|cpu: &mut CPU, val1: u16, val2: u16| -> Option<u16> {
                let res = val1 & val2;
                cpu.set_and_ins_flags_from_16bit_res(res);
                Some(res)
            },
        )
    }

    pub(in crate::cpu) fn execute_and_8bit_reg(&mut self, mem: &mut Memory) {
        let exec_fn = &|cpu: &mut CPU, val1: u8, val2: u8| -> Option<u8> {
            let res = val1 & val2;
            cpu.set_and_ins_flags_from_8bit_res(res);
            Some(res)
        };
        self.consume_bytes_and_parse_8bit_reg_as_first_arg_double_ins(mem, exec_fn);
    }

    pub(in crate::cpu) fn and_al_in_immediate_addressing(&mut self, mem: &mut Memory) {
        let val = self.consume_byte(mem);
        let res = self.get_ax_low() & val;
        self.set_and_ins_flags_from_8bit_res(res);
        self.set_ax_low(res);
    }

    fn set_and_ins_flags_from_16bit_res(&mut self, res: u16) {
        self.carry_flag = false;
        self.overflow_flag = false;
        self.zero_flag = res == 0;
        self.set_pairity_flag_from_16bit_res(res);
        self.set_negative_flag_from_16bit_res(res);
    }

    fn set_and_ins_flags_from_8bit_res(&mut self, res: u8) {
        self.carry_flag = false;
        self.overflow_flag = false;
        self.zero_flag = res == 0;
        self.set_negative_flag_from_8bit_res(res);
        self.set_pairity_flag_from_8bit_res(res);
    }
}

#[cfg(test)]
mod and_ins_exec_tests {
    use crate::cpu::instructions::test_macro::run_code;

    #[test]
    fn and_reg_and_reg_or_mem_tests() {
        let code = "
            org 100h
            .data 
                var dw 0x91 
                var2 db 0x91 
            code: 
                mov ax,  0x0F0F
                mov cx,  0x0F0F
                and ax,  cx 
                and cx,  ax 

                mov dx, 0x0F0F
                mov bx, 0x100 
                and dx, [bx+02]
                
                mov dx, 0x0F0F
                mov bx, 0x02
                and dx, [bx + 0x100]

                mov dx, 0x0F0F
                and dx, [var]

                mov dx, 0x0F0F
                and dx, w.[var2]
        ";
        let (cpu, _) = run_code(code, 16);
        assert_eq!(cpu.ax, 0x0F0F);
        assert_eq!(cpu.cx, 0x0F0F);
        assert_eq!(cpu.dx, 0x0801);
        assert_eq!(cpu.zero_flag, false);
        assert_eq!(cpu.negative_flag, false);
        assert_eq!(cpu.pairity_flag, false);
    }

    #[test]
    fn and_8bit_reg_and_8bitreg_or_mem_tests() {
        let code = "
        org 100h 
        .data 
        var dw 0x91 
        var2 db 0x91
        code: 
            mov al,  0x0F
            mov cl,  0x0F
            and al,  cl 
            and cl,  al 

            mov dl, 0x0F
            mov bx, 0x100 
            and dl, [bx+02]
            
            mov dl, 0x0F
            mov bx, 0x02
            and dl, [bx + 0x100]

            mov dl, 0x0F
            and dl, b.[var]

            mov dl, 0x0F
            and dl, var2
        ";

        let (cpu, _) = run_code(code, 16);
        assert_eq!(cpu.get_ax_low(), 0x0F);
        assert_eq!(cpu.get_bx_low(), 0x02);
        assert_eq!(cpu.get_cx_low(), 0x0F);
        assert_eq!(cpu.get_dx_low(), 0x01);
        assert_eq!(cpu.get_flags_as_binary(), 0x00);
    }

    #[test]
    fn and_al_and_immediate() {
        let code = "
        mov al , 0x0F
        and al, 0x0F
        ";
        let (cpu, _) = run_code(code, 2);
        assert_eq!(cpu.get_ax_low(), 0x0F);
        assert_eq!(cpu.pairity_flag, true);
    }
}
