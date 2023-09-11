use std::vec;

use crate::compiler::{
    compilation_error::CompilationError, tokens::Assembly8086Tokens, CompiledBytes, TokenizedLine,
};

use super::{
    pattern_extractors::{parse_line, AddressingMode},
    utils::{get_8bit_register, get_as_0xc0_0xff_pattern, get_idx_from_token, push_instruction},
};

pub(in crate::compiler) fn parse_mov(
    tokenized_line: &TokenizedLine,
    i: usize,
    compiled_bytes: &mut Vec<u8>,
    compiled_bytes_ref: &mut Vec<CompiledBytes>,
) -> Result<usize, CompilationError> {
    let token = tokenized_line.get(i, "This shouldn't happen, Please report this".to_string())?;
    match parse_line(tokenized_line, i, "MOV")? {
        AddressingMode::Registers16bit {
            high_token,
            low_token,
        } => {
            let high_reg_idx = get_idx_from_token(high_token)?;
            let low_reg_idx = get_idx_from_token(&low_token)?;
            let ins = get_as_0xc0_0xff_pattern(high_reg_idx, low_reg_idx);
            push_instruction(compiled_bytes, vec![0x8B], token, compiled_bytes_ref);
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
            push_instruction(compiled_bytes, vec![0x8A], token, compiled_bytes_ref);
            push_instruction(compiled_bytes, vec![ins], high_token, compiled_bytes_ref);
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
                &low_token,
                compiled_bytes_ref,
            );
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
            push_instruction(
                compiled_bytes,
                vec![0xB0 | high_reg.get_as_idx()],
                token,
                compiled_bytes_ref,
            );
            push_instruction(compiled_bytes, vec![number], low_token, compiled_bytes_ref);

            Ok(i + 3)
        }
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
