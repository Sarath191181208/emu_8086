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
            registers16bit::Registers16bit, registers8bit::Registers8bit, Assembly8086Tokens, Token,
        },
        types_structs::{
            ArrayIndex, Label, VariableAddressMap, VariableReferenceMap, VariableType,
        },
    },
    utils::Either,
};

use super::utils::check_comma;

pub(crate) enum AddressingMode<'a> {
    Registers16bit {
        high_token: &'a Token,
        low_token: &'a Token,
    },
    Registers8bit {
        high_token: &'a Token,
        low_token: &'a Token,
    },
    Registers16bitNumber {
        high_token: &'a Token,
        low_token: &'a Token,
        num: Either<u8, u16>,
    },
    Register8bitNumber {
        high_token: &'a Token,
        low_token: &'a Token,
        num: u8,
    },

    Register16bitAndAddress {
        high_token: &'a Token,
        low_token: &'a Token,
        address_bytes: [u8; 2],
        register_type: Registers16bit,
    },
    AddressAnd16bitRegister {
        high_token: &'a Token,
        low_token: Token,
        address_bytes: [u8; 2],
        register_type: Registers16bit,
    },
    AddressAnd16bitNumber {
        high_token: &'a Token,
        low_token: Token,
        address_bytes: [u8; 2],
        num: u16,
    },
    Register8bitAndAddress {
        high_token: &'a Token,
        low_token: &'a Token,
        address_bytes: [u8; 2],
        register_type: Registers8bit,
    },
    AddressAnd8bitRegister {
        high_token: &'a Token,
        low_token: &'a Token,
        address_bytes: [u8; 2],
        register_type: Registers8bit,
    },
    AddressAnd8bitNumber {
        high_token: &'a Token,
        low_token: &'a Token,
        address_bytes: [u8; 2],
        num: u8,
    },
}

fn get_label_address_or_push_into_ref(
    idx: ArrayIndex,
    label: &Label,
    var_type: VariableType,
    variable_abs_offset_bytes_map: &VariableAddressMap,
    var_ref_map: &mut VariableReferenceMap,
) -> [u8; 2] {
    match variable_abs_offset_bytes_map.get(label) {
        Some((_, abs_addr)) => {
            let ins = (abs_addr & 0xFF) as u8;
            let ins2 = (abs_addr >> 8) as u8;
            [ins, ins2]
        }
        None => {
            let placeholder = [0x00, 0x00];
            var_ref_map.insert(label.clone(), (var_type, idx));
            placeholder
        }
    }
}

pub(crate) fn parse_two_arguments_line<'a>(
    tokenized_line: &'a TokenizedLine<'a>,
    i: usize,
    ins: &'a str,
    variable_ref_map: &mut VariableReferenceMap,
    variable_abs_address_map: &VariableAddressMap,
) -> Result<AddressingMode<'a>, CompilationError> {
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

    match &high_token.token_type {
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
            match &low_token.token_type {
                Assembly8086Tokens::Number16bit(num) => Ok(AddressingMode::Registers16bitNumber {
                    high_token,
                    low_token,
                    num: Either::Right(*num),
                }),
                Assembly8086Tokens::Number8bit(num) => Ok(AddressingMode::Registers16bitNumber {
                    high_token,
                    low_token,
                    num: Either::Left(*num),
                }),
                Assembly8086Tokens::Register16bit(_) => Ok(AddressingMode::Registers16bit {
                    high_token,
                    low_token,
                }),
                Assembly8086Tokens::Character(label) => {
                    Ok(AddressingMode::Register16bitAndAddress {
                        high_token,
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
            match &low_token.token_type {
                Assembly8086Tokens::Number16bit(num) => Ok(AddressingMode::AddressAnd16bitNumber {
                    high_token,
                    low_token: low_token.clone(),
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
                        high_token,
                        low_token: low_token.clone(),
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
                    high_token,
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
                        high_token,
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
            match &low_token.token_type {
                Assembly8086Tokens::Number8bit(num) => Ok(AddressingMode::Register8bitNumber {
                    high_token,
                    low_token,
                    num: *num,
                }),
                Assembly8086Tokens::Register8bit(_) => Ok(AddressingMode::Registers8bit {
                    high_token,
                    low_token,
                }),
                Assembly8086Tokens::Character(label) => {
                    Ok(AddressingMode::Register8bitAndAddress {
                        high_token,
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
