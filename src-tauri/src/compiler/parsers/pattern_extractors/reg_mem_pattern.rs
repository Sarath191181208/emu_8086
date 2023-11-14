use std::collections::HashMap;

use crate::{
    compiler::{
        compilation_error::CompilationError,
        parsers::utils::{get_evaluated_token, push_instruction},
        tokenized_line::TokenizedLine,
        tokens::{Assembly8086Tokens, Token},
        types_structs::{CompiledBytesReference, VariableAddressMap, VariableReferenceMap},
        CompiledLineLabelRef,
    },
    convert_and_push_instructions,
};

pub(crate) struct ParseRegMemFnArgMaps<'a> {
    pub label_idx_map: &'a mut HashMap<String, (Token, usize, bool)>,
    pub variable_ref_map: &'a mut VariableReferenceMap,
    pub variable_abs_address_map: &'a VariableAddressMap,
    pub compiled_line_offset_maps: Option<&'a CompiledLineLabelRef<'a>>,
}

pub(crate) struct CompilationData {
    pub ins_16bit: u8,
    pub ins_8bit: u8,
    // This is the offset after the reg push_instruction
    // For `DIV BX`, DIV = 0xF7, BX = 3 ( reg_idx )
    // Normally BX is compiled to 0xC0 + reg_idx
    // But it's compiled to 0xF7, 0xF0 + reg_idx
    // ex: [0xF7, 0xC0 + 0x30 + reg_idx]
    //                   ^^^^ -> sub_idx_offset
    //  This logic also applies for 8bit registers and memory
    pub sub_idx_offset: u8,
}

pub(crate) fn parse_reg_mem_pattern_line(
    i: usize,
    tokenized_line: &TokenizedLine,
    is_org_defined: bool,
    arg_maps: ParseRegMemFnArgMaps,
    compiled_bytes: &mut Vec<u8>,
    compiled_bytes_ref: &mut Vec<CompiledBytesReference>,
    compilation_data: CompilationData,
) -> Result<usize, CompilationError> {
    let ParseRegMemFnArgMaps {
        label_idx_map,
        variable_ref_map,
        variable_abs_address_map,
        compiled_line_offset_maps,
    } = arg_maps;

    let CompilationData {
        ins_16bit,
        ins_8bit,
        sub_idx_offset,
    } = compilation_data;

    let (token, high_token) = get_evaluated_token(
        i,
        tokenized_line,
        is_org_defined,
        label_idx_map,
        variable_ref_map,
        variable_abs_address_map,
        compiled_line_offset_maps,
    )?;

    match &high_token.token_type {
        Assembly8086Tokens::Register16bit(reg) => {
            let reg_idx = reg.get_index_or_err(&high_token)? % 8;
            convert_and_push_instructions!(
                compiled_bytes,
                compiled_bytes_ref,
                (
                    token => vec![ins_16bit],
                    &high_token => vec![0xC0 + sub_idx_offset + reg_idx]
                )
            );
            Ok(i + 2)
        }

        Assembly8086Tokens::IndexedAddressing(indexed_addr) => {
            let (mut ins, mut addr_bytes) = indexed_addr.get_ins_and_and_offset();
            ins += sub_idx_offset;
            addr_bytes.insert(0, ins);
            convert_and_push_instructions!(
                compiled_bytes,
                compiled_bytes_ref,
                (
                    token => vec![ins_16bit],
                    &high_token => addr_bytes
                )
            );

            Ok(tokenized_line.len())
        }

        Assembly8086Tokens::Register8bit(reg) => {
            let reg_idx = reg.get_as_idx();
            convert_and_push_instructions!(
                compiled_bytes,
                compiled_bytes_ref,
                (
                    token => vec![ins_8bit],
                    &high_token => vec![0xC0 + sub_idx_offset + reg_idx]
                )
            );
            Ok(i + 2)
        }

        Assembly8086Tokens::ByteIndexedAddressing(indexed_addressing) => {
            let (mut ins, mut addr_bytes) = indexed_addressing.get_ins_and_and_offset();
            ins += sub_idx_offset;
            addr_bytes.insert(0, ins);
            convert_and_push_instructions!(
                compiled_bytes,
                compiled_bytes_ref,
                (
                    token => vec![ins_8bit],
                    &high_token => addr_bytes
                )
            );

            Ok(tokenized_line.len())
        }

        _ => Err(CompilationError::error_with_token(
            &high_token,
            &format!("Expected a register/memory got {}", high_token.token_type),
        )),
    }
}
