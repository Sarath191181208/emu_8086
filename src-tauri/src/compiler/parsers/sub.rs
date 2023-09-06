use crate::compiler::{
    compilation_error::CompilationError,
    tokenized_line::TokenizedLine,
    tokens::{Assembly8086Tokens, Token},
    CompiledBytes,
};

use super::utils::{
    get_as_0xc0_0xff_pattern, get_idx_from_reg, if_num_8bit_to_16bit, push_instruction,
};

pub(in crate::compiler) fn parse_sub(
    tokenized_line: &TokenizedLine,
    i: usize,
    compiled_bytes: &mut Vec<u8>,
    compiled_bytes_ref: &mut Vec<CompiledBytes>,
) -> Result<usize, CompilationError> {
    let len_lexed_strings = tokenized_line.get_len_lexed_strings();
    let token = tokenized_line.get(i, "This shouldn't happen, Please report this".to_string())?;
    let high_token = tokenized_line.get(
        i + 1,
        "Expected arguments after SUB got nothing".to_string(),
    )?;
    match &high_token.token_type {
        Assembly8086Tokens::Register16bit(high_reg) => {
            let low_token = tokenized_line.get(
                i + 3,
                format!("Expected 16bit value after {:?} got nothing", high_token).to_string(),
            )?;
            let high_reg_idx = get_idx_from_reg(high_token, &high_reg)?;
            let changed_low_token = if_num_8bit_to_16bit(low_token.token_type.clone());

            match changed_low_token {
                Assembly8086Tokens::Number16bit(number) => {
                    if high_reg_idx == 0 {
                        push_instruction(compiled_bytes, vec![0x2D], token, compiled_bytes_ref);
                        push_instruction(
                            compiled_bytes,
                            vec![(number & 0xFF) as u8, (number >> 8) as u8],
                            low_token,
                            compiled_bytes_ref,
                        );
                    } else {
                        let is_num_has_high_bit_full = (number & 0xFF00) == 0xFF00;
                        let sub_ins: u8 = if is_num_has_high_bit_full { 0x83 } else { 0x81 };
                        let data_ins = if is_num_has_high_bit_full {
                            vec![(number & 0xFF) as u8]
                        } else {
                            vec![(number & 0xFF) as u8, (number >> 8) as u8]
                        };
                        push_instruction(compiled_bytes, vec![sub_ins], token, compiled_bytes_ref);
                        push_instruction(
                            compiled_bytes,
                            vec![0xE8 + high_reg_idx],
                            high_token,
                            compiled_bytes_ref,
                        );
                        push_instruction(compiled_bytes, data_ins, low_token, compiled_bytes_ref);
                    }

                    Ok(i + 3)
                }
                Assembly8086Tokens::Register16bit(low_reg) => {
                    let low_reg_idx = get_idx_from_reg(low_token, &low_reg)?;
                    let ins = get_as_0xc0_0xff_pattern(high_reg_idx, low_reg_idx);
                    push_instruction(compiled_bytes, vec![0x2B], token, compiled_bytes_ref);
                    push_instruction(compiled_bytes, vec![ins], low_token, compiled_bytes_ref);
                    Ok(i + 3)
                }

                _ => Err(CompilationError::new(
                    token.line_number,
                    high_token.column_number + high_token.token_length + 1,
                    len_lexed_strings - high_token.column_number - high_token.token_length,
                    &format!(
                        "Expected a 16bit value after ADD got {:?} insted",
                        &low_token.token_type
                    ),
                )),
            }
        }
        Assembly8086Tokens::Register8bit(high_reg) => {
            let low_token = tokenized_line.get(
                i + 3,
                format!("Expected 8bit value after {:?} got nothing", high_token).to_string(),
            )?;
            match &low_token.token_type {
                Assembly8086Tokens::Number8bit(number) => {
                    if high_reg.get_as_idx() == 0 {
                        push_instruction(compiled_bytes, vec![0x2C], token, compiled_bytes_ref);
                        push_instruction(
                            compiled_bytes,
                            vec![*number],
                            low_token,
                            compiled_bytes_ref,
                        );
                    } else {
                        push_instruction(compiled_bytes, vec![0x80], token, compiled_bytes_ref);
                        push_instruction(
                            compiled_bytes,
                            vec![0xE8 + high_reg.get_as_idx()],
                            high_token,
                            compiled_bytes_ref,
                        );
                        push_instruction(
                            compiled_bytes,
                            vec![*number],
                            low_token,
                            compiled_bytes_ref,
                        );
                    }
                    Ok(i + 3)
                }
                Assembly8086Tokens::Register8bit(low_reg) => {
                    let ins = get_as_0xc0_0xff_pattern(high_reg.get_as_idx(), low_reg.get_as_idx());
                    push_instruction(compiled_bytes, vec![0x2A], token, compiled_bytes_ref);
                    push_instruction(compiled_bytes, vec![ins], low_token, compiled_bytes_ref);
                    Ok(i + 3)
                }

                _ => Err(CompilationError::new(
                    token.line_number,
                    high_token.column_number + high_token.token_length + 1,
                    len_lexed_strings - high_token.column_number - high_token.token_length,
                    &format!(
                        "Expected a 8bit value after ADD got {:?} insted",
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
                "Expected a 16bit or 8bit register after ADD got {:?} insted",
                &high_token.token_type
            ),
        )),
    }
}

#[cfg(test)]
mod tests16bit {
    use crate::{compiler::compile_str, test_compile};

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

    // sub di, 0xff00
    test_compile!(sub_di_0xff00, "SUB DI, 0xff00", |instructions: &Vec<u8>| {
        assert_eq!(instructions, &[0x83, 0xEF, 0x00]);
    });
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
}
