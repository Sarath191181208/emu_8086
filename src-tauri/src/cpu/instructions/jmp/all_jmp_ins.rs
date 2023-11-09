use crate::{cpu::CPU, generate_8bit_jmp_method, memory::Memory};

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

fn exec_zf_1_jmp(cpu: &mut CPU, offset: i16) -> Option<u16> {
    if !cpu.zero_flag {
        return None;
    }
    make_jmp(cpu, offset)
}

fn exec_zf_0_jmp(cpu: &mut CPU, offset: i16) -> Option<u16> {
    if cpu.zero_flag {
        return None;
    }
    make_jmp(cpu, offset)
}

fn exec_zf_0_and_sf_eq_of(cpu: &mut CPU, offset: i16) -> Option<u16> {
    if !cpu.zero_flag && cpu.negative_flag == cpu.overflow_flag {
        dbg!(offset);
        return make_jmp(cpu, offset);
    }
    None
}

fn exec_zf_1_or_sf_neq_of(cpu: &mut CPU, offset: i16) -> Option<u16> {
    if cpu.zero_flag || cpu.negative_flag != cpu.overflow_flag {
        return make_jmp(cpu, offset);
    }
    None
}

fn exec_sf_eq_of_jmp(cpu: &mut CPU, offset: i16) -> Option<u16> {
    if cpu.negative_flag == cpu.overflow_flag {
        return make_jmp(cpu, offset);
    }
    None
}

fn exec_sf_neq_of_jmp(cpu: &mut CPU, offset: i16) -> Option<u16> {
    if cpu.negative_flag != cpu.overflow_flag {
        return make_jmp(cpu, offset);
    }
    None
}

impl CPU {
    generate_8bit_jmp_method!(ja, exec_cf_zf_0_jmp);
    generate_8bit_jmp_method!(jae, exec_cf_0_jmp);
    generate_8bit_jmp_method!(jb, exec_cf_1_jmp);
    generate_8bit_jmp_method!(jbe, exec_cf_1_or_zf_1_jmp);
    generate_8bit_jmp_method!(je, exec_zf_1_jmp);
    generate_8bit_jmp_method!(jne, exec_zf_0_jmp);
    generate_8bit_jmp_method!(jg, exec_zf_0_and_sf_eq_of);
    generate_8bit_jmp_method!(jle, exec_zf_1_or_sf_neq_of);
    generate_8bit_jmp_method!(jge, exec_sf_eq_of_jmp);
    generate_8bit_jmp_method!(jl, exec_sf_neq_of_jmp);
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

    #[test]
    fn test_jbe_8bit() {
        let code = "
            MOV BX, 0x01
            CMP BX, 0x05
            JBE label
            INC AX
            label:
            INC AX
        ";
        let (cpu, _) = execute_code(code);
        assert_eq!(cpu.ax, 0x0001);
    }

    #[test]
    fn test_jbe_16bit() {
        let code = format!(
            "
            MOV BX, 0x01
            CMP BX, 0x05
            JBE label
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
    fn test_je_8bit() {
        let code = "
            MOV BX, 0x01
            CMP BX, 0x05
            JE label
            INC AX
            label:
            INC AX
        ";
        let (cpu, _) = execute_code(code);
        assert_eq!(cpu.ax, 0x0002);
    }

    #[test]
    fn test_je_16bit() {
        let code = format!(
            "
            MOV BX, 0x01
            CMP BX, 0x05
            JE label
            {}
            label:
            INC AX
        ",
            generate_inc_x80()
        );

        let (cpu, _) = execute_code(&code);
        assert_eq!(cpu.ax, 0x0082);
    }

    #[test]
    fn test_jg_execution() {
        let code = "
            MOV BX, 0x05
            CMP BX, -0x05
            JG label
            INC AX
            label:
            INC AX
        ";
        let (cpu, _) = execute_code(code);
        assert_eq!(cpu.ax, 0x0001);
    }

    #[test]
    fn test_jg_16bit() {
        let code = format!(
            "
            MOV BX, 0x05
            CMP BX, -0x05
            JG label
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
    fn test_jge_8bit() {
        let code = "
            MOV BX, 0x05
            CMP BX, -0x05
            JGE label
            INC AX
            label:
            INC AX
        ";
        let (cpu, _) = execute_code(code);
        assert_eq!(cpu.ax, 0x0001);
    }

    #[test]
    fn test_jge_16bit() {
        let code = format!(
            "
            MOV BX, 0x05
            CMP BX, -0x05
            JGE label
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
