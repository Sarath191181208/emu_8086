use std::collections::HashMap;

use crate::{
    compiler::{
        compilation_error::CompilationError,
        parsers::utils::push_instruction,
        tokenized_line::TokenizedLine,
        tokens::{indexed_addressing_types::IndexedAddressingTypes, Assembly8086Tokens, Token},
        types_structs::{VariableAddressMap, VariableReferenceMap, VariableType},
        CompiledBytesReference, CompiledLineLabelRef,
    },
    convert_and_push_instructions,
    utils::Either,
};

use super::pattern_extractors::{
    offset_label_pattern::{
        match_ins_to_bytes_single_ins_with_label_and_offset_label,
        parse_token_high_token_and_is_offset_defined, LabeledInstructionCompileData, Offset,
    },
    utils::{evaluate_ins, get_label_address_or_push_into_ref},
};

#[allow(clippy::too_many_arguments)]
pub(in crate::compiler) fn parse_push(
    tokenized_line: &TokenizedLine,
    i: usize,
    is_org_defined: bool,
    compiled_bytes: &mut Vec<u8>,
    compiled_bytes_ref: &mut Vec<CompiledBytesReference>,
    var_ref_map: &mut VariableReferenceMap,
    variable_address_map: Option<&VariableAddressMap>,
    label_idx_map: &mut HashMap<String, (Token, usize, bool)>,
    compiled_line_ref_with_offset_maps: Option<&CompiledLineLabelRef>,
) -> Result<usize, CompilationError> {
    let prev_i = i;
    let (i, token, high_token, is_offset) = parse_token_high_token_and_is_offset_defined(
        tokenized_line,
        i,
        variable_address_map,
        "PUSH",
    )?;

    let new_token = evaluate_ins(
        prev_i + 1,
        tokenized_line.tokens.len(),
        tokenized_line,
        is_org_defined,
        label_idx_map,
        var_ref_map,
        variable_address_map.unwrap_or(&VariableAddressMap::new()),
        compiled_line_ref_with_offset_maps,
    )?;

    let high_token = match &new_token {
        Some(token) => token,
        None => high_token,
    };

    let instruction_compile_data = LabeledInstructionCompileData {
        ins_8bit: vec![0x6A],
        ins_16bit: vec![0x68],
        bytes_of_8bit_ins: 1,
        bytes_of_16bit_ins: 2,
        is_offset,
        pointer_offset_instruction: vec![0xFF, 0x36],
    };

    match &high_token.token_type {
        Assembly8086Tokens::Register16bit(reg) => {
            match reg.is_segment() {
                true => {
                    let reg = reg.get_segment_as_idx().unwrap();
                    convert_and_push_instructions!(
                        compiled_bytes,
                        compiled_bytes_ref,
                        (
                            high_token => vec![0x06 | (reg << 3)]
                        )
                    );
                }

                false => {
                    let reg = reg.get_as_idx().unwrap();
                    convert_and_push_instructions!(
                        compiled_bytes,
                        compiled_bytes_ref,
                        (
                            high_token => vec![0x50 + reg]
                        )
                    );
                }
            }
            Ok(i + 1)
        }

        Assembly8086Tokens::IndexedAddressing(IndexedAddressingTypes::Offset(val)) => {
            convert_and_push_instructions!(
                compiled_bytes,
                compiled_bytes_ref,
                (
                    token => instruction_compile_data.pointer_offset_instruction,
                    high_token => val.to_le_bytes_vec()
                )
            );
            Ok(tokenized_line.len())
        }

        Assembly8086Tokens::IndexedAddressing(idx_addr) => {
            let offset = idx_addr.get_offset_and_default_bp_to_0();
            let reg_idx = match idx_addr.get_as_idx() {
                Ok(val) => val,
                Err(err) => return Err(CompilationError::error_with_token(high_token, err)),
            };

            match offset {
                Some(num) => {
                    let num = match num.as_num() {
                        Ok(val) => val,
                        Err(err) => {
                            return Err(CompilationError::error_with_token(high_token, err))
                        }
                    };

                    match num {
                        Either::Left(num_u8) => {
                            convert_and_push_instructions!(
                                compiled_bytes,
                                compiled_bytes_ref,
                                (
                                    token => vec![0xFF],
                                    high_token => vec![ 0x70 | reg_idx, num_u8]
                                )
                            );
                        }
                        Either::Right(num_u16) => {
                            convert_and_push_instructions!(
                                compiled_bytes,
                                compiled_bytes_ref,
                                (
                                    token => vec![0xFF],
                                    high_token => vec![ 0xB0 | reg_idx, num_u16.to_le_bytes()[0], num_u16.to_le_bytes()[1]]
                                )
                            );
                        }
                    }
                }
                None => {
                    convert_and_push_instructions!(
                        compiled_bytes,
                        compiled_bytes_ref,
                        (
                            token => vec![0xFF],
                            high_token => vec![ 0x30 | reg_idx]
                        )
                    );
                }
            }

            Ok(tokenized_line.len())
        }

        Assembly8086Tokens::Character(label) => {
            let either_pointer_or_num = get_label_address_or_push_into_ref(
                1,
                label,
                is_org_defined,
                VariableType::Word,
                variable_address_map.unwrap_or(&VariableAddressMap::new()),
                var_ref_map,
                compiled_line_ref_with_offset_maps,
            );

            let offset_case = match either_pointer_or_num {
                Either::Left(pointer_array) => Offset::Pointer(u16::from_le_bytes(pointer_array)),
                Either::Right(u8_or_u16) => match u8_or_u16 {
                    Either::Left(u8_val) => Offset::U8(u8_val),
                    Either::Right(u16_val) => Offset::U16(u16_val),
                },
            };

            match_ins_to_bytes_single_ins_with_label_and_offset_label(
                i,
                token,
                high_token,
                instruction_compile_data,
                offset_case,
                compiled_bytes,
                compiled_bytes_ref,
            );

            Ok(tokenized_line.len())
        }

        Assembly8086Tokens::Number16bit(val) => {
            convert_and_push_instructions!(
                compiled_bytes,
                compiled_bytes_ref,
                (
                    token => instruction_compile_data.ins_16bit,
                    high_token => val.to_le_bytes().to_vec()
                )
            );
            Ok(tokenized_line.len())
        }

        Assembly8086Tokens::Number8bit(val) => {
            convert_and_push_instructions!(
                compiled_bytes,
                compiled_bytes_ref,
                (
                    token => instruction_compile_data.ins_8bit,
                    high_token => vec![*val]
                )
            );
            Ok(tokenized_line.len())
        }

        _ => Err(CompilationError::error_with_token(
            token,
            &format!(
                "Can't compile {:?} as the first argument to PUSH , Expected a label/offset",
                high_token.token_type
            ),
        )),
    }
}

#[cfg(test)]
mod push_ins_test {
    use crate::{compile_and_compare_ins, compiler::compile_str, test_compile};

    test_compile!(
        test_push_16bit_register,
        "PUSH BP",
        |compiled_instructions: &Vec<u8>| {
            assert_eq!(compiled_instructions, &[0x55]);
        }
    );

    test_compile!(
        test_push_label,
        "
        MOV BX, CX
        label: 
        inc ax
        mov ax, bx 
        push label
        ",
        |compiled_instructions: &Vec<u8>| {
            assert_eq!(
                compiled_instructions,
                &[0x8B, 0xD9, 0x40, 0x8b, 0xC3, 0x6A, 0x02]
            );
        }
    );

    test_compile!(
        test_push_variable,
        "
        org 100h 
        .data 
        var1 dw 0x0101
        code:
        push var1
        ",
        |compiled_instructions: &Vec<u8>| {
            assert_eq!(
                compiled_instructions,
                &[0xEB, 0x02, 0x01, 0x01, 0xFF, 0x36, 0x02, 0x01]
            );
        }
    );

    compile_and_compare_ins!(
        push_variable_and_offset,
        "
        org 100h 
        .data 
        var1 dw 0x0101
        code:
        push var1 + 0x100
        ",
        &[0xEB, 0x02, 0x01, 0x01, 0xFF, 0x36, 0x02, 0x02]
    );

    compile_and_compare_ins!(
        push_offset_var,
        "
        org 100h 
        .data 
        var1 dw 0x101
        code: 
        push offset var1
        ",
        &[0xEB, 0x02, 0x01, 0x01, 0x68, 0x02, 0x01]
    );

        compile_and_compare_ins!(
        push_offset_8bit_variable,
        "
        org 100h 
        .data 
        var1 db 0x10
        code:
        push offset var1 
        ",
        &[0xEB, 0x02, 0x01, 0x01, 0xFF, 0x36, 0x02, 0x01]
    );

    compile_and_compare_ins!(
        push_offset_label,
        "
        org 100h 
        .data 
        var1 dw 0x0101
        code:
        push lab + 0x100
        lab:
        ",
        &[0xEB, 0x02, 0x01, 0x01, 0x68, 0x07, 0x02]
    );
}
