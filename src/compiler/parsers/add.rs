use crate::compiler::{
    compilation_error::CompilationError,
    tokens::{Assembly8086Tokens, Token},
    CompiledBytes,
};

pub(in crate::compiler) fn parse_add(
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
            (len_lexed_strings + 1) as u32,
            "Insufficient arguments to ADD",
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
                    "Insufficient arguments to ADD expected a 16bit value ",
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
                    if high_reg_idx == 0 {
                        // i.e AX register
                        compiled_bytes.push(0x05);
                        compiled_bytes.push((number & 0xFF) as u8);
                        compiled_bytes.push((number >> 8) as u8);

                        compiled_bytes_ref.push(CompiledBytes::new(
                            vec![0x05],
                            token.line_number,
                            token.column_number,
                        ));

                        compiled_bytes_ref.push(CompiledBytes::new(
                            vec![(number & 0xFF) as u8, (number >> 8) as u8],
                            low_token.line_number,
                            low_token.column_number,
                        ));
                    } else if (number & 0xFF00) == 0xFF00 {
                        compiled_bytes.push(0x83);
                        compiled_bytes.push(0xC0 | high_reg_idx);
                        compiled_bytes.push((number & 0xFF) as u8);

                        compiled_bytes_ref.push(CompiledBytes::new(
                            vec![0x83],
                            token.line_number,
                            token.column_number,
                        ));

                        compiled_bytes_ref.push(CompiledBytes::new(
                            vec![0xC0 | high_reg_idx],
                            high_token.line_number,
                            high_token.column_number,
                        ));

                        compiled_bytes_ref.push(CompiledBytes::new(
                            vec![(number & 0xFF) as u8],
                            low_token.line_number,
                            low_token.column_number,
                        ));
                    } else {
                        compiled_bytes.push(0x81);
                        compiled_bytes.push(0xC0 | high_reg_idx);
                        compiled_bytes.push((number & 0xFF) as u8);
                        compiled_bytes.push((number >> 8) as u8);

                        compiled_bytes_ref.push(CompiledBytes::new(
                            vec![0x81],
                            token.line_number,
                            token.column_number,
                        ));

                        compiled_bytes_ref.push(CompiledBytes::new(
                            vec![0xC0 | high_reg_idx],
                            high_token.line_number,
                            high_token.column_number,
                        ));

                        compiled_bytes_ref.push(CompiledBytes::new(
                            vec![(number & 0xFF) as u8, (number >> 8) as u8],
                            low_token.line_number,
                            low_token.column_number,
                        ));
                    }

                    return Ok(i + 3);
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
                    let ins = (0xC0) | (high_reg_idx / 2) << 4;
                    let ins2 = low_reg_idx | (high_reg_idx % 2) << 3;
                    compiled_bytes.push(0x03);
                    compiled_bytes.push(ins | ins2);

                    compiled_bytes_ref.push(CompiledBytes::new(
                        vec![0x03],
                        token.line_number,
                        token.column_number,
                    ));

                    compiled_bytes_ref.push(CompiledBytes::new(
                        vec![ins | ins2],
                        low_token.line_number,
                        low_token.column_number,
                    ));

                    return Ok(i + 3);
                }

                _ => {
                    return Err(CompilationError::new(
                        token.line_number,
                        high_token.column_number + high_token.token_length + 1,
                        len_lexed_strings - high_token.column_number - high_token.token_length,
                        &format!(
                            "Expected a 16bit value after ADD got {:?} insted",
                            &low_token.token_type
                        ),
                    ));
                }
            }
        }
        Assembly8086Tokens::Register8bit(high_reg) => {
            if i + 3 > lexed_str_without_spaces.len() - 1 {
                return Err(CompilationError::new(
                    high_token.line_number,
                    high_token.column_number,
                    len_lexed_strings + 1,
                    "Insufficient arguments to ADD expected a 8bit value ",
                ));
            }
            let low_token = lexed_str_without_spaces[i + 3];
            match &low_token.token_type {
                Assembly8086Tokens::Number8bit(number) => {
                    if high_reg.get_as_idx() == 0 {
                        // i.e AL register
                        compiled_bytes.push(0x04);
                        compiled_bytes.push(*number);

                        compiled_bytes_ref.push(CompiledBytes::new(
                            vec![0x04],
                            token.line_number,
                            token.column_number,
                        ));

                        compiled_bytes_ref.push(CompiledBytes::new(
                            vec![*number],
                            low_token.line_number,
                            low_token.column_number,
                        ));
                    } else {
                        compiled_bytes.push(0x82);
                        compiled_bytes.push(0xC0 | high_reg.get_as_idx());
                        compiled_bytes.push(*number);

                        compiled_bytes_ref.push(CompiledBytes::new(
                            vec![0x82],
                            token.line_number,
                            token.column_number,
                        ));

                        compiled_bytes_ref.push(CompiledBytes::new(
                            vec![0xC0 | high_reg.get_as_idx()],
                            high_token.line_number,
                            high_token.column_number,
                        ));

                        compiled_bytes_ref.push(CompiledBytes::new(
                            vec![*number],
                            low_token.line_number,
                            low_token.column_number,
                        ));
                    }
                    return Ok(i + 3);
                }
                Assembly8086Tokens::Register8bit(low_reg) => {
                    let ins = (0xC0) | (high_reg.get_as_idx() / 2) << 4;
                    let ins2 = (low_reg.get_as_idx()) | (high_reg.get_as_idx() % 2) << 3;
                    compiled_bytes.push(0x02);
                    compiled_bytes.push(ins | ins2);

                    compiled_bytes_ref.push(CompiledBytes::new(
                        vec![0x02],
                        token.line_number,
                        token.column_number,
                    ));

                    compiled_bytes_ref.push(CompiledBytes::new(
                        vec![ins | ins2],
                        low_token.line_number,
                        low_token.column_number,
                    ));

                    return Ok(i + 3);
                }

                _ => {
                    return Err(CompilationError::new(
                        token.line_number,
                        high_token.column_number + high_token.token_length + 1,
                        len_lexed_strings - high_token.column_number - high_token.token_length,
                        &format!(
                            "Expected a 8bit value after ADD got {:?} insted",
                            &low_token.token_type
                        ),
                    ));
                }
            }
        }
        _ => {
            return Err(CompilationError::new(
                high_token.line_number,
                high_token.column_number,
                high_token.token_length,
                &format!(
                    "Expected a 16bit or 8bit register after ADD got {:?} insted",
                    &high_token.token_type
                ),
            ));
        }
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
