/// This if for instructions fowlling this pattern
/// ```asm
///     INS OFFSET LABEL
///     INS LABEL
/// ```
use std::collections::HashMap;

use crate::{
    compiler::{
        compilation_error::CompilationError,
        suggestions_utils::get_all_registers_and_variable_suggestions,
        tokenized_line::TokenizedLine,
        tokens::{assembler_directives::AssemblerDirectives, Assembly8086Tokens, Token},
        types_structs::{Label, LineNumber, VariableAddressMap, CompiledBytesReference},
        CompiledLineLabelRef, parsers::utils::push_instruction,
    },
    utils::Either, convert_and_push_instructions,
};

pub(in crate::compiler) enum Offset {
    U8(u8),
    U16(u16),
    Pointer(u16),
}

pub(in crate::compiler) struct OffsetMaps<'a> {
    pub label_idx_map: &'a mut HashMap<String, (Token, usize, bool)>,
    pub compiled_line_ref_with_offset_maps: Option<&'a CompiledLineLabelRef<'a>>,
    pub variable_address_map: Option<&'a VariableAddressMap>,
}

pub(in crate::compiler) struct OffsetInstructionCompileData {
    pub pointer_offset_instruciton: Vec<u8>,
    pub ins_8bit: Vec<u8>,
    pub ins_16bit: Vec<u8>,
    pub bytes_of_8bit_ins: u8,
    pub bytes_of_16bit_ins: u16,
    pub is_offset: bool,
}

pub(in crate::compiler) fn parse_single_ins_labels(
    line_number: LineNumber,
    instruction_name: &str,
    token: &Token,
    instruction_compile_data: &OffsetInstructionCompileData,
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

        _ => Err(CompilationError::error_with_token(
            token,
            &format!(
                "Can't compile {:?} as the first argument to {}, Expected a label, offset",
                instruction_name, token.token_type
            ),
        )),
    }
}

pub(in crate::compiler) fn match_ins_to_bytes_single_ins_with_label_and_offset_label(
    i: usize,
    token: &Token,
    high_token: &Token,
    instruction_compile_data: OffsetInstructionCompileData,
    addressing_mode: Offset,
    compiled_bytes: &mut Vec<u8>,
compiled_bytes_ref: &mut Vec<CompiledBytesReference>,
) -> usize{
    match addressing_mode{
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
        Offset::Pointer(addr) => {
            convert_and_push_instructions!(
            compiled_bytes,
            compiled_bytes_ref,
            (
                token => instruction_compile_data.pointer_offset_instruciton,
                high_token => addr.to_le_bytes().to_vec()
            )
            );
            i + 2
        }
    }

}


pub(in crate::compiler) fn parse_token_high_token_and_is_offset_defined<'a>(
    tokenized_line: &'a TokenizedLine,
    i: usize,
    variable_address_map: Option<&VariableAddressMap>,
) -> Result<(usize, &'a Token, &'a Token, bool), CompilationError> {
    let mut i = i;
    let token = tokenized_line.get(
        i,
        "This shouldn't happen, Please report this".to_string(),
        None,
    )?;

    let high_token = tokenized_line.get(
        i + 1,
        "Expected arguments after JMP got nothing".to_string(),
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
        "Expected arguments after JMP got nothing".to_string(),
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
