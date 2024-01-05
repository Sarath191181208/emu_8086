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
        tokenized_line::{self, TokenizedLine},
        tokens::{
            indexed_addressing_types::IndexedAddressingTypes, registers16bit::Registers16bit,
            registers8bit::Registers8bit, Assembly8086Tokens, Token,
        },
        types_structs::{VariableAddressMap, VariableReferenceMap},
        CompiledLineLabelRef,
    },
    utils::Either,
};

use self::utils::evaluate_ins;
use super::utils::check_comma;

pub(in crate::compiler) mod compile_first_ins_reg_pattern;
pub(in crate::compiler) mod compile_tow_args_whole_ins;
pub(in super::super) mod compile_two_arguments_patterns;
pub(in super::super) mod offset_label_pattern;
pub(in super::super) mod reg_mem_pattern;
pub(in crate::compiler) mod utils;

#[derive(Debug, Clone)]
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

    Register16bitAndIndexedAddressing {
        high_token: Token,
        low_token: Token,
        register_type: Registers16bit,
        addr_type: IndexedAddressingTypes,
    },

    Register8bitAndIndexedAddressing {
        high_token: Token,
        low_token: Token,
        register_type: Registers8bit,
        addr_type: IndexedAddressingTypes,
    },

    AddressAnd16bitNumber {
        high_token: Token,
        low_token: Token,
        address_bytes: [u8; 2],
        num: u16,
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

    IndexedAddressingAndRegister {
        high_token: Token,
        low_token: Token,
        register_type: Registers16bit,
        addr_type: IndexedAddressingTypes,
    },
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn parse_high_low_tokens<'a>(
    tokenized_line: &'a TokenizedLine<'a>,
    i: usize,
    is_org_defined: bool,
    ins: &'a str,
    label_idx_map: &mut HashMap<String, (Token, usize, bool)>,
    variable_ref_map: &mut VariableReferenceMap,
    variable_abs_address_map: &VariableAddressMap,
    compiled_line_offset_maps: Option<&CompiledLineLabelRef>,
) -> Result<(Token, Option<Token>), CompilationError> {
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

    let low_token: Option<Token> = match tokenized_line.get(
        comma_pos.unwrap_or(i + 2) + 1,
        format!(
            "Expected 16bit value after {:?} got nothing",
            high_token.token_type
        )
        .to_string(),
        None,
    ) {
        Ok(token) => Some(token.clone()),
        Err(_) => None,
    };

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

    let mut low_token = match (low_token, compact_low_token) {
        (None, None) => None,
        (None, Some(compact_token)) => Some(compact_token),
        (Some(low_token), None) => Some(low_token),
        (Some(_), Some(compact_token)) => Some(compact_token),
    };

    if let Some(ref low_token_unwrapped) = low_token {
        if let Assembly8086Tokens::Number16bit(num) = low_token_unwrapped.token_type {
            if num < 0xFF {
                low_token = Some(Token::new(
                    Assembly8086Tokens::Number8bit(num as u8),
                    low_token_unwrapped.line_number,
                    low_token_unwrapped.column_number,
                    low_token_unwrapped.token_length,
                ));
            }
        }
    }

    Ok((high_token.clone(), low_token))
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
                Assembly8086Tokens::IndexedAddressing(field) => {
                    Ok(AddressingMode::Register16bitAndIndexedAddressing {
                        high_token: compact_high_token,
                        low_token,
                        register_type: register_type.clone(),
                        addr_type: field.clone(),
                    })
                }

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
                Assembly8086Tokens::Number16bit(num) => {
                    // check if num can be a u8 number
                    if (*num as i16 as u16) <= 0xFF {
                        Ok(AddressingMode::Register8bitNumber {
                            high_token: compact_high_token,
                            low_token,
                            num: *num as u8,
                        })
                    } else {
                        Err(CompilationError::new_without_suggestions(
                            token.line_number,
                            high_token.column_number + high_token.token_length + 1,
                            len_lexed_strings - high_token.column_number - high_token.token_length,
                            &format!(
                                "Expected a 8bit value after {} got {:?} insted",
                                ins, &low_token.token_type
                            ),
                        ))
                    }
                }
                Assembly8086Tokens::Register8bit(_) => Ok(AddressingMode::Registers8bit {
                    high_token: compact_high_token,
                    low_token,
                }),
                Assembly8086Tokens::ByteIndexedAddressing(field)
                | Assembly8086Tokens::IndexedAddressing(field) => {
                    Ok(AddressingMode::Register8bitAndIndexedAddressing {
                        high_token: compact_high_token,
                        low_token,
                        register_type: high_token_type.clone(),
                        addr_type: field.clone(),
                    })
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
            let low_token = match compact_low_token {
                Some(low_token) => low_token,
                None => tokenized_line
                    .get(
                        compact_high_until + 1,
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
                    )?
                    .clone(),
            };
            match &low_token.token_type {
                Assembly8086Tokens::Number16bit(num) => Ok(AddressingMode::AddressAnd16bitNumber {
                    high_token: compact_high_token,
                    low_token: low_token.clone(),
                    address_bytes: offset_val.to_le_bytes(),
                    num: *num,
                }),

                Assembly8086Tokens::Register16bit(reg) => {
                    Ok(AddressingMode::IndexedAddressingAndRegister {
                        high_token: compact_high_token,
                        low_token: low_token.clone(),
                        register_type: reg.clone(),
                        addr_type: IndexedAddressingTypes::Offset(*offset),
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

        Assembly8086Tokens::IndexedAddressing(indexed_addressing_type) => {
            check_comma(tokenized_line, high_token, compact_high_until)?;
            let low_token = match compact_low_token {
                Some(low_token) => low_token,
                None => tokenized_line
                    .get(
                        compact_high_until + 1,
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
                    )?
                    .clone(),
            };
            match &low_token.token_type {
                Assembly8086Tokens::Register16bit(reg) => {
                    Ok(AddressingMode::IndexedAddressingAndRegister {
                        high_token: compact_high_token,
                        low_token: low_token.clone(),
                        register_type: reg.clone(),
                        addr_type: indexed_addressing_type.clone(),
                    })
                }

                _ => Err(CompilationError::error_with_token(
                    &low_token,
                    &format!(
                        "Expected a 16bit value after {} got {:?} insted",
                        ins, &low_token.token_type
                    ),
                )),
            }
        }

        Assembly8086Tokens::ByteIndexedAddressing(IndexedAddressingTypes::Offset(offset)) => {
            let offset_val = offset.as_u16();
            check_comma(tokenized_line, high_token, compact_high_until)?;
            let low_token = tokenized_line.get(
                comma_pos.unwrap_or(i + 2) + 1,
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
