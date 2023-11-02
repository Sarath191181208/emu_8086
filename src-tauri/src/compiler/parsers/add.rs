use crate::{
    compiler::{
        compilation_error::CompilationError, parsers::utils::get_idx_from_reg,
        tokenized_line::TokenizedLine, CompiledBytesReference,
    },
    convert_and_push_instructions,
    utils::Either,
};

use super::{
    pattern_extractors::{
        compile_tow_args_whole_ins::{CompilingBytesForInstruction, compile_two_args_whole_ins},
        compile_two_arguments_patterns::{
            parse_indexed_addr_and_reg, parse_register_16bit_and_indexed_registers_with_offset,
            parse_register_16bit_and_indexed_registers_without_offset,
            parse_register_8bit_and_indexed_registers_with_offset,
            parse_register_8bit_and_indexed_registers_without_offset,
        },
        AddressingMode,
    },
    utils::{get_8bit_register, get_as_0xc0_0xff_pattern, get_idx_from_token, push_instruction},
};

pub(in crate::compiler) fn parse_add(
    tokenized_line: &TokenizedLine,
    i: usize,
    compiled_bytes: &mut Vec<u8>,
    compiled_bytes_ref: &mut Vec<CompiledBytesReference>,
    addressing_mode: AddressingMode,
) -> Result<usize, CompilationError> {
    let token = tokenized_line.get(
        i,
        "This shouldn't happen, Please report this".to_string(),
        None,
    )?;

    let ins = CompilingBytesForInstruction {
        reg_16bit_and_anything_ins: 0x03,
        reg_8bit_and_anything_ins: 0x02,
        indexed_addressing_and_anyting_ins: 0x01,
        addr_and_8bit_reg: 0x00,

        al_and_num_ins: Some(0x04),
        ax_and_num_ins: Some(0x05),
        reg16bit_and_16bit_num: 0x81,
        reg16bit_and_8bit_num: Some(0x83),
        reg8bit_and_num: 0x80,
        reg_num_sub_ins: 0xC0,

        addr16bit_and_16bit_num: 0x81,
        addr16bit_and_8bit_num: Some(0x83),
        addr8bit_and_num: 0x80,
        addr_num_sub_ins: 0x06,
    };

    compile_two_args_whole_ins(
        tokenized_line,
        i,
        ins,
        compiled_bytes,
        compiled_bytes_ref,
        addressing_mode,
    )
}

#[cfg(test)]
mod tests16bit {
    use crate::{compile_and_compare_ins, compiler::compile_str, test_compile};
    use pretty_assertions::assert_eq;

    test_compile!(
        add_register_and_immediate_value_or_reg,
        "
    ADD AX, SP
    ADD AX, 0x1234
    ADD BX, 0xff12
    ADD CX, 0x12
    ",
        |instructions: &Vec<u8>| {
            assert_eq!(
                instructions,
                &[0x03, 0xC4, 0x05, 0x34, 0x12, 0x81, 0xC3, 0x12, 0xff, 0x83, 0xC1, 0x12]
            );
        }
    );

    // test bx + var
    test_compile!(
        add_bx_var,
        "
    org 100h 
    .data 
    var dw 0x12
    code: 
    ADD BX, var",
        |instructions: &Vec<u8>| {
            assert_eq!(
                instructions,
                &[0xEB, 0x02, 0x12, 0x00, 0x03, 0x1E, 0x02, 0x01]
            );
        }
    );

    // test var + bx
    test_compile!(
        add_var_bx,
        "
    org 100h
    .data
    var dw 0x12
    code:
    ADD var, BX",
        |instructions: &Vec<u8>| {
            assert_eq!(
                instructions,
                &[0xEB, 0x02, 0x12, 0x00, 0x01, 0x1E, 0x02, 0x01]
            );
        }
    );

    // test var + 0x2000
    test_compile!(
        add_var_0x2000,
        "
    org 100h
    .data
    var dw 0x12
    code:
    ADD var, 0x2000",
        |instructions: &Vec<u8>| {
            assert_eq!(
                instructions,
                &[0xEB, 0x02, 0x12, 0x00, 0x81, 0x06, 0x02, 0x01, 0x00, 0x20]
            );
        }
    );

    // test var + 0x10
    test_compile!(
        add_var_0x10,
        "
    org 100h
    .data
    var dw 0x12
    code:
    ADD var, 0x10",
        |instructions: &Vec<u8>| {
            assert_eq!(
                instructions,
                &[0xEB, 0x02, 0x12, 0x00, 0x83, 0x06, 0x02, 0x01, 0x10]
            );
        }
    );

    test_compile!(
        add_dx_di_ref,
        "
    ADD DX, [DI
    ADD DX, Di + 0x20 + 0x30 + BX + 0x10 + BX []+ 0x10
    ADD DX, SI + 0x2000
    Add Cx, 0x10 - 0x20 + 0x30
    ",
        |instructions: &Vec<u8>| {
            assert_eq!(
                instructions,
                &[0x03, 0x15, 0x03, 0x51, 0x70, 0x03, 0x94, 0x00, 0x20, 0x83, 0xC1, 0x20]
            );
        }
    );

    test_compile!(
        add_ax_proc_ref,
        "
        ADD AX, main 
        inc ax

        proc main 
            mov ax, bx 
        endp main
        ",
        |instructions: &Vec<u8>| {
            assert_eq!(instructions, &[0x05, 0x05, 0x00, 0x40, 0x8B, 0xC3]);
        }
    );

    // Add bp, 0100o
    test_compile!(add_bp_0100o, "ADD BP, 0100o", |instructions: &Vec<u8>| {
        assert_eq!(instructions, &[0x83, 0xC5, 0x40]);
    });

    test_compile!(
        add_b_var_0x10,
        "
    org 100h
    .data
    var dw 0x12
    var2 db 0x20
    code:
    ADD b.[var], 0x10
    Add b.[var-0x02] b.[0x02], 0x10
    ADD w.[var], 0x10
    ",
        |instructions: &Vec<u8>| {
            assert_eq!(
                instructions,
                &[
                    0xEB, 0x03, 0x12, 0x00, 0x20, 0x80, 0x06, 0x02, 0x01, 0x10, 0x80, 0x06, 0x02,
                    0x01, 0x10, 0x83, 0x06, 0x02, 0x01, 0x10
                ]
            );
        }
    );

    compile_and_compare_ins!(
        add_indexed_addr_and_reg,
        "
        ADD [bx+si+0x393d], di
        ADD [bp+di+0x60], cx
        ADD [si+0x62], di
        ADD [bx+0x8f07], ax
        ADD [bx+0x28], bx
        ADD bp, bx
        ADD bx+si, ax
        ADD bx+si, dx
        ADD [bp+si+0xe5], dx
        ADD bx+di, sp
        ADD [bx+di+0x1746], cx
        ADD [bx+0xc00], sp
        ADD [bx+si+0x5b], dx
        ADD bp, cx
        ADD [bx+0x5b], ax
        ADD bp+si, bp
        ",
        vec![
            0x01, 0xB8, 0x3D, 0x39, 0x01, 0x4B, 0x60, 0x01, 0x7C, 0x62, 0x01, 0x87, 0x07, 0x8F,
            0x01, 0x5F, 0x28, 0x03, 0xEB, 0x01, 0x00, 0x01, 0x10, 0x01, 0x92, 0xE5, 0x00, 0x01,
            0x21, 0x01, 0x89, 0x46, 0x17, 0x01, 0xA7, 0x00, 0x0C, 0x01, 0x50, 0x5B, 0x03, 0xE9,
            0x01, 0x47, 0x5B, 0x01, 0x2A
        ]
    );
}

#[cfg(test)]
mod test8bit {
    use crate::{compiler::compile_str, test_compile};

    test_compile!(add_al_0x12, "ADD AL, 0x12", |instructions: &Vec<u8>| {
        assert_eq!(instructions, &[0x04, 0x12]);
    });

    // add bl and cl
    test_compile!(add_bl_cl, "ADD BL, CL", |instructions: &Vec<u8>| {
        assert_eq!(instructions, &[0x02, 0xD9]);
    });

    // add ah and bl
    test_compile!(add_ah_bl, "ADD AH, BL", |instructions: &Vec<u8>| {
        assert_eq!(instructions, &[0x02, 0xE3]);
    });

    // add ah and 0x12
    test_compile!(add_ah_0x12, "ADD AH, 0x12", |instructions: &Vec<u8>| {
        assert_eq!(instructions, &[0x80, 0xC4, 0x12]);
    });

    // test bl + var
    test_compile!(
        add_bl_var,
        "
    org 100h
    .data
    var db 0x12
    code:
    add al, var
    ",
        |instructions: &Vec<u8>| {
            assert_eq!(instructions, &[0xEB, 0x01, 0x12, 0x2, 0x06, 0x02, 0x01]);
        }
    );

    // test var + cl
    test_compile!(
        add_var_cl,
        "
    org 100h
    .data
    var db 0x12
    var2 db 0x11
    code:
    add var, cl
    ",
        |instructions: &Vec<u8>| {
            assert_eq!(
                instructions,
                &[0xEB, 0x02, 0x12, 0x11, 0x00, 0x0E, 0x02, 0x01]
            );
        }
    );

    // test var + 0x20
    test_compile!(
        add_var_0x20,
        "
    org 100h
    .data
    var db 0x12
    var2 db 0x11
    code:
    add var, 0x20
    ",
        |instructions: &Vec<u8>| {
            assert_eq!(
                instructions,
                &[0xEB, 0x02, 0x12, 0x11, 0x80, 0x06, 0x02, 0x01, 0x20]
            );
        }
    );

    test_compile!(add_dl_0x100_ref, "ADD DL, [0x100]", |instructions: &Vec<
        u8,
    >| {
        assert_eq!(instructions, &[0x02, 0x16, 0x00, 0x01]);
    });

    test_compile!(add_dx_di_ref, "ADD DL, [DI", |instructions: &Vec<u8>| {
        assert_eq!(instructions, &[0x02, 0x15]);
    });

    test_compile!(
        add_dl_di_bx_ref_0x70,
        "ADD Dl, Di + 0x20 + 0x30 + BX + 0x10 + BX []+ 0x10",
        |instructions: &Vec<u8>| {
            assert_eq!(instructions, &[0x02, 0x51, 0x70]);
        }
    );

    test_compile!(
        add_dx_si_value_ref,
        "ADD Dh, SI + 0x2000",
        |instructions: &Vec<u8>| {
            assert_eq!(instructions, &[0x02, 0xB4, 0x00, 0x20]);
        }
    );

    // test add al, 010o
    test_compile!(add_al_010o, "ADD AL, 010o", |instructions: &Vec<u8>| {
        assert_eq!(instructions, &[0x04, 0x08]);
    });

    test_compile!(
        add_bl_0x10_0x20_0x30,
        "ADD bl, 0x10 - 0x20 + 0x30",
        |instructions: &Vec<u8>| {
            assert_eq!(instructions, &[0x80, 0xC3, 0x20]);
        }
    );
}
