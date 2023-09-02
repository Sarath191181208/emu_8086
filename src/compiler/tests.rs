// write tests
// run tests with cargo test


macro_rules! test_compile {
    ($name:ident, $code:expr, $expected_fn:expr) => {
        #[test]
        fn $name() {
            let (inst, _) = match compile_str($code, false) {
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

#[cfg(test)]
mod tests {
    use crate::compiler::compile_str;

    test_compile!(
        test_compile_str_mov_ax_cx,
        "MOV \t AX, CX",
        |compiled_instructions: &Vec<u8>| {
            assert_eq!(compiled_instructions[0], 0x8b);
            assert_eq!(compiled_instructions[1], 0xc1);
        }
    );

    test_compile!(
        test_compile_str_mov_ax_sp,
        "MOV \t AX, SP",
        |compiled_instructions: &Vec<u8>| {
            assert_eq!(compiled_instructions[0], 0x8b);
            assert_eq!(compiled_instructions[1], 0xc4);
        }
    );

    test_compile!(
        test_compile_str_mov_bx_dx,
        "MOV \t BX, DX",
        |compiled_instructions: &Vec<u8>| {
            assert_eq!(compiled_instructions[0], 0x8b);
            assert_eq!(compiled_instructions[1], 0xda);
        }
    );

    test_compile!(
        test_compile_str_mov_sp_bp,
        "MOV \t SP, BP",
        |compiled_instructions: &Vec<u8>| {
            assert_eq!(compiled_instructions[0], 0x8b);
            assert_eq!(compiled_instructions[1], 0xe5);
        }
    );

    // write tests for 16 bit registers but with a instant mov value 
    test_compile!(
        test_compile_str_mov_ax_0x1234,
        "MOV \t AX, 0x1234",
        |compiled_instructions: &Vec<u8>| {
            assert_eq!(compiled_instructions[0], 0xb8);
            assert_eq!(compiled_instructions[1], 0x34);
            assert_eq!(compiled_instructions[2], 0x12);
        }
    );

    test_compile!(
        test_compile_str_mov_bx_0x1234,
        "MOV \t BX, 0x1234",
        |compiled_instructions: &Vec<u8>| {
            assert_eq!(compiled_instructions[0], 0xbb);
            assert_eq!(compiled_instructions[1], 0x34);
            assert_eq!(compiled_instructions[2], 0x12);
        }
    );

    test_compile!(
        test_compile_str_mov_cx_0x1234,
        "MOV \t CX, 0x1234",
        |compiled_instructions: &Vec<u8>| {
            assert_eq!(compiled_instructions[0], 0xb9);
            assert_eq!(compiled_instructions[1], 0x34);
            assert_eq!(compiled_instructions[2], 0x12);
        }
    );

    test_compile!(
        test_compile_str_mov_si_0x1234,
        "MOV \t SI, 0x1234",
        |compiled_instructions: &Vec<u8>| {
            assert_eq!(compiled_instructions[0], 0xbe);
            assert_eq!(compiled_instructions[1], 0x34);
            assert_eq!(compiled_instructions[2], 0x12);
        }
    );

    test_compile!(
        test_compile_str_mov_sp_0x1234,
        "MOV \t SP, 0x1234",
        |compiled_instructions: &Vec<u8>| {
            assert_eq!(compiled_instructions[0], 0xbc);
            assert_eq!(compiled_instructions[1], 0x34);
            assert_eq!(compiled_instructions[2], 0x12);
        }
    );
}

#[cfg(test)]
mod tests8bit {
    use crate::compiler::compile_str;

    test_compile!(
        test_compile_str_mov_al_cl,
        "MOV \t AL, CL",
        |compiled_instructions: &Vec<u8>| {
            assert_eq!(compiled_instructions[0], 0x8A);
            assert_eq!(compiled_instructions[1], 0xc1);
        }
    );

    test_compile!(
        test_compile_str_mov_bl_bh,
        "MOV \t BL, BH",
        |compiled_instructions: &Vec<u8>| {
            assert_eq!(compiled_instructions[0], 0x8a);
            assert_eq!(compiled_instructions[1], 0xdf);
        }
    );

    test_compile!(
        test_compile_str_mov_dl_ah,
        "MOV DL, BH",
        |compiled_instructions: &Vec<u8>| {
            assert_eq!(compiled_instructions[0], 0x8a);
            assert_eq!(compiled_instructions[1], 0xd7);
        }
    );

    test_compile!(
        test_mov_al_0x08,
        "MOV AL, 0x08",
        |compiled_instructions: &Vec<u8>| {
            assert_eq!(compiled_instructions[0], 0xb0);
            assert_eq!(compiled_instructions[1], 0x08);
        }
    );

    test_compile!(
        test_mov_bl_10,
        "MOV BL, 10",
        |compiled_instructions: &Vec<u8>| {
            assert_eq!(compiled_instructions[0], 0xb3);
            assert_eq!(compiled_instructions[1], 10);
        }
    );

    test_compile!(
        test_mov_ch_0x08,
        "MOV CH, 0x08",
        |compiled_instructions: &Vec<u8>| {
            assert_eq!(compiled_instructions[0], 0xb5);
            assert_eq!(compiled_instructions[1], 0x08);
        }
    );

}
