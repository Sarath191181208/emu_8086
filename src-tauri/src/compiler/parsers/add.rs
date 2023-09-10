use crate::compiler::{
    compilation_error::CompilationError, tokenized_line::TokenizedLine, tokens::Assembly8086Tokens,
    CompiledBytes,
};

use super::{
    pattern_extractors::{parse_line, AddressingMode},
    utils::{
        get_8bit_register, get_as_0xc0_0xff_pattern, 
        get_idx_from_token, push_instruction,
    },
};

pub(in crate::compiler) fn parse_add(
    tokenized_line: &TokenizedLine,
    i: usize,
    compiled_bytes: &mut Vec<u8>,
    compiled_bytes_ref: &mut Vec<CompiledBytes>,
) -> Result<usize, CompilationError> {
    let token = tokenized_line.get(i, "This shouldn't happen, Please report this".to_string())?;
    match parse_line(tokenized_line, i, &"ADD")? {
        AddressingMode::Registers16bit {
            high_token,
            low_token,
        } => {
            let high_reg_idx = get_idx_from_token(high_token)?;
            let low_reg_idx = get_idx_from_token(&low_token)?;
            let ins = get_as_0xc0_0xff_pattern(high_reg_idx, low_reg_idx);
            push_instruction(compiled_bytes, vec![0x03], token, compiled_bytes_ref);
            push_instruction(compiled_bytes, vec![ins], &low_token, compiled_bytes_ref);
            Ok(i + 3)
        }
        AddressingMode::Registers8bit {
            high_token,
            low_token,
        } => {
            let high_reg = get_8bit_register(high_token);
            let low_reg = get_8bit_register(low_token);
            let ins = get_as_0xc0_0xff_pattern(high_reg.get_as_idx(), low_reg.get_as_idx());
            push_instruction(compiled_bytes, vec![0x02], token, compiled_bytes_ref);
            push_instruction(compiled_bytes, vec![ins], low_token, compiled_bytes_ref);
            Ok(i + 3)
        }
        AddressingMode::Registers16bitNumber {
            high_token,
            low_token,
        } => {
            let high_reg_idx = get_idx_from_token(high_token)?;
            let number = match low_token.token_type {
                Assembly8086Tokens::Number16bit(num) => num,
                _ => unreachable!(),
            };
            let is_ax = high_reg_idx == 0;
            if is_ax {
                push_instruction(compiled_bytes, vec![0x05], token, compiled_bytes_ref);
                push_instruction(
                    compiled_bytes,
                    vec![(number & 0xFF) as u8, (number >> 8) as u8],
                    &low_token,
                    compiled_bytes_ref,
                );
            } else {
                let is_num_has_high_bit_full = (number & 0xFF00) == 0xFF00;
                let add_ins = if is_num_has_high_bit_full { 0x83 } else { 0x81 };
                let data_ins = if is_num_has_high_bit_full {
                    vec![(number & 0xFF) as u8]
                } else {
                    vec![(number & 0xFF) as u8, (number >> 8) as u8]
                };

                push_instruction(compiled_bytes, vec![add_ins], token, compiled_bytes_ref);
                push_instruction(
                    compiled_bytes,
                    vec![0xC0 | high_reg_idx],
                    high_token,
                    compiled_bytes_ref,
                );
                push_instruction(compiled_bytes, data_ins, &low_token, compiled_bytes_ref);
            }

            Ok(i + 3)
        }
        AddressingMode::Register8bitNumber {
            high_token,
            low_token,
        } => {
            let high_reg = get_8bit_register(high_token);
            let number = match low_token.token_type {
                Assembly8086Tokens::Number8bit(num) => num,
                _ => unreachable!(),
            };
            let is_al = high_reg.get_as_idx() == 0;
            if is_al{
                push_instruction(compiled_bytes, vec![0x04], token, compiled_bytes_ref);
                push_instruction(compiled_bytes, vec![number], low_token, compiled_bytes_ref);
            } else {
                push_instruction(compiled_bytes, vec![0x82], token, compiled_bytes_ref);
                push_instruction(
                    compiled_bytes,
                    vec![0xC0 | high_reg.get_as_idx()],
                    high_token,
                    compiled_bytes_ref,
                );
                push_instruction(compiled_bytes, vec![number], low_token, compiled_bytes_ref);
            }
            Ok(i + 3)
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
