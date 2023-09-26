use std::{collections::HashMap, vec};

use crate::{
    compiler::{
        compilation_error::CompilationError,
        tokens::{
            registers16bit::Registers16bit, registers8bit::Registers8bit, Assembly8086Tokens,
        },
        types_structs::{VariableAddressMap, VariableReferenceMap},
        CompiledBytesReference, TokenizedLine,
    },
    convert_and_push_instructions,
};

use super::{
    pattern_extractors::{parse_line, AddressingMode},
    utils::{
        get_8bit_register, get_as_0xc0_0xff_pattern, get_idx_from_token, invalid_path_error,
        push_instruction,
    },
};

pub(in crate::compiler) fn parse_mov(
    tokenized_line: &TokenizedLine,
    i: usize,
    compiled_bytes: &mut Vec<u8>,
    compiled_bytes_ref: &mut Vec<CompiledBytesReference>,
    variable_ref_map: &mut VariableReferenceMap,
    variable_abs_offset_map: Option<&VariableAddressMap>,
) -> Result<usize, CompilationError> {
    let token = tokenized_line.get(i, "This shouldn't happen, Please report this".to_string())?;
    match parse_line(
        tokenized_line,
        i,
        "MOV",
        variable_ref_map,
        variable_abs_offset_map.unwrap_or(&HashMap::new()),
    )? {
        AddressingMode::Registers16bit {
            high_token,
            low_token,
        } => {
            let high_reg_idx = get_idx_from_token(high_token)?;
            let low_reg_idx = get_idx_from_token(&low_token)?;
            let ins = get_as_0xc0_0xff_pattern(high_reg_idx, low_reg_idx);
            convert_and_push_instructions!(
                compiled_bytes,
                compiled_bytes_ref,
                (
                    token => vec![0x8B],
                    &low_token => vec![ins]
                )
            );
            Ok(i + 3)
        }
        AddressingMode::Registers8bit {
            high_token,
            low_token,
        } => {
            let high_reg = get_8bit_register(high_token);
            let low_reg = get_8bit_register(low_token);
            let ins = get_as_0xc0_0xff_pattern(high_reg.get_as_idx(), low_reg.get_as_idx());
            // push_instruction(compiled_bytes, vec![0x8A], token, compiled_bytes_ref);
            // push_instruction(compiled_bytes, vec![ins], high_token, compiled_bytes_ref);
            convert_and_push_instructions!(
                compiled_bytes,
                compiled_bytes_ref,
                (
                    token => vec![0x8A],
                    high_token => vec![ins]
                )
            );
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
            convert_and_push_instructions!(
                compiled_bytes,
                compiled_bytes_ref,
                (
                    token => vec![0xB8 | high_reg_idx],
                    &low_token => vec![ins, ins2]
                )
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
            // push_instruction(
            //     compiled_bytes,
            //     vec![0xB0 | high_reg.get_as_idx()],
            //     token,
            //     compiled_bytes_ref,
            // );
            // push_instruction(compiled_bytes, vec![number], low_token, compiled_bytes_ref);
            convert_and_push_instructions!(
                compiled_bytes,
                compiled_bytes_ref,
                (
                    token => vec![0xB0 | high_reg.get_as_idx()],
                    low_token => vec![number]
                )
            );

            Ok(i + 3)
        }
        AddressingMode::Register16bitAndVariable {
            high_token,
            low_token,
            address_bytes,
        } => match &high_token.token_type {
            Assembly8086Tokens::Register16bit(var) => match var {
                Registers16bit::AX => {
                    convert_and_push_instructions!(
                        compiled_bytes,
                        compiled_bytes_ref,
                        (
                            high_token => vec![0xA1],
                            &low_token => address_bytes.to_vec()
                        )
                    );
                    Ok(i + 3)
                }
                _ => {
                    let high_reg_idx = get_idx_from_token(high_token)?;
                    convert_and_push_instructions!(
                        compiled_bytes,
                        compiled_bytes_ref,
                        (
                            token => vec![0x8B],
                            high_token => vec![0x0E | (high_reg_idx-1) << 3],
                            &low_token => address_bytes.to_vec()
                        )
                    );
                    Ok(i + 3)
                }
            },
            _ => invalid_path_error(token),
        },
        AddressingMode::VariableAnd16bitRegister {
            high_token,
            low_token,
            address_bytes,
        } => match &high_token.token_type {
            Assembly8086Tokens::Character(_) => match &low_token.token_type {
                Assembly8086Tokens::Register16bit(reg) => match &reg {
                    Registers16bit::AX => {
                        convert_and_push_instructions!(
                            compiled_bytes,
                            compiled_bytes_ref,
                            (
                                high_token => vec![0xA3],
                                &low_token => address_bytes.to_vec()
                            )
                        );
                        Ok(i + 3)
                    }
                    _ => {
                        let reg_idx = get_idx_from_token(&low_token)?;
                        convert_and_push_instructions!(
                            compiled_bytes,
                            compiled_bytes_ref,
                            (
                                token => vec![0x89],
                                &high_token => vec![0x0E
                                 | (reg_idx-1) << 3],
                                &low_token => address_bytes.to_vec()
                            )
                        );
                        Ok(i + 3)
                    }
                },
                _ => invalid_path_error(token),
            },
            _ => invalid_path_error(token),
        },
        AddressingMode::VariableAnd16bitNumber {
            high_token,
            low_token,
            address_bytes,
        } => match &high_token.token_type {
            Assembly8086Tokens::Character(_) => match &low_token.token_type {
                Assembly8086Tokens::Number16bit(num) => {
                    convert_and_push_instructions!(
                        compiled_bytes,
                        compiled_bytes_ref,
                        (
                            token => vec![0xC7, 0x06],
                            &high_token => address_bytes.to_vec(),
                            &low_token => vec![(num & 0xFF) as u8, (num >> 8) as u8]
                        )
                    );
                    Ok(i + 3)
                }
                _ => invalid_path_error(token),
            },
            _ => invalid_path_error(token),
        },
        AddressingMode::Register8bitAndVariable {
            high_token,
            low_token,
            address_bytes,
        } => match &high_token.token_type {
            Assembly8086Tokens::Register8bit(Registers8bit::AL) => {
                convert_and_push_instructions!(
                    compiled_bytes,
                    compiled_bytes_ref,
                    (
                        token => vec![0xA0],
                        &low_token => address_bytes.to_vec()
                    )
                );
                Ok(i + 3)
            }
            Assembly8086Tokens::Register8bit(_) => {
                let reg_idx = get_8bit_register(&high_token).get_as_idx();
                convert_and_push_instructions!(
                    compiled_bytes,
                    compiled_bytes_ref,
                    (
                        token => vec![0x8A],
                        &high_token => vec![0x06 | reg_idx << 3],
                        &low_token => address_bytes.to_vec()
                    )
                );
                Ok(i + 3)
            }
            _ => invalid_path_error(token),
        },
        AddressingMode::VariableAnd8bitRegister {
            high_token,
            low_token,
            address_bytes,
        } => match &low_token.token_type {
            Assembly8086Tokens::Register8bit(Registers8bit::AL) => {
                convert_and_push_instructions!(
                    compiled_bytes,
                    compiled_bytes_ref,
                    (
                        token => vec![0xA2],
                        &high_token => address_bytes.to_vec()
                    )
                );
                Ok(i + 3)
            }
            Assembly8086Tokens::Register8bit(_) => {
                let reg_idx = get_8bit_register(&low_token).get_as_idx();
                convert_and_push_instructions!(
                    compiled_bytes,
                    compiled_bytes_ref,
                    (
                        token => vec![0x88],
                        &high_token => vec![0x06 | reg_idx << 3],
                        &low_token => address_bytes.to_vec()
                    )
                );
                Ok(i + 3)
            }
            _ => invalid_path_error(token),
        },
        AddressingMode::VariableAnd8bitNumber {
            high_token,
            low_token,
            address_bytes,
        } => match &low_token.token_type {
            Assembly8086Tokens::Number8bit(num) => {
                convert_and_push_instructions!(
                    compiled_bytes,
                    compiled_bytes_ref,
                    (
                        token => vec![0xC6, 0x06],
                        &high_token => address_bytes.to_vec(),
                        &low_token => vec![*num]
                    )
                );
                Ok(i + 3)
            }
            _ => invalid_path_error(token),
        },
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

    test_compile!(
        test_mov_dh_var,
        "
        org 100h 
        .data 
        var db 10h
        code: 
        MOV DH, var
        ",
        |compiled_instructions: &Vec<u8>| {
            assert_eq!(
                compiled_instructions,
                &[0xEB, 0x01, 0x10, 0x89, 0x36, 0x02, 0x01]
            );
        }
    );
}
