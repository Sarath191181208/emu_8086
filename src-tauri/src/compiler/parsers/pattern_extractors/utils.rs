use std::collections::HashMap;

use crate::{
    compiler::{
        compilation_error::CompilationError,
        tokenized_line::TokenizedLine,
        tokens::{
            assembler_directives::AssemblerDirectives,
            indexed_addressing_types::IndexedAddressingTypes, registers16bit::Registers16bit,
            Assembly8086Tokens, SignedU16, Token,
        },
        types_structs::{
            ArrayIndex, Label, VariableAddressMap, VariableReferenceMap, VariableType,
        },
        CompiledLineLabelRef,
    },
    utils::Either,
};

#[allow(clippy::too_many_arguments)]
pub(in crate::compiler) fn evaluate_ins<'a>(
    start_index: usize,
    end_index: usize,
    tokenized_line: &'a TokenizedLine<'a>,
    is_org_defined: bool,
    label_idx_map: &mut HashMap<String, (Token, usize, bool)>,
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
    let mut open_sqaure_bracket_exists = tokenized_line
        .slice(start_index, end_index)
        .iter()
        .any(|token| token.token_type == Assembly8086Tokens::OpenSquareBracket);

    // possible patterns are
    // bx + si ; bx + di ; bp + si ; bp + di ; si ; di ; bp ; bx  all with offsets

    let mut is_bx_in_line = (false, 0_u32);
    let mut is_bp_in_line = (false, 0_u32);
    let mut is_si_in_line = (false, 0_u32);
    let mut is_di_in_line = (false, 0_u32);
    let mut is_offset_directive_defined = false;
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

    #[derive(PartialEq)]
    enum IndexingType {
        LabelIndexing,
        VariableIndexing,
        Undefined,
    }

    let mut indexing_type = IndexingType::Undefined;

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
            Assembly8086Tokens::AssemblerDirectives(AssemblerDirectives::Offset) => {
                is_offset_directive_defined = true;
            }
            Assembly8086Tokens::Character(label) => {
                let is_address_or_num = get_label_address_or_push_into_ref(
                    i,
                    label,
                    is_org_defined,
                    VariableType::Word,
                    variable_abs_address_map,
                    var_ref_map,
                    compiled_line_offset_maps,
                );

                match is_address_or_num {
                    Either::Left(addr_byte) => {
                        if indexing_type == IndexingType::LabelIndexing {
                            return Err(CompilationError::error_with_token(
                                token,
                                "Can't use a label along with a variable in the same expression",
                            ));
                        }
                        label_idx_map.insert(
                            label.to_string(),
                            (token.clone(), i, is_offset_directive_defined),
                        );
                        open_sqaure_bracket_exists = true;
                        stack.push(StackItem::Number(token, SignedU16::from(addr_byte)));
                        indexing_type = IndexingType::VariableIndexing;
                    }
                    Either::Right(num) => {
                        if indexing_type == IndexingType::VariableIndexing {
                            return Err(CompilationError::error_with_token(
                                token,
                                "Can't use a variable along with a label in the same expression",
                            ));
                        }
                        stack.push(StackItem::Number(token, SignedU16::from(num)));
                        indexing_type = IndexingType::LabelIndexing;
                    }
                }
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
                    stack.push(StackItem::Number(low_token, res.negate()));
                }
            }
        }
    }

    if !stack.is_empty() {
        let item = stack.pop().unwrap();
        match item {
            StackItem::Register16bit(_) => {}
            StackItem::Number(_, num) => {
                offset = Some(num);
            }
        }
    }

    if is_bx_in_line.0 && is_bp_in_line.0 {
        let token = tokenized_line.get(
            is_bx_in_line.1 as usize,
            "This shouldn't happen, Please report this".to_string(),
            None,
        )?;
        return Err(CompilationError::error_with_token(
            token,
            "Expected either bx or bp got both",
        ));
    }

    if is_si_in_line.0 && is_di_in_line.0 {
        let token = tokenized_line.get(
            is_si_in_line.1 as usize,
            "This shouldn't happen, Please report this".to_string(),
            None,
        )?;
        return Err(CompilationError::error_with_token(
            token,
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

    if is_offset_directive_defined
        || (!is_bx_in_line.0 && !is_bp_in_line.0 && !is_si_in_line.0 && !is_di_in_line.0)
    {
        if let Some(num_type) = offset {
            if open_sqaure_bracket_exists && !is_offset_directive_defined {
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

type Address = [u8; 2];
type Number = Either<u8, u16>;

pub(in crate::compiler) fn get_label_address_or_push_into_ref(
    idx: ArrayIndex,
    label: &Label,
    is_org_defined: bool,
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
                Some(offset) => {
                    let optional_offset = match is_org_defined {
                        true => 0x100,
                        false => 0x00,
                    };
                    Either::Right(Either::<u8, u16>::from(offset + optional_offset))
                }
            }
        }
    }
}
