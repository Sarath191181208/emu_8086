use std::vec;

use crate::compiler::{
    compilation_error::CompilationError, tokens::Assembly8086Tokens, CompiledBytes, TokenizedLine,
};

use super::utils::{
    get_as_0xc0_0xff_pattern, get_idx_from_reg, if_num_8bit_to_16bit, push_instruction,
};

pub(in crate::compiler) fn parse_mov(
    tokenized_line: &TokenizedLine,
    i: usize,
    compiled_bytes: &mut Vec<u8>,
    compiled_bytes_ref: &mut Vec<CompiledBytes>,
) -> Result<usize, CompilationError> {
    let len_lexed_strings = tokenized_line.get_len_lexed_strings();
    let token = tokenized_line.get(i, "This shouldn't happen, Please report this".to_string())?;
    let high_token = tokenized_line.get(
        i + 1,
        "Expected arguments after MOV got nothing".to_string(),
    )?;
    match &high_token.token_type {
        Assembly8086Tokens::Register16bit(high_reg) => {
            let low_token = tokenized_line.get(
                i + 3,
                format!("Expected a 16bit value after {:?} got nothing", high_reg),
            )?;
            let high_reg_idx = get_idx_from_reg(high_token, high_reg)?;
            let changed_low_token = if_num_8bit_to_16bit(low_token.token_type.clone());
            match &changed_low_token {
                Assembly8086Tokens::Number16bit(number) => {
                    let ins = (number & 0xFF) as u8;
                    let ins2 = (number >> 8) as u8;
                    push_instruction(
                        compiled_bytes,
                        vec![0xB8 | high_reg_idx],
                        token,
                        compiled_bytes_ref,
                    );
                    push_instruction(
                        compiled_bytes,
                        vec![ins, ins2],
                        low_token,
                        compiled_bytes_ref,
                    );
                    Ok(i + 3)
                }
                Assembly8086Tokens::Register16bit(low_reg) => {
                    let low_reg_idx = get_idx_from_reg(low_token, low_reg)?;
                    let ins = get_as_0xc0_0xff_pattern(high_reg_idx, low_reg_idx);
                    push_instruction(compiled_bytes, vec![0x8B], token, compiled_bytes_ref);
                    push_instruction(compiled_bytes, vec![ins], low_token, compiled_bytes_ref);
                    Ok(i + 3)
                }
                _ => Err(CompilationError::new(
                    token.line_number,
                    high_token.column_number + high_token.token_length + 1,
                    len_lexed_strings - high_token.column_number - high_token.token_length,
                    &format!(
                        "Expected a 16bit value after {:?} got {:?} insted",
                        &high_token.token_type, &low_token.token_type
                    ),
                )),
            }
        }

        Assembly8086Tokens::Register8bit(high_reg) => {
            let low_token = tokenized_line.get(
                i + 3,
                format!("Expected a 8bit value after MOV {:?} got nothing", high_reg),
            )?;
            match &low_token.token_type {
                Assembly8086Tokens::Number8bit(number) => {
                    push_instruction(
                        compiled_bytes,
                        vec![0xB0 | high_reg.get_as_idx()],
                        token,
                        compiled_bytes_ref,
                    );
                    push_instruction(compiled_bytes, vec![*number], low_token, compiled_bytes_ref);

                    Ok(i + 3)
                }
                Assembly8086Tokens::Register8bit(low_reg) => {
                    let ins = get_as_0xc0_0xff_pattern(high_reg.get_as_idx(), low_reg.get_as_idx());
                    push_instruction(compiled_bytes, vec![0x8A], token, compiled_bytes_ref);
                    push_instruction(compiled_bytes, vec![ins], high_token, compiled_bytes_ref);
                    Ok(i + 3)
                }
                _ => Err(CompilationError::new(
                    high_token.line_number,
                    high_token.column_number + high_token.token_length + 1,
                    len_lexed_strings - high_token.column_number - high_token.token_length,
                    &format!(
                        "Expected a 8bit value after MOV got {:?} insted",
                        &low_token.token_type
                    ),
                )),
            }
        }

        _ => Err(CompilationError::new(
            high_token.line_number,
            high_token.column_number,
            high_token.token_length,
            &format!(
                "Expected a 16bit or 8bit register after MOV got {:?} insted",
                &high_token.token_type
            ),
        )),
    }
}

#[cfg(test)]
mod tests {
    use crate::{compiler::compile_str, test_compile};

    test_compile!(
        test_compile_str_mov_ax_cx,
        "MOV \t AX, CX",
        |compiled_instructions: &Vec<u8>| {
            assert_eq!(compiled_instructions, &[0x8b, 0xc1]);
        }
    );

    test_compile!(
        test_compile_str_mov_ax_sp,
        "MOV \t AX, SP",
        |compiled_instructions: &Vec<u8>| {
            assert_eq!(compiled_instructions, &[0x8b, 0xc4]);
        }
    );

    test_compile!(
        test_compile_str_mov_bx_dx,
        "MOV \t BX, DX",
        |compiled_instructions: &Vec<u8>| {
            assert_eq!(compiled_instructions, &[0x8b, 0xda]);
        }
    );

    test_compile!(
        test_compile_str_mov_sp_bp,
        "MOV \t SP, BP",
        |compiled_instructions: &Vec<u8>| {
            assert_eq!(compiled_instructions, &[0x8b, 0xe5]);
        }
    );

    // write tests for 16 bit registers but with a instant mov value
    test_compile!(
        test_compile_str_mov_ax_0x1234,
        "MOV \t AX, 0x1234",
        |compiled_instructions: &Vec<u8>| {
            assert_eq!(compiled_instructions, &[0xb8, 0x34, 0x12]);
        }
    );

    test_compile!(
        test_compile_str_mov_bx_0x1234,
        "MOV \t BX, 0x1234",
        |compiled_instructions: &Vec<u8>| {
            assert_eq!(compiled_instructions, &[0xbb, 0x34, 0x12]);
        }
    );

    test_compile!(
        test_compile_str_mov_cx_0x1234,
        "MOV \t CX, 0x1234",
        |compiled_instructions: &Vec<u8>| {
            assert_eq!(compiled_instructions, &[0xb9, 0x34, 0x12]);
        }
    );

    test_compile!(
        test_compile_str_mov_si_0x1234,
        "MOV \t SI, 0x1234",
        |compiled_instructions: &Vec<u8>| {
            assert_eq!(compiled_instructions, &[0xbe, 0x34, 0x12]);
        }
    );

    test_compile!(
        test_compile_str_mov_sp_0x1234,
        "MOV \t SP, 0x1234",
        |compiled_instructions: &Vec<u8>| {
            assert_eq!(compiled_instructions, &[0xbc, 0x34, 0x12]);
        }
    );
}

#[cfg(test)]
mod tests8bit {
    use crate::{compiler::compile_str, test_compile};

    test_compile!(
        test_compile_str_mov_al_cl,
        "MOV \t AL, CL",
        |compiled_instructions: &Vec<u8>| {
            assert_eq!(compiled_instructions, &[0x8a, 0xc1]);
        }
    );

    test_compile!(
        test_compile_str_mov_bl_bh,
        "MOV \t BL, BH",
        |compiled_instructions: &Vec<u8>| {
            assert_eq!(compiled_instructions, &[0x8a, 0xdf]);
        }
    );

    test_compile!(
        test_compile_str_mov_dl_ah,
        "MOV DL, BH",
        |compiled_instructions: &Vec<u8>| {
            assert_eq!(compiled_instructions, &[0x8a, 0xd7]);
        }
    );

    test_compile!(
        test_mov_al_0x08,
        "MOV AL, 0x08",
        |compiled_instructions: &Vec<u8>| {
            assert_eq!(compiled_instructions, &[0xb0, 0x08]);
        }
    );

    test_compile!(
        test_mov_bl_10,
        "MOV BL, 10",
        |compiled_instructions: &Vec<u8>| {
            assert_eq!(compiled_instructions, &[0xb3, 10]);
        }
    );

    test_compile!(
        test_mov_ch_0x08,
        "MOV CH, 0x08",
        |compiled_instructions: &Vec<u8>| {
            assert_eq!(compiled_instructions, &[0xb5, 0x08]);
        }
    );
}
