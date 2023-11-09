use crate::{cpu::CPU, generate_16bit_jmp_label_method, generate_8bit_jmp_method, memory::Memory};

fn get_new_ip(ip: u16, offset: i16) -> u16 {
    if offset < 0 {
        ip.wrapping_sub(offset.unsigned_abs())
    } else {
        ip.wrapping_add(offset.unsigned_abs())
    }
}

fn exec_cf_zf_0_jmp(cpu: &mut CPU, offset: i16) -> Option<u16> {
    let is_carry_or_zero_true = cpu.carry_flag || cpu.zero_flag;
    if is_carry_or_zero_true {
        return None;
    }

    let ip = cpu.instruction_pointer;
    let new_ip = get_new_ip(ip, offset);
    Some(new_ip)
}

impl CPU {
    generate_8bit_jmp_method!(ja, exec_cf_zf_0_jmp);
    generate_16bit_jmp_label_method!(jnbe, exec_cf_zf_0_jmp);
}

#[cfg(test)]
mod tests {
    use crate::cpu::instructions::test_macro::run_code;

    #[test]
    fn test_ja_8bit() {
        let code = "
            MOV BX, 0x01
            CMP BX, 0x00
            JA label
            INC AX
            label:
            INC AX
        ";
        let (cpu, _) = run_code(code, 5);
        assert_eq!(cpu.instruction_pointer, 12);
        assert_eq!(cpu.ax, 0x0001);
    }
}
