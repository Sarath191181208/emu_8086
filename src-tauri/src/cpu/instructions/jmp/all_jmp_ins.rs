use crate::{cpu::CPU, generate_16bit_jmp_label_method, generate_8bit_jmp_method, memory::Memory};

fn get_new_ip(ip: u16, offset: i16) -> u16 {
    if offset < 0 {
        ip.wrapping_sub(offset.unsigned_abs())
    } else {
        ip.wrapping_add(offset.unsigned_abs())
    }
}

fn make_jmp(cpu: &mut CPU, offset: i16) -> Option<u16> {
    let ip = cpu.instruction_pointer;
    let new_ip = get_new_ip(ip, offset);
    Some(new_ip)
}

fn exec_cf_zf_0_jmp(cpu: &mut CPU, offset: i16) -> Option<u16> {
    let is_carry_or_zero_true = cpu.carry_flag || cpu.zero_flag;
    if is_carry_or_zero_true {
        return None;
    }
    make_jmp(cpu, offset)
}

fn exec_cf_0_jmp(cpu: &mut CPU, offset: i16) -> Option<u16> {
    if cpu.carry_flag {
        return None;
    }

    make_jmp(cpu, offset)
}

fn exec_cf_1_jmp(cpu: &mut CPU, offset: i16) -> Option<u16> {
    if !cpu.carry_flag {
        return None;
    }
    make_jmp(cpu, offset)
}

fn exec_cf_1_or_zf_1_jmp(cpu: &mut CPU, offset: i16) -> Option<u16> {
    let is_carry_or_zero_true = cpu.carry_flag || cpu.zero_flag;
    if !is_carry_or_zero_true {
        return None;
    }
    make_jmp(cpu, offset)
}

impl CPU {
    generate_8bit_jmp_method!(ja, exec_cf_zf_0_jmp);
    generate_16bit_jmp_label_method!(ja, exec_cf_zf_0_jmp);
    
    generate_8bit_jmp_method!(jae, exec_cf_0_jmp);
    generate_16bit_jmp_label_method!(jae, exec_cf_0_jmp);

    generate_8bit_jmp_method!(jb, exec_cf_1_jmp);
    generate_16bit_jmp_label_method!(jb, exec_cf_1_jmp);

    generate_8bit_jmp_method!(jbe, exec_cf_1_or_zf_1_jmp);
    generate_16bit_jmp_label_method!(jbe, exec_cf_1_or_zf_1_jmp);
}

#[cfg(test)]
mod tests {
    use crate::cpu::instructions::test_macro::execute_code;

    fn generate_inc_x80() -> String {
        let mut code = String::new();
        for _ in 0..=0x80 {
            code.push_str(&format!("INC AX\n"));
        }
        code
    }

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
        let (cpu, _) = execute_code(code);
        assert_eq!(cpu.ax, 0x0001);
    }

    #[test]
    fn test_ja_16bit() {
        let code = format!(
            "
            MOV BX, 0x01
            CMP BX, 0x00
            JA label
            {}
            label:
            INC AX
        ",
            generate_inc_x80()
        );

        let (cpu, _) = execute_code(&code);
        assert_eq!(cpu.ax, 0x0001);
    }

    #[test]
    fn test_jae_8bit() {
        let code = "
            MOV BX, 0x01
            CMP BX, 0x00
            JAE label
            INC AX
            label:
            INC AX
        ";
        let (cpu, _) = execute_code(code);
        assert_eq!(cpu.ax, 0x0001);
    }

    #[test]
    fn test_jae_16bit() {
        let code = format!(
            "
            MOV BX, 0x01
            CMP BX, 0x00
            JAE label
            {}
            label:
            INC AX
        ",
            generate_inc_x80()
        );

        let (cpu, _) = execute_code(&code);
        assert_eq!(cpu.ax, 0x0001);
    }

    #[test]
    fn test_jb_8bit() {
        let code = "
            MOV BX, 0x01 
            CMP BX, 0x05
            JB label
            INC AX
            label:
            INC AX
        ";
        let (cpu, _) = execute_code(code);
        assert_eq!(cpu.ax, 0x0001);
    }

    #[test]
    fn test_jb_16bit() {
        let code = format!(
            "
            MOV BX, 0x01
            CMP BX, 0x05
            JB label
            {}
            label:
            INC AX
        ",
            generate_inc_x80()
        );

        let (cpu, _) = execute_code(&code);
        assert_eq!(cpu.ax, 0x0001);
    }
}
