use crate::{
    compiler::{
        compilation_error::CompilationError, parsers::utils::get_idx_from_reg,
        tokenized_line::TokenizedLine, types_structs::CompiledBytesReference,
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

pub(in crate::compiler) fn parse_sub(
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
        reg_16bit_and_anything_ins: 0x2B,
        reg_8bit_and_anything_ins: 0x2A,
        indexed_addressing_and_anyting_ins: 0x29,
        addr_and_8bit_reg: 0x28,

        al_and_num_ins: Some(0x2C),
        ax_and_num_ins: Some(0x2D),
        reg16bit_and_16bit_num: 0x81,
        reg16bit_and_8bit_num: Some(0x83),
        reg8bit_and_num: 0x80,
        reg_num_sub_ins: 0xE8,

        addr16bit_and_16bit_num: 0x81,
        addr16bit_and_8bit_num: Some(0x83),
        addr8bit_and_num: 0x80,
        addr_num_sub_ins: 0x2E,
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

    test_compile!(add_ax_sp, "SUB AX, SP", |instructions: &Vec<u8>| {
        assert_eq!(instructions, &[0x2b, 0xC4]);
    });

    test_compile!(add_sp_ax, "SUB SP, AX", |instructions: &Vec<u8>| {
        assert_eq!(instructions, &[0x2b, 0xE0]);
    });

    // sub ax, 0x1234
    test_compile!(sub_ax_0x1234, "SUB AX, 0x1234", |instructions: &Vec<u8>| {
        assert_eq!(instructions, &[0x2D, 0x34, 0x12]);
    });

    // sub dx, 0x0f0f
    test_compile!(sub_dx_0x0f0f, "SUB DX, 0x0f0f", |instructions: &Vec<u8>| {
        assert_eq!(instructions, &[0x81, 0xEA, 0x0F, 0x0F]);
    });

    // sub di, 0xff
    test_compile!(sub_di_0xff, "SUB DI, 0xff", |instructions: &Vec<u8>| {
        assert_eq!(instructions, &[0x81, 0xEF, 0xff, 0x00]);
    });

    // sub cx, var
    test_compile!(
        sub_cx_var,
        "
    org 100h 
    .data 
    var dw 0x1234
    code: 
    SUB CX, var",
        |instructions: &Vec<u8>| {
            assert_eq!(
                instructions,
                &[0xEB, 0x02, 0x34, 0x12, 0x2B, 0x0E, 0x02, 0x01]
            );
        }
    );

    // sub var, sp
    test_compile!(
        sub_var_sp,
        "
    org 100h 
    .data 
    var dw 0x1234
    code: 
    SUB var, SP",
        |instructions: &Vec<u8>| {
            assert_eq!(
                instructions,
                &[0xEB, 0x02, 0x34, 0x12, 0x29, 0x26, 0x02, 0x01]
            );
        }
    );

    // sub var, 0x12
    test_compile!(
        sub_var_0x12,
        "
    org 100h
    .data
    var dw 0x1234
    code:
    SUB var, 0x12",
        |instructions: &Vec<u8>| {
            assert_eq!(
                instructions,
                &[0xEB, 0x02, 0x34, 0x12, 0x83, 0x2E, 0x02, 0x01, 0x12]
            );
        }
    );

    // sub var, 0x1234
    test_compile!(
        sub_var_0x1234,
        "
    org 100h
    .data 
    var dw 0x1234 
    code: 
    SUB var, 0x1234",
        |instructions: &Vec<u8>| {
            assert_eq!(
                instructions,
                &[0xEB, 0x02, 0x34, 0x12, 0x81, 0x2E, 0x02, 0x01, 0x34, 0x12]
            );
        }
    );

    test_compile!(sub_dx_bp_di, "SUB DX, [BP] + DI]]", |instructions: &Vec<
        u8,
    >| {
        assert_eq!(instructions, &[0x2B, 0x13]);
    });

    test_compile!(
        sub_dx_bp_di_offset,
        "SUB DX, [BP] + [0x10] + DI + 0x20 + 0x30",
        |instructions: &Vec<u8>| {
            assert_eq!(instructions, &[0x2B, 0x53, 0x60]);
        }
    );

    // sub bx, 0x10 - 0x20 + 0x30
    test_compile!(
        sub_bx_0x10_0x20_0x30,
        "SUB BX, 0x10 - 0x20 + 0x30",
        |instructions: &Vec<u8>| {
            assert_eq!(instructions, &[0x83, 0xEB, 0x20]);
        }
    );

    compile_and_compare_ins!(
        sub_mem_reg,
        "
        SUB bx+di, bp
        SUB [bx+0x53d], bx
        SUB [di+0xb396], bx
        ",
        vec![0x29, 0x29, 0x29, 0x9F, 0x3D, 0x05, 0x29, 0x9D, 0x96, 0xB3]
    );
}

#[cfg(test)]
mod tests8bit {
    use crate::{compiler::compile_str, test_compile};

    test_compile!(sub_al_0x12, "SUB AL, 0x12", |instructions: &Vec<u8>| {
        assert_eq!(instructions, &[0x2C, 0x12]);
    });

    test_compile!(sub_al_cl, "SUB AL, CL", |instructions: &Vec<u8>| {
        assert_eq!(instructions, &[0x2A, 0xC1]);
    });

    test_compile!(sub_dl_bh, "SUB DL, BH", |instructions: &Vec<u8>| {
        assert_eq!(instructions, &[0x2A, 0xD7]);
    });

    test_compile!(sub_bh_dl, "SUB BH, DL", |instructions: &Vec<u8>| {
        assert_eq!(instructions, &[0x2A, 0xFA]);
    });

    test_compile!(sub_bh_0x12, "SUB BH, 0x12", |instructions: &Vec<u8>| {
        assert_eq!(instructions, &[0x80, 0xEF, 0x12]);
    });

    test_compile!(sub_al_0x0a, "SUB AL, 0x0a", |instructions: &Vec<u8>| {
        assert_eq!(instructions, &[0x2C, 0x0a]);
    });

    // sub bl, var
    test_compile!(
        sub_bl_var,
        "
    org 100h
    .data 
    var db 0x12
    code:
    SUB bL, var",
        |instructions: &Vec<u8>| {
            assert_eq!(instructions, &[0xEB, 0x01, 0x12, 0x2A, 0x1E, 0x02, 0x01]);
        }
    );

    // sub var, bl
    test_compile!(
        sub_var_bl,
        "
    org 100h
    .data
    var db 0x12
    code:
    SUB var, bL",
        |instructions: &Vec<u8>| {
            assert_eq!(instructions, &[0xEB, 0x01, 0x12, 0x28, 0x1E, 0x02, 0x01]);
        }
    );

    // sub var, 0x12
    test_compile!(
        sub_var_0x12,
        "
    org 100h
    .data
    var db 0x12
    code:
    SUB var, 0x12",
        |instructions: &Vec<u8>| {
            assert_eq!(
                instructions,
                &[0xEB, 0x01, 0x12, 0x80, 0x2E, 0x02, 0x01, 0x12]
            );
        }
    );

    test_compile!(
        sub_bl_0x1000_offset,
        "SUB DL, [0x1000]",
        |instructions: &Vec<u8>| {
            assert_eq!(instructions, &[0x2A, 0x16, 0x00, 0x10]);
        }
    );

    test_compile!(sub_dl_bp_di, "SUB DL, [BP] + DI]]", |instructions: &Vec<
        u8,
    >| {
        assert_eq!(instructions, &[0x2A, 0x13]);
    });

    test_compile!(
        sub_bl_bp_di_offset,
        "SUB BL, [BP] + [0x10] + DI + 0x20 + 0x30",
        |instructions: &Vec<u8>| {
            assert_eq!(instructions, &[0x2A, 0x5B, 0x60]);
        }
    );

    test_compile!(
        sub_cl_0x10_0x20_0x30,
        "SUB CL, 0x10 - 0x20 + 0x30",
        |instructions: &Vec<u8>| {
            assert_eq!(instructions, &[0x80, 0xE9, 0x20]);
        }
    );
}
