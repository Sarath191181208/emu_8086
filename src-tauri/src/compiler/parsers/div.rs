use std::collections::HashMap;

use crate::{
    compiler::{
        compilation_error::CompilationError,
        parsers::utils::push_instruction,
        tokenized_line::TokenizedLine,
        tokens::{Assembly8086Tokens, Token},
        types_structs::{CompiledBytesReference, VariableAddressMap, VariableReferenceMap},
        CompiledLineLabelRef,
    },
    convert_and_push_instructions,
};

use super::utils::get_evaluated_token;

#[allow(clippy::too_many_arguments)]
pub(crate) fn parse_div(
    i: usize,
    tokenized_line: &TokenizedLine,
    is_org_defined: bool,
    label_idx_map: &mut HashMap<String, (Token, usize, bool)>,
    variable_ref_map: &mut VariableReferenceMap,
    variable_abs_address_map: &VariableAddressMap,
    compiled_line_offset_maps: Option<&CompiledLineLabelRef>,
    compiled_bytes: &mut Vec<u8>,
    compiled_bytes_ref: &mut Vec<CompiledBytesReference>,
) -> Result<usize, CompilationError> {
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
                    token => vec![0xF7],
                    &high_token => vec![0xF0 + reg_idx]
                )
            );
            Ok(i + 2)
        }

        Assembly8086Tokens::IndexedAddressing(indexed_addr) => {
            let (mut ins, mut addr_bytes) = indexed_addr.get_ins_and_and_offset();
            ins += 0x30;
            addr_bytes.insert(0, ins);
            convert_and_push_instructions!(
                compiled_bytes,
                compiled_bytes_ref,
                (
                    token => vec![0xF7],
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
                    token => vec![0xF6],
                    &high_token => vec![0xF0 + reg_idx]
                )
            );
            Ok(i + 2)
        }

        Assembly8086Tokens::ByteIndexedAddressing(indexed_addressing) => {
            let (mut ins, mut addr_bytes) = indexed_addressing.get_ins_and_and_offset();
            ins += 0x30;
            addr_bytes.insert(0, ins);
            convert_and_push_instructions!(
                compiled_bytes,
                compiled_bytes_ref,
                (
                    token => vec![0xF6],
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

#[cfg(test)]
mod les_compilation_tests {
    use crate::{compile_and_compare_ins, compiler::compile_str, test_compile};
    use pretty_assertions::assert_eq;

    compile_and_compare_ins!(
        les_1,
        "
        org 100h 
        .data 
        var dw 0x101
        code: 
        les ax, [bx+si+0x1234]
        les cx, [si+0x10]
        les si, [0x100]
        les di, var
        ",
        vec![
            0xEB, 0x02, 0x01, 0x01, 0xC4, 0x80, 0x34, 0x12, 0xC4, 0x4c, 0x10, 0xC4, 0x36, 0x00,
            0x01, 0xc4, 0x3E, 0x02, 0x01
        ]
    );
}
