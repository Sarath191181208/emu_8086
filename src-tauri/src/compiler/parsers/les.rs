use std::collections::HashMap;

use crate::compiler::{
    compilation_error::CompilationError,
    suggestions_utils::get_all_registers_and_variable_suggestions,
    tokenized_line::TokenizedLine,
    tokens::{Assembly8086Tokens, Token},
    types_structs::{CompiledBytesReference, VariableAddressMap, VariableReferenceMap},
    CompiledLineLabelRef,
};

use super::{
    pattern_extractors::{
        compile_two_arguments_patterns::parse_indexed_addr_and_reg, utils::evaluate_ins,
    },
    utils::check_comma,
};

#[allow(clippy::too_many_arguments)]
pub(crate) fn parse_les(
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
    let token = tokenized_line.get(
        i,
        "This shouldn't happen, Please report this".to_string(),
        None,
    )?;

    let high_token = tokenized_line.get(
        i + 1,
        "Expected a register got nothing!".to_string(),
        Some(vec![get_all_registers_and_variable_suggestions(Some(
            variable_abs_address_map,
        ))]),
    )?;

    check_comma(tokenized_line, high_token, i + 2)?;

    let low_token = tokenized_line.get(
        i + 3,
        "Expected a register got nothing!".to_string(),
        Some(vec![get_all_registers_and_variable_suggestions(Some(
            variable_abs_address_map,
        ))]),
    )?;

    let eval_low_token = evaluate_ins(
        i + 3,
        tokenized_line.len(),
        tokenized_line,
        is_org_defined,
        label_idx_map,
        variable_ref_map,
        variable_abs_address_map,
        compiled_line_offset_maps,
    )?;

    let low_token = if let Some(token) = &eval_low_token {
        token
    } else {
        low_token
    };

    match &high_token.token_type {
        Assembly8086Tokens::Register16bit(reg) => {
            match &low_token.token_type {
                Assembly8086Tokens::IndexedAddressing(idx_addr) => {
                    // Optimizing for the case where the offset is a constant
                    parse_indexed_addr_and_reg(
                        0xC4,
                        token,
                        high_token,
                        low_token,
                        reg.clone(),
                        idx_addr.clone(),
                        compiled_bytes,
                        compiled_bytes_ref,
                    )?;

                    Ok(tokenized_line.len())
                }
                _ => Err(CompilationError::error_with_token(
                    low_token,
                    &format!(
                        "Expected a 16 bit memory Address got {}",
                        low_token.token_type
                    ),
                )),
            }
        }

        _ => Err(CompilationError::error_with_token(
            high_token,
            &format!("Expected a 16 bit register got {}", high_token.token_type),
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
