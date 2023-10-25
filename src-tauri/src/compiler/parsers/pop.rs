use std::collections::HashMap;

use crate::{
    compiler::{
        compilation_error::CompilationError,
        parsers::utils::push_instruction,
        tokenized_line::TokenizedLine,
        tokens::{
            indexed_addressing_types::IndexedAddressingTypes, registers16bit::Registers16bit,
            Assembly8086Tokens, Token,
        },
        types_structs::{
            CompiledBytesReference, VariableAddressMap, VariableReferenceMap, VariableType,
        },
        CompiledLineLabelRef,
    },
    convert_and_push_instructions,
    utils::Either,
};

use super::{
    pattern_extractors::utils::{evaluate_ins, get_label_address_or_push_into_ref},
    utils::THIS_SHOULDNT_HAPPEN,
};

#[allow(clippy::too_many_arguments)]
pub(in crate::compiler) fn parse_pop(
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
    let token = tokenized_line.get(i, THIS_SHOULDNT_HAPPEN.to_string(), None)?;
    let high_token = tokenized_line.get(
        i + 1,
        "Expected a Register (or) Segment (or) Address got none".to_string(),
        None,
    )?;

    let evaluated_token = evaluate_ins(
        i + 1,
        tokenized_line.tokens.len(),
        tokenized_line,
        is_org_defined,
        label_idx_map,
        var_ref_map,
        variable_address_map.unwrap_or(&VariableAddressMap::new()),
        compiled_line_ref_with_offset_maps,
    )?;

    let high_token = match &evaluated_token {
        Some(token) => token,
        None => high_token,
    };

    match &high_token.token_type {
        Assembly8086Tokens::Register16bit(Registers16bit::CS) => Err(
            CompilationError::error_with_token(high_token, "Cannot pop into CS"),
        ),
        Assembly8086Tokens::Register16bit(reg) => match reg.is_segment() {
            true => {
                let reg_idx = reg.get_segment_as_idx().unwrap();
                convert_and_push_instructions!(
                    compiled_bytes,
                    compiled_bytes_ref,
                    (
                        high_token => vec![0x07 | (reg_idx << 3)]
                    )
                );
                Ok(i + 1)
            }
            false => {
                let reg_idx = reg.get_as_idx().unwrap();
                convert_and_push_instructions!(
                    compiled_bytes,
                    compiled_bytes_ref,
                    (
                        high_token => vec![0x58 + reg_idx]
                    )
                );
                Ok(i + 1)
            }
        },
        Assembly8086Tokens::IndexedAddressing(IndexedAddressingTypes::Offset(val)) => {
            convert_and_push_instructions!(
                compiled_bytes,
                compiled_bytes_ref,
                (
                    token => vec![0x8F, 0x06],
                    high_token => val.to_le_bytes_vec()
                )
            );
            Ok(i + 1)
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
                                    token => vec![0x8F],
                                    high_token => vec![ 0x40 | reg_idx, num_u8]
                                )
                            );
                        }
                        Either::Right(num_u16) => {
                            convert_and_push_instructions!(
                                compiled_bytes,
                                compiled_bytes_ref,
                                (
                                    token => vec![0x8F],
                                    high_token => vec![ 0x80 | reg_idx, num_u16.to_le_bytes()[0], num_u16.to_le_bytes()[1]]
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
                            token => vec![0x8F],
                            high_token => vec![ reg_idx]
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
                high_token,
                is_org_defined,
                false,
                VariableType::Word,
                variable_address_map.unwrap_or(&VariableAddressMap::new()),
                var_ref_map,
                label_idx_map,
                compiled_line_ref_with_offset_maps,
            );

            let variable_addr_val = match either_pointer_or_num {
                Either::Left(pointer_array) => u16::from_le_bytes(pointer_array),
                _ => {
                    return Err(CompilationError::error_with_token(
                        high_token,
                        &format!(
                            "Expected a variable address got {:?} insted",
                            high_token.token_type
                        ),
                    ))
                }
            };

            convert_and_push_instructions!(
                compiled_bytes,
                compiled_bytes_ref,
                (
                    token => vec![0x8F, 0x06],
                    high_token => variable_addr_val.to_le_bytes().to_vec()
                )
            );

            Ok(tokenized_line.len())
        }

        _ => Err(CompilationError::error_with_token(
            high_token,
            &format!(
                "Expected a Register (or) Segment (or) Address got {:?}",
                high_token.token_type
            ),
        )),
    }
}

#[cfg(test)]
mod push_ins_test {
    use crate::{compile_and_compare_ins, compiler::compile_str, test_compile};

    compile_and_compare_ins!(
        test_pop_16bit_reg_and_segments,
        "
            POP AX
            POP CX
            POP DX 
            POP BX 
            POP SP
            POP BP
            POP SI
            POP DI

            POP DS
            POP SS
            POP ES
            ",
        vec![0x58, 0x59, 0x5A, 0x5B, 0x5C, 0x5D, 0x5E, 0x5F, 0x1F, 0x17, 0x07]
    );

    compile_and_compare_ins!(
        test_segment_regs_with_and_without_offset,
        "
            POP [BX]
            pop [bx + 0x10]
            pop [bx + 0x90 - 0x10 + 0x10 + 0x20]

        ",
        vec![0x8F, 0x07, 0x8F, 0x47, 0x10, 0x8F, 0x87, 0xB0, 0x00]
    );
}
