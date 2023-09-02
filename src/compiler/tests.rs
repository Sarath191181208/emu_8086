// write tests
// run tests with cargo test
#[cfg(test)]
mod tests {
    use crate::compiler;

    macro_rules! test_compile {
        ($name:ident, $code:expr, $expected_fn:expr) => {
            #[test]
            fn $name() {
                let (inst, _) = match compiler::compile_str($code, false) {
                    Ok(instructions) => instructions,
                    Err(e) => {
                        e.print_compilation_error($code);
                        return;
                    }
                };
                $expected_fn(&inst);
            }
        };
    }

    test_compile!(test_compile_str_mov_ax_cx, "MOV \t AX, CX", |compiled_instructions: &Vec<
        u8,
    >| {
        assert_eq!(compiled_instructions[0], 0x8b);
        assert_eq!(compiled_instructions[1], 0xc1);
    });

    test_compile!(test_compile_str_mov_ax_sp, "MOV \t AX, SP", |compiled_instructions: &Vec<
        u8,
    >| {
        assert_eq!(compiled_instructions[0], 0x8b);
        assert_eq!(compiled_instructions[1], 0xc4);
    });

        test_compile!(test_compile_str_mov_bx_dx, "MOV \t BX, DX", |compiled_instructions: &Vec<
        u8,
    >| {
        assert_eq!(compiled_instructions[0], 0x8b);
        assert_eq!(compiled_instructions[1], 0xda);
    });

    test_compile!(test_compile_str_mov_sp_bp, "MOV \t SP, BP", |compiled_instructions: &Vec<
        u8,
    >| {
        assert_eq!(compiled_instructions[0], 0x8b);
        assert_eq!(compiled_instructions[1], 0xe5);
    });
}
