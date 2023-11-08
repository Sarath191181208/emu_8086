use crate::{
    cpu::CPU, generate_16bit_jmp_method, generate_8bit_jmp_method, generate_mem_jmp_method,
    memory::Memory,
};

pub mod jcxz;

fn exec_fn(cpu: &mut CPU, offset: i16) -> Option<u16> {
    let ip = cpu.instruction_pointer;
    // if -ve sub else address
    if offset < 0 {
        Some(ip.wrapping_sub(offset.unsigned_abs()))
    } else {
        Some(ip.wrapping_add(offset.unsigned_abs()))
    }
}

fn exec_fn_abs(_: &mut CPU, offset: u16) -> Option<u16> {
    Some(offset)
}

impl CPU {
    generate_8bit_jmp_method!(jmp, exec_fn);
    generate_16bit_jmp_method!(jmp, exec_fn);
    generate_mem_jmp_method!(jmp, exec_fn_abs);
}
#[cfg(test)]
mod test_8bit_jmp {
    use crate::cpu::instructions::test_macro::run_code;

    #[test]
    fn jmp_8bit_positive() {
        let code = "
            label:
                INC AX
            jmp label
            ";
        let (cpu, _) = run_code(code, 5);
        assert_eq!(cpu.instruction_pointer, 0x0001);
        assert_eq!(cpu.ax, 0x0003);
    }

    #[test]
    fn jmp_8bit_negative() {
        let code = "
        INC AX
        JMP label
        DEC AX
        label:
        INC AX  
    ";
        let (cpu, _) = run_code(code, 3);
        assert_eq!(cpu.instruction_pointer, 0x0005);
        assert_eq!(cpu.ax, 0x0002);
    }
}

#[cfg(test)]
mod test_16_bit_jmp {
    use crate::cpu::instructions::test_macro::run_code;

    fn generate_0x80_long_ins() -> String {
        let mut ins = String::new();
        for _ in 0..0x80 {
            ins.push_str("INC AX\n");
        }
        ins
    }

    #[test]
    fn jmp_16bit_positive() {
        let code = format!(
            "
        label:
            {}
        jmp label
        ",
            generate_0x80_long_ins()
        );
        let (cpu, _) = run_code(&code, 0x82);
        assert_eq!(cpu.instruction_pointer, 0x0001);
        assert_eq!(cpu.ax, 0x0081);
    }

    #[test]
    fn jmp_16bit_negative() {
        let code = format!(
            "
        INC AX
        JMP label
        {}
        label:
        INC AX
        ",
            generate_0x80_long_ins()
        );
        let (cpu, _) = run_code(&code, 0x3);
        assert_eq!(cpu.instruction_pointer, 0x085);
        assert_eq!(cpu.ax, 0x0002);
    }

    #[test]
    fn test_jmp_var() {
        let code = "
        org 0x100
        .data
        var dw 0x0110
        code: 
        jmp var
    ";
        let (cpu, _) = run_code(code, 2);
        assert_eq!(cpu.instruction_pointer, 0x0102);
    }
}
