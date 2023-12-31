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

    // if start token is a register return we don't want to evaluate it
    if (end_index - start_index) == 1 {
        let start_token = tokenized_line.get(
            start_index,
            "This shouldn't happen, Please report this".to_string(),
            None,
        )?;

        match start_token.token_type {
            Assembly8086Tokens::Register16bit(_)
            | Assembly8086Tokens::Register8bit(_)
            // | Assembly8086Tokens::Number16bit(_)
            // | Assembly8086Tokens::Number8bit(_)
            | Assembly8086Tokens::Instruction(_) => {
                return Ok(None);
            }
            _ => {}
        }
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
    let offset_token_enum = Assembly8086Tokens::AssemblerDirectives(AssemblerDirectives::Offset);
    let mut is_offset_directive_defined = tokenized_line.find_token(offset_token_enum).is_some();
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
    let mut variable_type = None;

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
                    return Err(CompilationError::error_with_token(
                        token,
                        &format!(
                            "Expected a 16bit register only **BX, SI, DI & BP** are valid but got {:?} insted",
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
            Assembly8086Tokens::AssemblerDirectives(AssemblerDirectives::AsByte) => {
                if variable_type.is_none() {
                    variable_type = Some(VariableType::Byte);
                }
            }

            Assembly8086Tokens::AssemblerDirectives(AssemblerDirectives::AsWord) => {
                variable_type = Some(VariableType::Word);
            }

            Assembly8086Tokens::Character(label) => {
                let is_address_or_num = get_label_address_or_push_into_ref(
                    i,
                    label,
                    token,
                    is_org_defined,
                    is_offset_directive_defined,
                    VariableType::Byte,
                    variable_abs_address_map,
                    var_ref_map,
                    label_idx_map,
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
                        if is_offset_directive_defined {
                            label_idx_map.insert(
                                label.to_string(),
                                (token.clone(), i, is_offset_directive_defined),
                            );
                        }
                        open_sqaure_bracket_exists = true;
                        stack.push(StackItem::Number(token, SignedU16::from(addr_byte)));
                        indexing_type = IndexingType::VariableIndexing;
                        let var_type = variable_abs_address_map
                            .get(label)
                            .unwrap_or(&(VariableType::Word, 0))
                            .0;
                        // variable_type = Some(var_type);
                        if var_type == VariableType::Byte && variable_type.is_none() {
                            variable_type = Some(VariableType::Byte);
                        }
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
                            low_token,
                            high_token,
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
                let sign = operator_stack.pop();
                if let Some(StackOperator::Minus) = sign {
                    offset = Some(num.negate());
                } else {
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

    fn get_indexed_addressing(
        addressing_type: IndexedAddressingTypes,
        variable_type: VariableType,
    ) -> Assembly8086Tokens {
        // Assembly8086Tokens::IndexedAddressing(addressing_type)
        match variable_type {
            VariableType::Byte => Assembly8086Tokens::ByteIndexedAddressing(addressing_type),
            VariableType::Word => Assembly8086Tokens::IndexedAddressing(addressing_type),
        }
    }

    let variable_type = variable_type.unwrap_or(VariableType::Word);

    if is_offset_directive_defined
        || (!is_bx_in_line.0 && !is_bp_in_line.0 && !is_si_in_line.0 && !is_di_in_line.0)
    {
        if let Some(num_type) = offset {
            if open_sqaure_bracket_exists && !is_offset_directive_defined {
                return Ok(Some(Token {
                    line_number,
                    column_number,
                    token_length,
                    token_type: get_indexed_addressing(
                        IndexedAddressingTypes::Offset(num_type),
                        variable_type,
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
                    get_indexed_addressing(IndexedAddressingTypes::BX(offset), variable_type)
                }
                (true, false) => {
                    get_indexed_addressing(IndexedAddressingTypes::BxSi(offset), variable_type)
                }
                (false, true) => {
                    get_indexed_addressing(IndexedAddressingTypes::BxDi(offset), variable_type)
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
                    get_indexed_addressing(IndexedAddressingTypes::BP(offset), variable_type)
                }
                (true, false) => {
                    get_indexed_addressing(IndexedAddressingTypes::BpSi(offset), variable_type)
                }
                (false, true) => {
                    get_indexed_addressing(IndexedAddressingTypes::BpDi(offset), variable_type)
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
            token_type: get_indexed_addressing(IndexedAddressingTypes::SI(offset), variable_type),
            line_number,
            column_number,
            token_length,
        }));
    }

    if is_di_in_line.0 {
        return Ok(Some(Token {
            token_type: get_indexed_addressing(IndexedAddressingTypes::DI(offset), variable_type),
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

#[allow(clippy::too_many_arguments)]
pub(in crate::compiler) fn get_label_address_or_push_into_ref(
    idx: ArrayIndex,
    label: &Label,
    token: &Token,
    is_org_defined: bool,
    is_offset_directive_defined: bool,
    var_type: VariableType,
    variable_abs_offset_bytes_map: &VariableAddressMap,
    var_ref_map: &mut VariableReferenceMap,
    label_idx_map: &mut HashMap<String, (Token, usize, bool)>,
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
                    if !is_offset_directive_defined {
                        var_ref_map.insert(label.clone(), (var_type, idx));
                    } else {
                        label_idx_map.insert(
                            label.to_string(),
                            (token.clone(), idx, is_offset_directive_defined),
                        );
                    }
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
