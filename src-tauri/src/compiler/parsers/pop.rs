use std::collections::HashMap;

use crate::compiler::{
    compilation_error::CompilationError,
    tokenized_line::TokenizedLine,
    tokens::Token,
    types_structs::{CompiledBytesReference, VariableAddressMap, VariableReferenceMap},
    CompiledLineLabelRef,
};

use super::{utils::THIS_SHOULDNT_HAPPEN, pattern_extractors::utils::evaluate_ins};

#[allow(clippy::too_many_arguments)]
pub(in crate::compiler) fn parse_pop(
    tokenized_line: &TokenizedLine,
    i: usize,
    is_org_defined: bool,
    compiled_bytes: &mut Vec<u8>,
    compiled_bytes_ref: &mut Vec<CompiledBytesReference>,
    var_ref_map: &mut VariableReferenceMap,
    variable_address_map: Option<&VariableAddressMap>,
    label_idx_map: &mut HashMap<String, (Token, usize, bool)>,
    compiled_line_ref_with_offset_maps: Option<&CompiledLineLabelRef>,
) -> Result<usize, CompilationError> {
    let token = tokenized_line.get(i, THIS_SHOULDNT_HAPPEN.to_string(), None)?;
    let high_token = tokenized_line.get(
        i + 1,
        "Expected a Register (or) Segment (or) Address got none".to_string(),
        None,
    )?;

    let evaluated_token = evaluate_ins(
        i + 1,
        tokenized_line.tokens.len(),
        tokenized_line,
        is_org_defined,
        label_idx_map,
        var_ref_map,
        variable_address_map.unwrap_or(&VariableAddressMap::new()),
        compiled_line_ref_with_offset_maps,
    )?;

    let high_token = match &evaluated_token {
        Some(token) => token,
        None => high_token,
    };

    match high_token.token_type {
        _ => {
            return Err(CompilationError::error_with_token(
                high_token,
                &format!(
                    "Expected a Register (or) Segment (or) Address got {:?}",
                    high_token.token_type
                ),
            ))
        }
    }
}
