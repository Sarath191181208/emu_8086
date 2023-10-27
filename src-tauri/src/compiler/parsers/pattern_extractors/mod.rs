use std::collections::HashMap;

use crate::{
    compiler::{
        compilation_error::CompilationError,
        suggestions_utils::{
            get_16bit_number_suggestion, get_8bit_number_suggestion,
            get_all_16bit_registers_suggestions, get_all_16bit_variables_suggestions,
            get_all_8bit_registers_suggestions, get_all_8bit_variables_suggestions,
            get_all_registers_and_variable_suggestions,
        },
        tokenized_line::TokenizedLine,
        tokens::{
            indexed_addressing_types::IndexedAddressingTypes, registers16bit::Registers16bit,
            registers8bit::Registers8bit, Assembly8086Tokens, SignedU16, Token,
        },
        types_structs::{VariableAddressMap, VariableReferenceMap, VariableType},
        CompiledLineLabelRef,
    },
    utils::Either,
};

use self::utils::{evaluate_ins, get_label_address_or_push_into_ref};
use super::utils::check_comma;

pub(in super::super) mod compile_two_arguments_patterns;
pub(in super::super) mod offset_label_pattern;
pub(in crate::compiler) mod utils;

pub(crate) enum AddressingMode {
    Registers16bit {
        high_token: Token,
        low_token: Token,
    },
    Registers8bit {
        high_token: Token,
        low_token: Token,
    },
    Registers16bitNumber {
        high_token: Token,
        low_token: Token,
        num: Either<u8, u16>,
    },
    Register8bitNumber {
        high_token: Token,
        low_token: Token,
        num: u8,
    },

    Register16bitAndAddress {
        high_token: Token,
        low_token: Token,
        address_bytes: [u8; 2],
        register_type: Registers16bit,
    },

    Register16bitAndIndexedAddress {
        high_token: Token,
        low_token: Token,
    },

    Register8bitAndIndexedAddress {
        high_token: Token,
        low_token: Token,
        register_type: Registers8bit,
    },

    Register16bitAndIndexedAddressWithOffset {
        high_token: Token,
        low_token: Token,
        offset: SignedU16,
    },

    Register8bitAndIndexedAddressWithOffset {
        high_token: Token,
        low_token: Token,
        register_type: Registers8bit,
        offset: SignedU16,
    },

    AddressAnd16bitRegister {
        high_token: Token,
        low_token: Token,
        address_bytes: [u8; 2],
        register_type: Registers16bit,
    },
    AddressAnd16bitNumber {
        high_token: Token,
        low_token: Token,
        address_bytes: [u8; 2],
        num: u16,
    },

    Register8bitAndAddress {
        high_token: Token,
        low_token: Token,
        address_bytes: [u8; 2],
        register_type: Registers8bit,
    },
    AddressAnd8bitRegister {
        high_token: Token,
        low_token: Token,
        address_bytes: [u8; 2],
        register_type: Registers8bit,
    },
    AddressAnd8bitNumber {
        high_token: Token,
        low_token: Token,
        address_bytes: [u8; 2],
        num: u8,
    },
    ByteAddressAnd8bitNumber {
        high_token: Token,
        low_token: Token,
        address_bytes: [u8; 2],
        num: u8,
    },
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn parse_two_arguments_line<'a>(
    tokenized_line: &'a TokenizedLine<'a>,
    i: usize,
    is_org_defined: bool,
    ins: &'a str,
    label_idx_map: &mut HashMap<String, (Token, usize, bool)>,
    variable_ref_map: &mut VariableReferenceMap,
    variable_abs_address_map: &VariableAddressMap,
    compiled_line_offset_maps: Option<&CompiledLineLabelRef>,
) -> Result<AddressingMode, CompilationError> {
    let len_lexed_strings = tokenized_line.get_len_lexed_strings();
    let token = tokenized_line.get(
        i,
        "This shouldn't happen, Please report this".to_string(),
        None,
    )?;

    let high_token = tokenized_line.get(
        i + 1,
        format!("Expected arguments after {} got nothing", ins).to_string(),
        Some(vec![get_all_registers_and_variable_suggestions(Some(
            variable_abs_address_map,
        ))]),
    )?;

    let comma_pos = tokenized_line.find_comma();
    let compact_high_until = match comma_pos {
        Some(pos) => pos,
        None => tokenized_line.len(),
    };

    let compact_high_token = evaluate_ins(
        i + 1,
        compact_high_until,
        tokenized_line,
        is_org_defined,
        label_idx_map,
        variable_ref_map,
        variable_abs_address_map,
        compiled_line_offset_maps,
    )?
    .unwrap_or(high_token.clone());

    let high_token = &compact_high_token;
    let compact_low_token = evaluate_ins(
        compact_high_until + 1,
        tokenized_line.len(),
        tokenized_line,
        is_org_defined,
        label_idx_map,
        variable_ref_map,
        variable_abs_address_map,
        compiled_line_offset_maps,
    )?;

    match &high_token.token_type.clone() {
        Assembly8086Tokens::Register16bit(register_type) => {
            check_comma(tokenized_line, high_token, i + 2)?;
            let low_token = tokenized_line.get(
                i + 3,
                format!(
                    "Expected 16bit value after {:?} got nothing",
                    high_token.token_type
                )
                .to_string(),
                Some(vec![
                    get_all_16bit_registers_suggestions(),
                    get_all_16bit_variables_suggestions(Some(variable_abs_address_map)),
                    get_8bit_number_suggestion(),
                    get_16bit_number_suggestion(),
                ]),
            )?;
            let low_token = match compact_low_token {
                Some(low_token) => low_token,
                None => low_token.clone(),
            };
            match &low_token.token_type.clone() {
                Assembly8086Tokens::Number16bit(num) => Ok(AddressingMode::Registers16bitNumber {
                    high_token: compact_high_token,
                    low_token,
                    num: Either::Right(*num),
                }),
                Assembly8086Tokens::Number8bit(num) => Ok(AddressingMode::Registers16bitNumber {
                    high_token: compact_high_token,
                    low_token,
                    num: Either::Left(*num),
                }),
                Assembly8086Tokens::Register16bit(_) => Ok(AddressingMode::Registers16bit {
                    high_token: compact_high_token,
                    low_token,
                }),
                Assembly8086Tokens::IndexedAddressing(field) => match field.get_offset() {
                    Some(offset) => match field {
                        IndexedAddressingTypes::Offset(offset) => {
                            let offset_val = offset.as_u16();
                            Ok(AddressingMode::Register16bitAndAddress {
                                high_token: compact_high_token,
                                low_token,
                                address_bytes: offset_val.to_le_bytes(),
                                register_type: register_type.clone(),
                            })
                        }
                        _ => Ok(AddressingMode::Register16bitAndIndexedAddressWithOffset {
                            high_token: compact_high_token,
                            low_token,
                            offset,
                        }),
                    },
                    None => Ok(AddressingMode::Register16bitAndIndexedAddress {
                        high_token: compact_high_token,
                        low_token,
                    }),
                },

                _ => Err(CompilationError::new_without_suggestions(
                    token.line_number,
                    high_token.column_number + high_token.token_length + 1,
                    len_lexed_strings - high_token.column_number - high_token.token_length,
                    &format!(
                        "Expected a 16bit value after {} got {:?} insted",
                        ins, &low_token.token_type
                    ),
                )),
            }
        }

        Assembly8086Tokens::Register8bit(high_token_type) => {
            check_comma(tokenized_line, high_token, i + 2)?;
            let low_token = match compact_low_token {
                None => tokenized_line
                    .get(
                        i + 3,
                        format!("Expected 8bit value after {:?} got nothing", high_token)
                            .to_string(),
                        Some(vec![
                            get_all_8bit_registers_suggestions(),
                            get_all_8bit_variables_suggestions(Some(variable_abs_address_map)),
                            get_8bit_number_suggestion(),
                        ]),
                    )?
                    .clone(),
                Some(low_token) => low_token,
            };
            match &low_token.token_type.clone() {
                Assembly8086Tokens::Number8bit(num) => Ok(AddressingMode::Register8bitNumber {
                    high_token: compact_high_token,
                    low_token,
                    num: *num,
                }),
                Assembly8086Tokens::Register8bit(_) => Ok(AddressingMode::Registers8bit {
                    high_token: compact_high_token,
                    low_token,
                }),
                Assembly8086Tokens::ByteIndexedAddressing(field)
                | Assembly8086Tokens::IndexedAddressing(field) => match field.get_offset() {
                    Some(offset) => match field {
                        // if only the offset exists it's a different case
                        IndexedAddressingTypes::Offset(offset) => {
                            let offset_val = offset.as_u16();
                            Ok(AddressingMode::Register8bitAndAddress {
                                high_token: compact_high_token,
                                low_token,
                                address_bytes: offset_val.to_le_bytes(),
                                register_type: high_token_type.clone(),
                            })
                        }
                        // for the idexed addressing case i.e [bx+0x100]
                        _ => Ok(AddressingMode::Register8bitAndIndexedAddressWithOffset {
                            high_token: compact_high_token,
                            low_token,
                            offset,
                            register_type: high_token_type.clone(),
                        }),
                    },
                    // if the offsset doesn't exists case i.e only [bx]
                    None => Ok(AddressingMode::Register8bitAndIndexedAddress {
                        high_token: compact_high_token,
                        low_token,
                        register_type: high_token_type.clone(),
                    }),
                },

                Assembly8086Tokens::Character(label) => {
                    let address_bytes = get_label_address_or_push_into_ref(
                        i + 3,
                        label,
                        &low_token,
                        is_org_defined,
                        false,
                        VariableType::Byte,
                        variable_abs_address_map,
                        variable_ref_map,
                        label_idx_map,
                        compiled_line_offset_maps,
                    );

                    match address_bytes {
                        Either::Left(address_bytes) => Ok(AddressingMode::Register8bitAndAddress {
                            high_token: compact_high_token,
                            low_token,
                            address_bytes,
                            register_type: high_token_type.clone(),
                        }),
                        Either::Right(num) => match num {
                            Either::Left(num) => Ok(AddressingMode::Register8bitNumber {
                                high_token: compact_high_token,
                                low_token,
                                num,
                            }),
                            Either::Right(_) => Err(CompilationError::error_with_token(
                                token,
                                &format!(
                                    "Expected a 8bit value after {} got 16bit value insted",
                                    ins
                                ),
                            )),
                        },
                    }
                }

                _ => Err(CompilationError::new_without_suggestions(
                    token.line_number,
                    high_token.column_number + high_token.token_length + 1,
                    len_lexed_strings - high_token.column_number - high_token.token_length,
                    &format!(
                        "Expected a 8bit value after {} got {:?} insted",
                        ins, &low_token.token_type
                    ),
                )),
            }
        }

        Assembly8086Tokens::IndexedAddressing(IndexedAddressingTypes::Offset(offset)) => {
            let offset_val = offset.as_u16();
            check_comma(tokenized_line, high_token, compact_high_until)?;
            let low_token = tokenized_line.get(
                i + 3,
                format!(
                    "Expected 16bit value after {:?} got nothing",
                    high_token.token_type
                )
                .to_string(),
                Some(vec![
                    get_all_16bit_registers_suggestions(),
                    get_all_16bit_variables_suggestions(Some(variable_abs_address_map)),
                    get_8bit_number_suggestion(),
                    get_16bit_number_suggestion(),
                ]),
            )?;
            let low_token = match compact_low_token {
                Some(low_token) => low_token,
                None => low_token.clone(),
            };
            match &low_token.token_type {
                Assembly8086Tokens::Number16bit(num) => Ok(AddressingMode::AddressAnd16bitNumber {
                    high_token: compact_high_token,
                    low_token: low_token.clone(),
                    address_bytes: offset_val.to_le_bytes(),
                    num: *num,
                }),

                Assembly8086Tokens::Register16bit(reg) => {
                    Ok(AddressingMode::AddressAnd16bitRegister {
                        high_token: compact_high_token,
                        low_token: low_token.clone(),
                        address_bytes: offset_val.to_le_bytes(),
                        register_type: reg.clone(),
                    })
                }

                Assembly8086Tokens::Number8bit(num) => Ok(AddressingMode::AddressAnd8bitNumber {
                    high_token: compact_high_token,
                    low_token: low_token.clone(),
                    address_bytes: offset_val.to_le_bytes(),
                    num: *num,
                }),

                Assembly8086Tokens::Register8bit(reg) => {
                    Ok(AddressingMode::AddressAnd8bitRegister {
                        high_token: compact_high_token,
                        low_token: low_token.clone(),
                        address_bytes: offset_val.to_le_bytes(),
                        register_type: reg.clone(),
                    })
                }

                _ => Err(CompilationError::error_with_token(
                    &low_token,
                    &format!(
                        "Expected a 16bit/8bit value after {} got {:?} insted",
                        ins, &low_token.token_type
                    ),
                )),
            }
        }

        Assembly8086Tokens::ByteIndexedAddressing(IndexedAddressingTypes::Offset(offset)) => {
            let offset_val = offset.as_u16();
            check_comma(tokenized_line, high_token, compact_high_until)?;
            let low_token = tokenized_line.get(
                i + 3,
                format!(
                    "Expected 16bit value after {:?} got nothing",
                    high_token.token_type
                )
                .to_string(),
                Some(vec![
                    get_all_8bit_registers_suggestions(),
                    get_all_8bit_variables_suggestions(Some(variable_abs_address_map)),
                ]),
            )?;
            let low_token = match compact_low_token {
                Some(low_token) => low_token,
                None => low_token.clone(),
            };
            match &low_token.token_type {
                Assembly8086Tokens::Number8bit(num) => {
                    Ok(AddressingMode::ByteAddressAnd8bitNumber {
                        high_token: compact_high_token,
                        low_token: low_token.clone(),
                        address_bytes: offset_val.to_le_bytes(),
                        num: *num,
                    })
                }

                Assembly8086Tokens::Register8bit(reg) => {
                    Ok(AddressingMode::AddressAnd8bitRegister {
                        high_token: compact_high_token,
                        low_token: low_token.clone(),
                        address_bytes: offset_val.to_le_bytes(),
                        register_type: reg.clone(),
                    })
                }

                _ => Err(CompilationError::error_with_token(
                    &low_token,
                    &format!(
                        "Expected a 8bit value after {} got {:?} insted",
                        ins, &low_token.token_type
                    ),
                )),
            }
        }

        _ => Err(CompilationError::new_without_suggestions(
            high_token.line_number,
            high_token.column_number,
            high_token.token_length,
            &format!(
                "Expected a 16bit or 8bit register after {} got {:?} insted",
                ins, &high_token.token_type
            ),
        )),
    }
}
