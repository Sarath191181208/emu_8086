use std::collections::HashMap;

use crate::compiler::{
    compilation_error::CompilationError,
    tokenized_line::TokenizedLine,
    tokens::{Assembly8086Tokens, Token},
    CompiledBytes,
};

use super::utils::push_instruction;

pub(in crate::compiler) fn parse_jmp(
    tokenized_line: &TokenizedLine,
    i: usize,
    compiled_bytes: &mut Vec<u8>,
    compiled_bytes_ref: &mut Vec<CompiledBytes>,
    label_idx_map: &mut HashMap<String, (Token, u16)>,
) -> Result<usize, CompilationError> {
    let token = tokenized_line.get(i, "This shouldn't happen, Please report this".to_string())?;
    let high_token = tokenized_line.get(
        i + 1,
        "Expected arguments after JMP got nothing".to_string(),
    )?;
    match &high_token.token_type {
        Assembly8086Tokens::Character(label) => {
            push_instruction(compiled_bytes, vec![0xEB], token, compiled_bytes_ref);
            push_instruction(compiled_bytes, vec![0x00], high_token, compiled_bytes_ref);
            let ins_idx = (compiled_bytes.len() - 1) as u16;
            label_idx_map.insert(label.to_string(), (high_token.clone(), ins_idx));
            Ok(i + 2)
        }

        _ => Err(CompilationError::new(
            high_token.line_number,
            high_token.column_number,
            high_token.token_length,
            &format!(
                "Can't compile {:?} as the first argument to DEC, Expected a register",
                high_token.token_type
            ),
        )),
    }
}