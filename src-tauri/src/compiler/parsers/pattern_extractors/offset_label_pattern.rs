/// This if for instructions fowlling this pattern
/// ```asm
///     INS OFFSET LABEL
///     INS LABEL
/// ```
use std::collections::HashMap;

use crate::{
    compiler::{
        compilation_error::CompilationError,
        parsers::utils::{check_token, push_instruction},
        suggestions_utils::get_all_registers_and_variable_suggestions,
        tokenized_line::TokenizedLine,
        tokens::{assembler_directives::AssemblerDirectives, Assembly8086Tokens, Token},
        types_structs::{CompiledBytesReference, Label, LineNumber, VariableAddressMap},
        CompiledLineLabelRef,
    },
    convert_and_push_instructions,
    utils::Either,
};

#[allow(clippy::too_many_arguments)]
pub(in crate::compiler) fn parse_labeled_relative_offset(
    tokenized_line: &TokenizedLine,
    i: usize,
    line_number: LineNumber,
    instruction_name: &str,
    compiled_bytes: &mut Vec<u8>,
    compiled_bytes_ref: &mut Vec<CompiledBytesReference>,
    variable_address_map: Option<&VariableAddressMap>,
    label_idx_map: &mut HashMap<String, (Token, usize, bool)>,
    compiled_line_ref_with_offset_maps: Option<&CompiledLineLabelRef>,
    instruction_compile_data: &mut LabeledInstructionCompileData,
) -> Result<usize, CompilationError> {
    let (i, token, high_token, is_offset) = parse_token_high_token_and_is_offset_defined(
        tokenized_line,
        i,
        variable_address_map,
        instruction_name,
    )?;

    instruction_compile_data.is_offset = is_offset;

    let offset_case = parse_single_label_or_variable(
        tokenized_line,
        i,
        line_number,
        instruction_name,
        high_token,
        instruction_compile_data,
        OffsetMaps {
            label_idx_map,
            compiled_line_ref_with_offset_maps,
            variable_address_map,
        },
    )?;

    Ok(compile_single_ins_similar_as_jmp(
        i,
        token,
        high_token,
        instruction_compile_data.clone(),
        offset_case,
        compiled_bytes,
        compiled_bytes_ref,
    ))
}

pub(in crate::compiler) enum Offset {
    U8(u8),
    U16(u16),
    Pointer(u16),
    SegmentedAddressing(u16, u16),
}

pub(in crate::compiler) struct OffsetMaps<'a> {
    pub label_idx_map: &'a mut HashMap<String, (Token, usize, bool)>,
    pub compiled_line_ref_with_offset_maps: Option<&'a CompiledLineLabelRef<'a>>,
    pub variable_address_map: Option<&'a VariableAddressMap>,
}

#[derive(Debug, Clone)]
pub(in crate::compiler) struct LabeledInstructionCompileData {
    pub ins_8bit: Vec<u8>,
    pub ins_16bit: Vec<u8>,
    pub pointer_offset_instruction: Vec<u8>,
    pub segmented_indexing_instruction: Vec<u8>,

    pub bytes_of_8bit_ins: u8,
    pub bytes_of_16bit_ins: u16,

    pub is_offset: bool,
}

pub(in crate::compiler) fn parse_single_label_or_variable(
    tokenized_line: &TokenizedLine,
    i: usize,
    line_number: LineNumber,
    instruction_name: &str,
    token: &Token,
    instruction_compile_data: &LabeledInstructionCompileData,
    offset_maps: OffsetMaps,
) -> Result<Offset, CompilationError> {
    let is_offset_defined = instruction_compile_data.is_offset;
    match &token.token_type {
        Assembly8086Tokens::Character(label) => {
            let offset_bytes_from_line_and_is_label_before_ref =
                get_offset_bytes_from_line_from_maps(
                    label,
                    line_number,
                    is_offset_defined,
                    offset_maps.compiled_line_ref_with_offset_maps,
                );

            if offset_bytes_from_line_and_is_label_before_ref.is_none() {
                offset_maps
                    .label_idx_map
                    .insert(label.to_string(), (token.clone(), 1, is_offset_defined));
            }

            let (offset_bytes, is_jmp_after_label) =
                offset_bytes_from_line_and_is_label_before_ref.unwrap_or((0, false));

            if let Some(addr) = get_address(label, offset_maps.variable_address_map) {
                if !is_offset_defined {
                    return Ok(Offset::Pointer(u16::from_le_bytes(addr)));
                }
            }

            match calc_offset(
                offset_bytes,
                is_jmp_after_label,
                instruction_compile_data.bytes_of_8bit_ins,
                instruction_compile_data.bytes_of_16bit_ins,
            ) {
                Either::Left(num) => Ok(Offset::U8(num)),
                Either::Right(num) => Ok(Offset::U16(num)),
            }
        }

        Assembly8086Tokens::Number16bit(segment_num) => {
            // check if the next token is Colon
            check_token(tokenized_line, token, i + 2, &Assembly8086Tokens::Colon)?;

            let low_token =
                tokenized_line.get(i + 3, "Expected a 8bit token got None!".to_string(), None)?;

            match &low_token.token_type {
                Assembly8086Tokens::Number8bit(address_num) => Ok(Offset::SegmentedAddressing(
                    *segment_num,
                    *address_num as u16,
                )),
                Assembly8086Tokens::Number16bit(address_num) => {
                    Ok(Offset::SegmentedAddressing(*segment_num, *address_num))
                }

                _ => Err(CompilationError::error_with_token(
                    token,
                    &format!(
                        "Can't compile {:?} as the second argument to {}, Expected a label, offset",
                        token.token_type, low_token.token_type
                    ),
                )),
            }
        }

        Assembly8086Tokens::Number8bit(segment_num) => {
            // check if the next token is Colon
            check_token(tokenized_line, token, i + 1, &Assembly8086Tokens::Colon)?;

            let low_token =
                tokenized_line.get(i + 2, "Expected a 8bit token got None!".to_string(), None)?;

            match &low_token.token_type {
                Assembly8086Tokens::Number8bit(address_num) => Ok(Offset::SegmentedAddressing(
                    *segment_num as u16,
                    *address_num as u16,
                )),
                Assembly8086Tokens::Number16bit(address_num) => Ok(Offset::SegmentedAddressing(
                    *segment_num as u16,
                    *address_num,
                )),

                _ => Err(CompilationError::error_with_token(
                    token,
                    &format!(
                        "Can't compile {:?} as the second argument to {}, Expected a label, offset",
                        token.token_type, low_token.token_type
                    ),
                )),
            }
        }

        _ => Err(CompilationError::error_with_token(
            token,
            &format!(
                "Can't compile {:?} as the first argument to {}, Expected a label, offset",
                instruction_name, token.token_type
            ),
        )),
    }
}

pub(in crate::compiler) fn compile_single_ins_similar_as_jmp(
    i: usize,
    token: &Token,
    high_token: &Token,
    instruction_compile_data: LabeledInstructionCompileData,
    addressing_mode: Offset,
    compiled_bytes: &mut Vec<u8>,
    compiled_bytes_ref: &mut Vec<CompiledBytesReference>,
) -> usize {
    match addressing_mode {
        Offset::U8(offset) => {
            convert_and_push_instructions!(
                compiled_bytes,
                compiled_bytes_ref,
                (
                    token => instruction_compile_data.ins_8bit,
                    high_token => vec![offset]
                )
            );
            i + 1
        }
        Offset::U16(offset) => {
            convert_and_push_instructions!(
                compiled_bytes,
                compiled_bytes_ref,
                (
                    token => instruction_compile_data.ins_16bit,
                    high_token => offset.to_le_bytes().to_vec()
                )
            );
            i + 2
        }

        Offset::Pointer(offset) => {
            convert_and_push_instructions!(
                compiled_bytes,
                compiled_bytes_ref,
                (
                    token => instruction_compile_data.pointer_offset_instruction,
                    high_token => offset.to_le_bytes().to_vec()
                )
            );
            i + 2
        }
        Offset::SegmentedAddressing(segment_addr, pointer_addr) => {
            let data_ins = [pointer_addr.to_le_bytes(), segment_addr.to_le_bytes()].concat();
            let segment_ins = instruction_compile_data.segmented_indexing_instruction;
            convert_and_push_instructions!(
                compiled_bytes,
                compiled_bytes_ref,
                (
                    token => segment_ins,
                    high_token => data_ins
                )
            );
            // i + 2
            u8::MAX as usize
        }
    }
}

pub(in crate::compiler) fn parse_token_high_token_and_is_offset_defined<'a>(
    tokenized_line: &'a TokenizedLine,
    i: usize,
    variable_address_map: Option<&VariableAddressMap>,
    instruction_name: &str,
) -> Result<(usize, &'a Token, &'a Token, bool), CompilationError> {
    let mut i = i;
    let token = tokenized_line.get(
        i,
        "This shouldn't happen, Please report this".to_string(),
        None,
    )?;

    let high_token = tokenized_line.get(
        i + 1,
        format!("Expected arguments after {} got nothing", instruction_name),
        Some(vec![get_all_registers_and_variable_suggestions(
            variable_address_map,
        )]),
    )?;
    let is_offset = matches!(
        &high_token.token_type,
        Assembly8086Tokens::AssemblerDirectives(AssemblerDirectives::Offset)
    );
    if is_offset {
        i += 1;
    }

    let high_token = tokenized_line.get(
        i + 1,
        format!("Expected arguments after {} got nothing", instruction_name),
        Some(vec![get_all_registers_and_variable_suggestions(
            variable_address_map,
        )]),
    )?;

    Ok((i, token, high_token, is_offset))
}

fn get_offset_bytes_from_line_from_maps(
    label: &Label,
    line_number: LineNumber,
    is_offset: bool,
    compiled_line_ref_with_offset_maps: Option<&CompiledLineLabelRef>,
) -> Option<(u16, bool)> {
    match compiled_line_ref_with_offset_maps {
        None => None,
        Some(compiled_line_ref_with_offset_maps) => {
            match compiled_line_ref_with_offset_maps.find_label_offset(label, line_number) {
                None => {
                    if is_offset {
                        return compiled_line_ref_with_offset_maps
                            .find_var_as_label_offset(label, line_number);
                    }
                    None
                }
                Some(a) => Some(a),
            }
        }
    }
}

fn calc_offset(
    offset_bytes: u16,
    is_jmp_after_label: bool,
    small_ins_offset: u8,
    long_ins_offset: u16,
) -> Either<u8, u16> {
    // TODO: handle overflow of offset_bytes i.e line limit exceed
    if is_jmp_after_label {
        let offset = 0xFF - offset_bytes - small_ins_offset as u16;
        if offset > 0x7F && offset_bytes < 0x100 {
            Either::Left(offset as u8)
        } else {
            Either::Right(0xFFFF - offset_bytes - long_ins_offset)
        }
    } else {
        let offset = offset_bytes;
        if offset < 0x80 {
            Either::Left(offset as u8)
        } else {
            Either::Right(offset_bytes)
        }
    }
}

fn get_address(
    label: &Label,
    variable_address_map: Option<&VariableAddressMap>,
) -> Option<[u8; 2]> {
    match variable_address_map {
        None => None,
        Some(variable_address_map) => variable_address_map
            .get(label)
            .map(|(_, address)| address.to_le_bytes()),
    }
}
