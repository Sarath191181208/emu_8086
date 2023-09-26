use crate::compiler::{
    compilation_error::CompilationError,
    tokenized_line::TokenizedLine,
    tokens::{Assembly8086Tokens, Token},
    types_structs::{
        ArrayIndex, Label, VariableReferenceMap, VariableAddressMap, VariableType,
    },
};

use super::utils::{check_comma, if_num_8bit_to_16bit};

pub(crate) enum AddressingMode<'a> {
    Registers16bit {
        high_token: &'a Token,
        low_token: Token,
    },
    Registers8bit {
        high_token: &'a Token,
        low_token: &'a Token,
    },
    Registers16bitNumber {
        high_token: &'a Token,
        low_token: Token,
    },
    Register8bitNumber {
        high_token: &'a Token,
        low_token: &'a Token,
    },

    Register16bitAndVariable {
        high_token: &'a Token,
        low_token: Token,
        address_bytes: [u8; 2],
    },
    VariableAnd16bitRegister {
        high_token: &'a Token,
        low_token: Token,
        address_bytes: [u8; 2],
    },
    VariableAnd16bitNumber {
        high_token: &'a Token,
        low_token: Token,
        address_bytes: [u8; 2],
    },
    Register8bitAndVariable {
        high_token: &'a Token,
        low_token: &'a Token,
        address_bytes: [u8; 2],
    },
    VariableAnd8bitRegister {
        high_token: &'a Token,
        low_token: &'a Token,
        address_bytes: [u8; 2],
    },
    VariableAnd8bitNumber {
        high_token: &'a Token,
        low_token: &'a Token,
        address_bytes: [u8; 2],
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
        Some(abs_addr) => {
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

pub(crate) fn parse_line<'a>(
    tokenized_line: &'a TokenizedLine<'a>,
    i: usize,
    ins: &'a str,
    variable_ref_map: &mut VariableReferenceMap,
    variable_abs_address_map: &VariableAddressMap,
) -> Result<AddressingMode<'a>, CompilationError> {
    let len_lexed_strings = tokenized_line.get_len_lexed_strings();
    let token = tokenized_line.get(i, "This shouldn't happen, Please report this".to_string())?;
    let high_token = tokenized_line.get(
        i + 1,
        format!("Expected arguments after {} got nothing", ins).to_string(),
    )?;

    match &high_token.token_type {
        Assembly8086Tokens::Register16bit(_) => {
            check_comma(tokenized_line, high_token, i + 2)?;
            let low_token = tokenized_line.get(
                i + 3,
                format!("Expected 16bit value after {:?} got nothing", high_token).to_string(),
            )?;
            let changed_low_token = Token::new(
                if_num_8bit_to_16bit(low_token.token_type.clone()),
                low_token.line_number,
                low_token.column_number,
                low_token.token_length,
            );
            match &changed_low_token.token_type {
                Assembly8086Tokens::Number16bit(_) => Ok(AddressingMode::Registers16bitNumber {
                    high_token,
                    low_token: changed_low_token.clone(),
                }),
                Assembly8086Tokens::Register16bit(_) => Ok(AddressingMode::Registers16bit {
                    high_token,
                    low_token: changed_low_token.clone(),
                }),
                Assembly8086Tokens::Character(label) => {
                    Ok(AddressingMode::Register16bitAndVariable {
                        high_token,
                        low_token: changed_low_token.clone(),
                        address_bytes: get_label_address_or_push_into_ref(
                            i + 3,
                            label,
                            VariableType::Word,
                            variable_abs_address_map,
                            variable_ref_map,
                        ),
                    })
                }

                _ => Err(CompilationError::new(
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
            )?;
            // let changed_low_token = Token::new(
            //     if_num_8bit_to_16bit(low_token.token_type.clone()),
            //     low_token.line_number,
            //     low_token.column_number,
            //     low_token.token_length,
            // );
            match &low_token.token_type {
                Assembly8086Tokens::Number16bit(_) => Ok(AddressingMode::VariableAnd16bitNumber {
                    high_token,
                    low_token: low_token.clone(),
                    address_bytes: get_label_address_or_push_into_ref(
                        i + 1,
                        label,
                        VariableType::Word,
                        variable_abs_address_map,
                        variable_ref_map,
                    ),
                }),
                Assembly8086Tokens::Register16bit(_) => {
                    Ok(AddressingMode::VariableAnd16bitRegister {
                        high_token,
                        low_token: low_token.clone(),
                        address_bytes: get_label_address_or_push_into_ref(
                            i + 1,
                            label,
                            VariableType::Word,
                            variable_abs_address_map,
                            variable_ref_map,
                        ),
                    })
                }
                Assembly8086Tokens::Number8bit(_) => Ok(AddressingMode::VariableAnd8bitNumber {
                    high_token,
                    low_token,
                    address_bytes: get_label_address_or_push_into_ref(
                        i + 1,
                        label,
                        VariableType::Byte,
                        variable_abs_address_map,
                        variable_ref_map,
                    ),
                }),
                Assembly8086Tokens::Register8bit(_) => {
                    Ok(AddressingMode::VariableAnd8bitRegister {
                        high_token,
                        low_token,
                        address_bytes: get_label_address_or_push_into_ref(
                            i + 1,
                            label,
                            VariableType::Byte,
                            variable_abs_address_map,
                            variable_ref_map,
                        ),
                    })
                }
                _ => Err(CompilationError::new(
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

        Assembly8086Tokens::Register8bit(_) => {
            check_comma(tokenized_line, high_token, i + 2)?;
            let low_token = tokenized_line.get(
                i + 3,
                format!("Expected 8bit value after {:?} got nothing", high_token).to_string(),
            )?;
            match &low_token.token_type {
                Assembly8086Tokens::Number8bit(_) => Ok(AddressingMode::Register8bitNumber {
                    high_token,
                    low_token,
                }),
                Assembly8086Tokens::Register8bit(_) => Ok(AddressingMode::Registers8bit {
                    high_token,
                    low_token,
                }),
                Assembly8086Tokens::Character(label) => {
                    Ok(AddressingMode::Register8bitAndVariable {
                        high_token,
                        low_token,
                        address_bytes: get_label_address_or_push_into_ref(
                            i + 3,
                            label,
                            VariableType::Byte,
                            variable_abs_address_map,
                            variable_ref_map,
                        ),
                    })
                }

                _ => Err(CompilationError::new(
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

        _ => Err(CompilationError::new(
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
