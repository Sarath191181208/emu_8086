use crate::compiler::{
    compilation_error::CompilationError,
    tokens::{Assembly8086Tokens, Token},
    CompiledBytes,
};

use super::utils::get_as_0xc0_0xff_pattern;

pub(in crate::compiler) fn parse_mov(
    lexed_str_without_spaces: &Vec<&Token>,
    token: &Token,
    i: usize,
    len_lexed_strings: u32,
    compiled_bytes: &mut Vec<u8>,
    compiled_bytes_ref: &mut Vec<CompiledBytes>,
) -> Result<usize, CompilationError> {
    if lexed_str_without_spaces.len() - 1 < i + 1 {
        return Err(CompilationError::new(
            token.line_number,
            token.column_number + token.token_length,
            len_lexed_strings + 1,
            "Insufficient arguments to MOV",
        ));
    }
    let high_token = lexed_str_without_spaces[i + 1];
    match &high_token.token_type {
        Assembly8086Tokens::Register16bit(high_reg) => {
            if i + 3 > lexed_str_without_spaces.len() - 1 {
                return Err(CompilationError::new(
                    high_token.line_number,
                    high_token.column_number + high_token.token_length + 1,
                    len_lexed_strings + 1,
                    "Insufficient arguments to MOV expected a 16bit value ",
                ));
            }
            let low_token = lexed_str_without_spaces[i + 3];
            let high_reg_idx = match high_reg.get_as_idx() {
                Ok(idx) => idx,
                Err(err) => {
                    return Err(CompilationError::new(
                        token.line_number,
                        high_token.column_number,
                        high_token.token_length,
                        err,
                    ));
                }
            };
            match &low_token.token_type {
                Assembly8086Tokens::Number16bit(number) => {
                    compiled_bytes.push(0xB8 | high_reg_idx);
                    compiled_bytes.push((number & 0xFF) as u8);
                    compiled_bytes.push((number >> 8) as u8);

                    compiled_bytes_ref.push(CompiledBytes::new(
                        vec![0xB8],
                        token.line_number,
                        token.column_number,
                    ));

                    compiled_bytes_ref.push(CompiledBytes::new(
                        vec![(number & 0xFF) as u8, (number >> 8) as u8],
                        low_token.line_number,
                        low_token.column_number,
                    ));
                    Ok(i + 3)
                }
                Assembly8086Tokens::Register16bit(low_reg) => {
                    let low_reg_idx = match low_reg.get_as_idx() {
                        Ok(idx) => idx,
                        Err(err) => {
                            return Err(CompilationError::new(
                                token.line_number,
                                token.column_number,
                                token.token_length,
                                err,
                            ));
                        }
                    };
                    compiled_bytes.push(0x8B);
                    let ins = get_as_0xc0_0xff_pattern(high_reg_idx, low_reg_idx);
                    compiled_bytes.push(ins);

                    compiled_bytes_ref.push(CompiledBytes::new(
                        vec![0x8B],
                        token.line_number,
                        token.column_number,
                    ));
                    compiled_bytes_ref.push(CompiledBytes::new(
                        vec![ins],
                        low_token.line_number,
                        low_token.column_number,
                    ));
                    Ok(i + 3)
                }
                _ => Err(CompilationError::new(
                    token.line_number,
                    high_token.column_number + high_token.token_length + 1,
                    len_lexed_strings - high_token.column_number - high_token.token_length,
                    &format!(
                        "Expected a 16bit value after MOV got {:?} insted",
                        &low_token.token_type
                    ),
                )),
            }
        }

        Assembly8086Tokens::Register8bit(high_reg) => {
            if i + 3 > lexed_str_without_spaces.len() - 1 {
                return Err(CompilationError::new(
                    high_token.line_number,
                    high_token.column_number,
                    len_lexed_strings + 1,
                    "Insufficient arguments to MOV expected a 8bit value ",
                ));
            }
            let low_token = lexed_str_without_spaces[i + 3];
            match &low_token.token_type {
                Assembly8086Tokens::Number8bit(number) => {
                    compiled_bytes.push(0xB0 | high_reg.get_as_idx());
                    compiled_bytes.push(*number);

                    compiled_bytes_ref.push(CompiledBytes::new(
                        vec![0xB0],
                        token.line_number,
                        token.column_number,
                    ));
                    compiled_bytes_ref.push(CompiledBytes::new(
                        vec![*number],
                        low_token.line_number,
                        low_token.column_number,
                    ));

                    Ok(i + 3)
                }
                Assembly8086Tokens::Register8bit(low_reg) => {
                    
                    let ins = get_as_0xc0_0xff_pattern(high_reg.get_as_idx(), low_reg.get_as_idx());
                    compiled_bytes.push(0x8A);
                    compiled_bytes.push(ins);

                    compiled_bytes_ref.push(CompiledBytes::new(
                        vec![0x8A],
                        token.line_number,
                        token.column_number,
                    ));

                    compiled_bytes_ref.push(CompiledBytes::new(
                        vec![ins],
                        high_token.line_number,
                        high_token.column_number,
                    ));
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
