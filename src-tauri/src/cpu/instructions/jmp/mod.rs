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
    use crate::{compiler::compile_lines, cpu::CPU, generate_test_with_cycles, memory::Memory};

    generate_test_with_cycles!(
        jmp_8bit_positive,
        (|cpu: &mut CPU, mem: &mut Memory| {
            let (compiled_bytes, _, _) = compile_lines(
                "
            label:
                INC AX
            jmp label
            ",
                false,
            )
            .unwrap();
            cpu.write_instructions(mem, &compiled_bytes);
        }),
        (|cpu: &CPU, _: &Memory| {
            assert_eq!(cpu.instruction_pointer, 0x0001);
            assert_eq!(cpu.ax, 0x0003);
        }),
        5
    );

    generate_test_with_cycles!(
        jmp_8bit_negative,
        (|cpu: &mut CPU, mem: &mut Memory| {
            let (compiled_bytes, _, _) = compile_lines(
                "
                INC AX
                JMP label
                DEC AX
                label:
                INC AX  
            ",
                false,
            )
            .unwrap();
            cpu.write_instructions(mem, &compiled_bytes);
        }),
        (|cpu: &CPU, _: &Memory| {
            assert_eq!(cpu.instruction_pointer, 0x0005);
            assert_eq!(cpu.ax, 0x0002);
        }),
        3
    );
}

#[cfg(test)]
mod test_16_bit_jmp {
    use crate::{
        compiler::compile_lines,
        cpu::{instructions::test_macro::compile_and_test_str, CPU},
        generate_test_with_cycles,
        memory::Memory,
    };

    fn generate_0x80_long_ins() -> String {
        let mut ins = String::new();
        for _ in 0..0x80 {
            ins.push_str("INC AX\n");
        }
        ins
    }

    generate_test_with_cycles!(
        jmp_16bit_positive,
        (|cpu: &mut CPU, mem: &mut Memory| {
            let (compiled_bytes, _, _) = compile_lines(
                format!(
                    "
            label:
                {}
            jmp label
            ",
                    generate_0x80_long_ins()
                )
                .as_str(),
                false,
            )
            .unwrap();
            cpu.write_instructions(mem, &compiled_bytes);
        }),
        (|cpu: &CPU, _: &Memory| {
            assert_eq!(cpu.instruction_pointer, 0x0001);
            assert_eq!(cpu.ax, 0x0081);
        }),
        0x82
    );

    generate_test_with_cycles!(
        jmp_16bit_negative,
        (|cpu: &mut CPU, mem: &mut Memory| {
            let (compiled_bytes, _, _) = compile_lines(
                format!(
                    "
                INC AX
                JMP label
                {}
                label:
                INC AX
            ",
                    generate_0x80_long_ins()
                )
                .as_str(),
                false,
            )
            .unwrap();
            cpu.write_instructions(mem, &compiled_bytes);
        }),
        (|cpu: &CPU, _: &Memory| {
            assert_eq!(cpu.instruction_pointer, 0x085);
            assert_eq!(cpu.ax, 0x0002);
        }),
        0x3
    );

    #[test]
    fn test_jmp_var() {
        compile_and_test_str(
            "
            org 0x100
            .data
            var dw 0x0110
            code: 
            jmp var
            ",
            2,
            |cpu: &CPU, _: &Memory| {
                assert_eq!(cpu.instruction_pointer, 0x0102);
            },
        );
    }
}
