use crate::compiler::{
    compilation_error::CompilationError,
    tokens::Assembly8086Tokens,
    CompiledBytes, tokenized_line::TokenizedLine,
};

use super::utils::{
    get_as_0xc0_0xff_pattern, get_idx_from_reg, if_num_8bit_to_16bit, push_instruction,
};

pub(in crate::compiler) fn parse_add(
    tokenized_line: &TokenizedLine,
    i: usize,
    compiled_bytes: &mut Vec<u8>,
    compiled_bytes_ref: &mut Vec<CompiledBytes>,
) -> Result<usize, CompilationError> {
    let len_lexed_strings = tokenized_line.get_len_lexed_strings();
    let token = tokenized_line.get(i, "This shouldn't happen, Please report this".to_string())?;
    let high_token = tokenized_line.get(
        i + 1,
        "Expected arguments after ADD got nothing".to_string(),
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
                        push_instruction(compiled_bytes, vec![0x05], token, compiled_bytes_ref);
                        push_instruction(
                            compiled_bytes,
                            vec![(number & 0xFF) as u8, (number >> 8) as u8],
                            low_token,
                            compiled_bytes_ref,
                        );
                    } else if (number & 0xFF00) == 0xFF00 {
                        push_instruction(compiled_bytes, vec![0x83], token, compiled_bytes_ref);
                        push_instruction(
                            compiled_bytes,
                            vec![0xC0 | high_reg_idx],
                            high_token,
                            compiled_bytes_ref,
                        );
                        push_instruction(
                            compiled_bytes,
                            vec![(number & 0xFF) as u8],
                            low_token,
                            compiled_bytes_ref,
                        );
                    } else {
                        push_instruction(compiled_bytes, vec![0x81], token, compiled_bytes_ref);
                        push_instruction(
                            compiled_bytes,
                            vec![0xC0 | high_reg_idx],
                            high_token,
                            compiled_bytes_ref,
                        );
                        push_instruction(
                            compiled_bytes,
                            vec![(number & 0xFF) as u8, (number >> 8) as u8],
                            low_token,
                            compiled_bytes_ref,
                        );
                    }

                    Ok(i + 3)
                }
                Assembly8086Tokens::Register16bit(low_reg) => {
                    let low_reg_idx = get_idx_from_reg(low_token, &low_reg)?;
                    let ins = get_as_0xc0_0xff_pattern(high_reg_idx, low_reg_idx);
                    push_instruction(compiled_bytes, vec![0x03], token, compiled_bytes_ref);
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
                        push_instruction(compiled_bytes, vec![0x04], token, compiled_bytes_ref);
                        push_instruction(
                            compiled_bytes,
                            vec![*number],
                            low_token,
                            compiled_bytes_ref,
                        );
                    } else {
                        push_instruction(compiled_bytes, vec![0x82], token, compiled_bytes_ref);
                        push_instruction(
                            compiled_bytes,
                            vec![0xC0 | high_reg.get_as_idx()],
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
                    push_instruction(compiled_bytes, vec![0x02], token, compiled_bytes_ref);
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

    test_compile!(add_ax_sp, "ADD AX, SP", |instructions: &Vec<u8>| {
        assert_eq!(instructions, &[0x03, 0xC4]);
    });

    test_compile!(add_ax_0x1234, "ADD AX, 0x1234", |instructions: &Vec<u8>| {
        assert_eq!(instructions, &[0x05, 0x34, 0x12]);
    });

    test_compile!(add_bx_0xff00, "ADD BX, 0xff12", |instructions: &Vec<u8>| {
        assert_eq!(instructions, &[0x83, 0xC3, 0x12]);
    });

    // test cx + 0x1234
    test_compile!(add_cx_0x1234, "ADD CX, 0x1234", |instructions: &Vec<u8>| {
        assert_eq!(instructions, &[0x81, 0xC1, 0x34, 0x12]);
    });
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
        assert_eq!(instructions, &[0x82, 0xC4, 0x12]);
    });
}
