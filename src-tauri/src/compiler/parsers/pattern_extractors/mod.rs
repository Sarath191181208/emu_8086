use serde_json::value::Index;
use serde_with::As;

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
            registers8bit::Registers8bit, Assembly8086Tokens, Token,
        },
        types_structs::{VariableAddressMap, VariableReferenceMap, VariableType},
    },
    utils::Either,
};

use super::utils::{
    check_comma, check_token, get_label_address_or_push_into_ref, iterate_with_seperator,
};

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
}

fn get_compact_ins<'a>(
    start_index: usize,
    end_index: usize,
    tokenized_line: &'a TokenizedLine<'a>,
) -> Result<Option<Token>, CompilationError> {
    if start_index >= tokenized_line.len()
        || start_index == end_index
        || end_index > tokenized_line.len()
    {
        return Ok(None);
    }
    // if the start index and end index have difference 1 then it means that the instruction is compact
    if (end_index - start_index) == 1 {
        return Ok(None);
    }
    // check if there is an openSquareBracket in tokenized_line[start_index..end_index] use iter()
    let open_sqaure_bracket_exists = tokenized_line
        .slice(start_index, end_index)
        .iter()
        .any(|token| token.token_type == Assembly8086Tokens::OpenSquareBracket);

    // possible patterns are
    // bx + si ; bx + di ; bp + si ; bp + di ; si ; di ; bp ; bx  all with offsets

    let mut is_bx_in_line = (false, 0 as u32);
    let mut is_bp_in_line = (false, 0 as u32);
    let mut is_si_in_line = (false, 0 as u32);
    let mut is_di_in_line = (false, 0 as u32);
    let mut offset: Option<u16> = None;

    let mut i = start_index;
    let mut while_limit = 1000;
    while i < end_index && while_limit > 0 {
        while_limit += 1;
        let token = tokenized_line.get(
            i,
            "This shouldn't happen, Report this! Err: iterate_with_seperator:174".to_string(),
            None,
        )?;
        match token.token_type {
            Assembly8086Tokens::Register16bit(Registers16bit::BX) => {
                is_bx_in_line = (true, token.column_number);
            }
            Assembly8086Tokens::Register16bit(Registers16bit::BP) => {
                is_bp_in_line = (true, token.column_number);
            }
            Assembly8086Tokens::Register16bit(Registers16bit::SI) => {
                is_si_in_line = (true, token.column_number);
            }
            Assembly8086Tokens::Register16bit(Registers16bit::DI) => {
                is_di_in_line = (true, token.column_number);
            }

            Assembly8086Tokens::Number16bit(num) => {
                // add to offset if overflow return error
                let (val, overflow) = offset.unwrap_or(0).overflowing_add(num);
                offset = Some(val);
                if overflow {
                    return Err(CompilationError::new_without_suggestions(
                        token.line_number,
                        token.column_number,
                        token.token_length,
                        "Offset overflowed",
                    ));
                }
            }

            Assembly8086Tokens::Number8bit(num) => {
                // add to offset if overflow return error
                let (val, overflow) = offset.unwrap_or(0).overflowing_add(num as u16);
                offset = Some(val);
                if overflow {
                    return Err(CompilationError::new_without_suggestions(
                        token.line_number,
                        token.column_number,
                        token.token_length,
                        "Offset overflowed",
                    ));
                }
            }

            Assembly8086Tokens::Register16bit(_) => {
                return Err(CompilationError::new_without_suggestions(
                    token.line_number,
                    token.column_number,
                    token.token_length,
                    &format!(
                        "Expected a 16bit register of types {{ BX, BP, SI, DI }} got {:?} insted",
                        token.token_type
                    ),
                ));
            }

            Assembly8086Tokens::OpenSquareBracket | Assembly8086Tokens::CloseSquareBracket => {
                i += 1;
                continue;
            }

            _ => {
                return Err(CompilationError::new_without_suggestions(
                    token.line_number,
                    token.column_number,
                    token.token_length,
                    &format!(
                        "Expected a 16bit register got {:?} insted",
                        token.token_type
                    ),
                ));
            }
        }
        i += 1;
        if i < end_index {
            let temp_token = tokenized_line.get(
                i,
                "This shouldn't happen, Report this! Err: iterate_with_seperator:174".to_string(),
                None,
            )?;
            match temp_token.token_type {
                Assembly8086Tokens::OpenSquareBracket | Assembly8086Tokens::CloseSquareBracket => {}
                _ => check_token(tokenized_line, token, i, &Assembly8086Tokens::Plus)?,
            }
            i += 1;
        }
    }

    if while_limit == 0 {
        panic!("While limit reached");
    }
    if is_bx_in_line.0 && is_bp_in_line.0 {
        let token = tokenized_line.get(
            is_bx_in_line.1 as usize,
            "This shouldn't happen, Please report this".to_string(),
            None,
        )?;
        return Err(CompilationError::new_without_suggestions(
            token.line_number,
            token.column_number,
            token.token_length,
            "Expected either bx or bp got both",
        ));
    }

    if is_si_in_line.0 && is_di_in_line.0 {
        let token = tokenized_line.get(
            is_si_in_line.1 as usize,
            "This shouldn't happen, Please report this".to_string(),
            None,
        )?;
        return Err(CompilationError::new_without_suggestions(
            token.line_number,
            token.column_number,
            token.token_length,
            "Expected either si or di got both",
        ));
    }

    let first_ins = tokenized_line.get(
        start_index,
        "This shouldn't happen, Please report this".to_string(),
        None,
    )?;

    let last_ins = tokenized_line.get(
        end_index - 1,
        "This shouldn't happen, Please report this".to_string(),
        None,
    )?;

    let line_number = first_ins.line_number;
    let column_number = first_ins.column_number;
    let token_length = last_ins.column_number + last_ins.token_length - first_ins.column_number;

    if !is_bx_in_line.0
        && !is_bp_in_line.0
        && !is_si_in_line.0
        && !is_di_in_line.0
        && offset.is_some()
    {
        if open_sqaure_bracket_exists {
            return Ok(Some(Token {
                line_number,
                column_number,
                token_length,
                token_type: Assembly8086Tokens::IndexedAddressing(IndexedAddressingTypes::Offset(
                    offset.unwrap(),
                )),
            }));
        }
        return Ok(Some(Token {
            line_number,
            column_number,
            token_length,
            token_type: Assembly8086Tokens::convert_to_min_num_type(offset.unwrap()),
        }));
    }

    if is_bx_in_line.0 {
        return Ok(Some(Token {
            token_type: match (is_si_in_line.0, is_di_in_line.0) {
                (false, false) => {
                    Assembly8086Tokens::IndexedAddressing(IndexedAddressingTypes::BX(offset))
                }
                (true, false) => {
                    Assembly8086Tokens::IndexedAddressing(IndexedAddressingTypes::BxSi(offset))
                }
                (false, true) => {
                    Assembly8086Tokens::IndexedAddressing(IndexedAddressingTypes::BxDi(offset))
                }
                (true, true) => panic!("This shouldn't happen, Please report this"),
            },
            line_number,
            column_number,
            token_length,
        }));
    }

    if is_bp_in_line.0 {
        return Ok(Some(Token {
            token_type: match (is_si_in_line.0, is_di_in_line.0) {
                (false, false) => {
                    Assembly8086Tokens::IndexedAddressing(IndexedAddressingTypes::BP(offset))
                }
                (true, false) => {
                    Assembly8086Tokens::IndexedAddressing(IndexedAddressingTypes::BpSi(offset))
                }
                (false, true) => {
                    Assembly8086Tokens::IndexedAddressing(IndexedAddressingTypes::BpDi(offset))
                }
                (true, true) => panic!("This shouldn't happen, Please report this"),
            },
            line_number,
            column_number,
            token_length,
        }));
    }

    if is_si_in_line.0 {
        return Ok(Some(Token {
            token_type: Assembly8086Tokens::IndexedAddressing(IndexedAddressingTypes::SI(offset)),
            line_number,
            column_number,
            token_length,
        }));
    }

    if is_di_in_line.0 {
        return Ok(Some(Token {
            token_type: Assembly8086Tokens::IndexedAddressing(IndexedAddressingTypes::DI(offset)),
            line_number,
            column_number,
            token_length,
        }));
    }

    Err(CompilationError::new_without_suggestions(
        line_number,
        column_number,
        token_length,
        "The given argument can't be compiled, Please check the syntax",
    ))
}

pub(crate) fn parse_two_arguments_line<'a>(
    tokenized_line: &'a TokenizedLine<'a>,
    i: usize,
    ins: &'a str,
    variable_ref_map: &mut VariableReferenceMap,
    variable_abs_address_map: &VariableAddressMap,
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

    let compact_high_token =
        get_compact_ins(i + 1, compact_high_until, tokenized_line)?.unwrap_or(high_token.clone());
    let high_token = &compact_high_token;
    let compact_low_token =
        get_compact_ins(compact_high_until + 1, tokenized_line.len(), tokenized_line)?;

    match &high_token.token_type.clone() {
        Assembly8086Tokens::Register16bit(register_type) => {
            check_comma(tokenized_line, high_token, i + 2)?;
            let low_token = tokenized_line.get(
                i + 3,
                format!("Expected 16bit value after {:?} got nothing", high_token).to_string(),
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
                Assembly8086Tokens::IndexedAddressing(_) => {
                    Ok(AddressingMode::Register16bitAndIndexedAddress {
                        high_token: compact_high_token,
                        low_token,
                    })
                }
                Assembly8086Tokens::Character(label) => {
                    Ok(AddressingMode::Register16bitAndAddress {
                        high_token: compact_high_token,
                        low_token,
                        address_bytes: get_label_address_or_push_into_ref(
                            i + 3,
                            label,
                            VariableType::Word,
                            variable_abs_address_map,
                            variable_ref_map,
                        ),
                        register_type: register_type.clone(),
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

        Assembly8086Tokens::Character(label) => {
            check_comma(tokenized_line, high_token, i + 2)?;
            let low_token = tokenized_line.get(
                i + 3,
                format!("Expected 16bit value after {:?} got nothing", high_token).to_string(),
                Some(
                    // try to get label if it doesn't exists show 8bit suggestions else match no the tye of the var and show suggestions
                    if let Some((var_type, _)) = variable_ref_map.get(label) {
                        match var_type {
                            VariableType::Byte => vec![
                                get_all_8bit_registers_suggestions(),
                                get_8bit_number_suggestion(),
                            ],
                            VariableType::Word => vec![
                                get_all_16bit_registers_suggestions(),
                                get_16bit_number_suggestion(),
                                get_8bit_number_suggestion(),
                            ],
                        }
                    } else {
                        vec![
                            get_all_16bit_registers_suggestions(),
                            get_all_8bit_registers_suggestions(),
                            get_16bit_number_suggestion(),
                            get_8bit_number_suggestion(),
                        ]
                    },
                ),
            )?;
            let low_token = match compact_low_token {
                Some(low_token) => low_token,
                None => low_token.clone(),
            };
            match &low_token.token_type.clone() {
                Assembly8086Tokens::Number16bit(num) => Ok(AddressingMode::AddressAnd16bitNumber {
                    high_token: compact_high_token,
                    low_token,
                    address_bytes: get_label_address_or_push_into_ref(
                        i + 1,
                        label,
                        VariableType::Word,
                        variable_abs_address_map,
                        variable_ref_map,
                    ),
                    num: *num,
                }),
                Assembly8086Tokens::Register16bit(low_token_register_type) => {
                    Ok(AddressingMode::AddressAnd16bitRegister {
                        high_token: compact_high_token,
                        low_token,
                        address_bytes: get_label_address_or_push_into_ref(
                            i + 1,
                            label,
                            VariableType::Word,
                            variable_abs_address_map,
                            variable_ref_map,
                        ),
                        register_type: low_token_register_type.clone(),
                    })
                }
                Assembly8086Tokens::Number8bit(num) => Ok(AddressingMode::AddressAnd8bitNumber {
                    high_token: compact_high_token,
                    low_token,
                    address_bytes: get_label_address_or_push_into_ref(
                        i + 1,
                        label,
                        VariableType::Byte,
                        variable_abs_address_map,
                        variable_ref_map,
                    ),
                    num: *num,
                }),
                Assembly8086Tokens::Register8bit(low_token_reg_type) => {
                    Ok(AddressingMode::AddressAnd8bitRegister {
                        high_token: compact_high_token,
                        low_token,
                        address_bytes: get_label_address_or_push_into_ref(
                            i + 1,
                            label,
                            VariableType::Byte,
                            variable_abs_address_map,
                            variable_ref_map,
                        ),
                        register_type: low_token_reg_type.clone(),
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
            let low_token = tokenized_line.get(
                i + 3,
                format!("Expected 8bit value after {:?} got nothing", high_token).to_string(),
                Some(vec![
                    get_all_8bit_registers_suggestions(),
                    get_all_8bit_variables_suggestions(Some(variable_abs_address_map)),
                    get_8bit_number_suggestion(),
                ]),
            )?;
            let low_token = match compact_low_token {
                Some(low_token) => low_token,
                None => low_token.clone(),
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
                Assembly8086Tokens::Character(label) => {
                    Ok(AddressingMode::Register8bitAndAddress {
                        high_token: compact_high_token,
                        low_token,
                        address_bytes: get_label_address_or_push_into_ref(
                            i + 3,
                            label,
                            VariableType::Byte,
                            variable_abs_address_map,
                            variable_ref_map,
                        ),
                        register_type: high_token_type.clone(),
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
