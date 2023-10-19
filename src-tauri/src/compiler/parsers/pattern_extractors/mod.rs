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
        types_structs::{
            ArrayIndex, Label, LineNumber, VariableAddressMap, VariableReferenceMap, VariableType,
        },
        CompiledLineLabelRef,
    },
    utils::Either,
};

use super::utils::check_comma;

pub(in super::super) mod compile_two_arguments_patterns;

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
}

fn get_compact_ins<'a>(
    start_index: usize,
    end_index: usize,
    tokenized_line: &'a TokenizedLine<'a>,

    var_ref_map: &mut VariableReferenceMap,
    variable_abs_address_map: &VariableAddressMap,
    compiled_line_offset_maps: Option<&CompiledLineLabelRef>,
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

    let mut is_bx_in_line = (false, 0_u32);
    let mut is_bp_in_line = (false, 0_u32);
    let mut is_si_in_line = (false, 0_u32);
    let mut is_di_in_line = (false, 0_u32);
    let mut offset: Option<SignedU16> = None;

    // use stack to convert the expression into a postifx one

    enum StackItem<'a> {
        Register16bit(&'a Token),
        Number(&'a Token, SignedU16),
    }

    enum StackOperator {
        Plus,
        Minus,
    }

    let mut stack = Vec::<StackItem>::new();
    let mut operator_stack = Vec::<StackOperator>::new();
    for i in start_index..end_index {
        let token = tokenized_line.get(
            i,
            "This shouldn't happen, Please report this".to_string(),
            None,
        )?;

        match &token.token_type {
            Assembly8086Tokens::Register16bit(reg) => match reg {
                Registers16bit::BX => {
                    is_bx_in_line = (true, i as u32);
                    stack.push(StackItem::Register16bit(token))
                }
                Registers16bit::SI => {
                    is_si_in_line = (true, i as u32);
                    stack.push(StackItem::Register16bit(token))
                }
                Registers16bit::DI => {
                    is_di_in_line = (true, i as u32);
                    stack.push(StackItem::Register16bit(token))
                }
                Registers16bit::BP => {
                    is_bp_in_line = (true, i as u32);
                    stack.push(StackItem::Register16bit(token));
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
            },
            Assembly8086Tokens::Number8bit(val) => {
                stack.push(StackItem::Number(token, SignedU16::new(*val as u16)));
            }
            Assembly8086Tokens::Number16bit(val) => {
                stack.push(StackItem::Number(token, SignedU16::new(*val)));
            }
            Assembly8086Tokens::Plus => {
                operator_stack.push(StackOperator::Plus);
            }
            Assembly8086Tokens::Minus => {
                operator_stack.push(StackOperator::Minus);
            }
            Assembly8086Tokens::OpenSquareBracket | Assembly8086Tokens::CloseSquareBracket => {
                operator_stack.push(StackOperator::Plus)
            }
            Assembly8086Tokens::Character(label) => {
                let addr_bytes = get_label_address_or_push_into_ref(
                    i,
                    label,
                    VariableType::Word,
                    variable_abs_address_map,
                    var_ref_map,
                    compiled_line_offset_maps,
                );

                let val = match addr_bytes {
                    Either::Left(val) => Ok(SignedU16::from(val)),
                    Either::Right(val) => Ok(SignedU16::from(val)),
                }?;

                stack.push(StackItem::Number(token, val));
            }

            _ => {
                if operator_stack.is_empty() && !stack.is_empty() {
                    return Err(CompilationError::error_with_token(
                        token,
                        &format!("Expected an operator got {:?} insted", token.token_type),
                    ));
                } else {
                    return Err(CompilationError::error_with_token(
                        token,
                        &format!(
                            "Expected a 16bit register or a number got {:?} insted",
                            token.token_type
                        ),
                    ));
                }
            }
        }

        if stack.len() == 2 && operator_stack.is_empty() {
            let token = match stack.get(0).unwrap() {
                StackItem::Register16bit(token) => token,
                StackItem::Number(token, _) => token,
            };
            return Err(CompilationError::error_with_token(
                token,
                &format!(
                    "Expected an operator after {:?} got nothing",
                    token.token_type
                ),
            ));
        }

        if stack.len() == 2 && !operator_stack.is_empty() {
            let operator = operator_stack.pop().unwrap();
            let high_item = stack.pop().unwrap();
            let low_item = stack.pop().unwrap();

            match (high_item, operator, low_item) {
                (StackItem::Register16bit(_), _, StackItem::Register16bit(reg2)) => {
                    stack.push(StackItem::Register16bit(reg2));
                }
                (
                    StackItem::Number(token, num),
                    StackOperator::Plus,
                    StackItem::Register16bit(_),
                )
                | (
                    StackItem::Register16bit(_),
                    StackOperator::Plus,
                    StackItem::Number(token, num),
                ) => {
                    stack.push(StackItem::Number(token, num));
                }
                (
                    StackItem::Number(token, num),
                    StackOperator::Minus,
                    StackItem::Register16bit(_),
                )
                | (
                    StackItem::Register16bit(_),
                    StackOperator::Minus,
                    StackItem::Number(token, num),
                ) => {
                    stack.push(StackItem::Number(token, num.negate()));
                }
                (
                    StackItem::Number(high_token, val_high),
                    StackOperator::Plus,
                    StackItem::Number(low_token, val_low),
                ) => {
                    let (res, overflow) = val_high.overflowing_add(val_low);
                    if overflow {
                        return Err(CompilationError::error_between_tokens(
                            high_token,
                            low_token,
                            "The sum of the values overflows",
                        ));
                    }
                    stack.push(StackItem::Number(low_token, res));
                }
                (
                    StackItem::Number(high_token, val_high),
                    StackOperator::Minus,
                    StackItem::Number(low_token, val_low),
                ) => {
                    let (res, overflow) = val_high.overflowing_sub(val_low);
                    if overflow {
                        return Err(CompilationError::error_between_tokens(
                            high_token,
                            low_token,
                            "The sum of the values overflows",
                        ));
                    }
                    stack.push(StackItem::Number(low_token, res));
                }
            }
        }
    }

    if !stack.is_empty() {
        let item = stack.pop().unwrap();
        match item {
            StackItem::Register16bit(_) => {}
            StackItem::Number(_, num) => {
                if num.val != 0 {
                    offset = Some(num);
                }
            }
        }
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

    if !is_bx_in_line.0 && !is_bp_in_line.0 && !is_si_in_line.0 && !is_di_in_line.0 {
        if let Some(num_type) = offset {
            if open_sqaure_bracket_exists {
                return Ok(Some(Token {
                    line_number,
                    column_number,
                    token_length,
                    token_type: Assembly8086Tokens::IndexedAddressing(
                        IndexedAddressingTypes::Offset(num_type),
                    ),
                }));
            }
            let num_type = match num_type.as_num_token() {
                Ok(tt) => tt,
                Err(err) => {
                    return Err(CompilationError::new_without_suggestions(
                        line_number,
                        column_number,
                        token_length,
                        err,
                    ))
                }
            };
            return Ok(Some(Token {
                line_number,
                column_number,
                token_length,
                token_type: num_type,
            }));
        }
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
                (true, true) => return Err(CompilationError::default()),
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
                (true, true) => return Err(CompilationError::default()),
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
    _line_number: LineNumber,
    ins: &'a str,
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

    let compact_high_token = get_compact_ins(
        i + 1,
        compact_high_until,
        tokenized_line,
        variable_ref_map,
        variable_abs_address_map,
        compiled_line_offset_maps,
    )?
    .unwrap_or(high_token.clone());
    let high_token = &compact_high_token;
    let compact_low_token = get_compact_ins(
        compact_high_until + 1,
        tokenized_line.len(),
        tokenized_line,
        variable_ref_map,
        variable_abs_address_map,
        compiled_line_offset_maps,
    )?;

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
                Assembly8086Tokens::Character(label) => {
                    let addr_bytes = get_label_address_or_push_into_ref(
                        i + 3,
                        label,
                        VariableType::Word,
                        variable_abs_address_map,
                        variable_ref_map,
                        compiled_line_offset_maps,
                    );

                    match addr_bytes {
                        Either::Left(bytes) => Ok(AddressingMode::Register16bitAndAddress {
                            high_token: compact_high_token,
                            low_token,
                            address_bytes: bytes,
                            register_type: register_type.clone(),
                        }),
                        Either::Right(number) => Ok(AddressingMode::Registers16bitNumber {
                            high_token: compact_high_token,
                            low_token: low_token.clone(),
                            num: Either::Right(number.get_as_u16()),
                        }),
                    }
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
                Assembly8086Tokens::Number16bit(num) => {
                    let address_bytes = get_label_address_or_push_into_ref(
                        i + 1,
                        label,
                        VariableType::Word,
                        variable_abs_address_map,
                        variable_ref_map,
                        compiled_line_offset_maps,
                    );

                    let address_bytes = match address_bytes {
                        Either::Left(bytes) => bytes,
                        Either::Right(number) => number.get_as_u16().to_le_bytes(),
                    };

                    Ok(AddressingMode::AddressAnd16bitNumber {
                        high_token: compact_high_token,
                        low_token,
                        num: *num,
                        address_bytes,
                    })
                }
                Assembly8086Tokens::Register16bit(low_token_register_type) => {
                    let address_bytes = get_label_address_or_push_into_ref(
                        i + 1,
                        label,
                        VariableType::Word,
                        variable_abs_address_map,
                        variable_ref_map,
                        compiled_line_offset_maps,
                    );

                    let address_bytes = match address_bytes {
                        Either::Left(bytes) => bytes,
                        Either::Right(number) => number.get_as_u16().to_le_bytes(),
                    };

                    Ok(AddressingMode::AddressAnd16bitRegister {
                        high_token: compact_high_token,
                        low_token,
                        register_type: low_token_register_type.clone(),
                        address_bytes,
                    })
                }
                Assembly8086Tokens::Number8bit(num) => {
                    let address_bytes = get_label_address_or_push_into_ref(
                        i + 1,
                        label,
                        VariableType::Byte,
                        variable_abs_address_map,
                        variable_ref_map,
                        compiled_line_offset_maps,
                    );

                    let address_bytes = match address_bytes {
                        Either::Left(bytes) => bytes,
                        Either::Right(number) => number.get_as_u16().to_le_bytes(),
                    };

                    Ok(AddressingMode::AddressAnd8bitNumber {
                        high_token: compact_high_token,
                        low_token,
                        address_bytes,
                        num: *num,
                    })
                }

                Assembly8086Tokens::Register8bit(low_token_reg_type) => {
                    let address_bytes = get_label_address_or_push_into_ref(
                        i + 1,
                        label,
                        VariableType::Byte,
                        variable_abs_address_map,
                        variable_ref_map,
                        compiled_line_offset_maps,
                    );

                    let address_bytes = match address_bytes {
                        Either::Left(bytes) => bytes,
                        Either::Right(number) => number.get_as_u16().to_le_bytes(),
                    };

                    Ok(AddressingMode::AddressAnd8bitRegister {
                        high_token: compact_high_token,
                        low_token,
                        register_type: low_token_reg_type.clone(),
                        address_bytes,
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
                Assembly8086Tokens::IndexedAddressing(field) => match field.get_offset() {
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
                        VariableType::Byte,
                        variable_abs_address_map,
                        variable_ref_map,
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
                            Number::Left(num) => Ok(AddressingMode::Register8bitNumber {
                                high_token: compact_high_token,
                                low_token,
                                num,
                            }),
                            Number::Right(_) => Err(CompilationError::error_with_token(
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

type Address = [u8; 2];
type Number = Either<u8, u16>;

fn get_label_address_or_push_into_ref(
    idx: ArrayIndex,
    label: &Label,
    var_type: VariableType,
    variable_abs_offset_bytes_map: &VariableAddressMap,
    var_ref_map: &mut VariableReferenceMap,
    compiled_line_offset_maps: Option<&CompiledLineLabelRef>,
) -> Either<Address, Number> {
    let placeholder = [0x00, 0x00];
    match variable_abs_offset_bytes_map.get(label) {
        Some((_, abs_addr)) => {
            let ins = (abs_addr & 0xFF) as u8;
            let ins2 = (abs_addr >> 8) as u8;
            Either::Left([ins, ins2])
        }
        None => {
            let offset: Option<i16> = match compiled_line_offset_maps {
                None => None,
                Some(compiled_line_offset_maps) => {
                    let start_line_num = 0;
                    compiled_line_offset_maps
                        .find_label_offset_or_proc_offset(label, start_line_num)
                }
            };
            match offset {
                None => {
                    var_ref_map.insert(label.clone(), (var_type, idx));
                    Either::Left(placeholder)
                }
                Some(offset) => Either::Right(Either::<u8, u16>::from(offset)),
            }
        }
    }
}
