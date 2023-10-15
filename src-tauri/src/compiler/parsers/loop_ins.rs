use std::collections::HashMap;

use crate::{
    compiler::{
        compilation_error::CompilationError,
        tokenized_line::TokenizedLine,
        tokens::{Assembly8086Tokens, Token},
        CompiledBytesReference, types_structs::LineNumber, CompiledLineLabelRef,
    },
    convert_and_push_instructions,
    utils::Either,
};

use super::utils::push_instruction;

pub(in crate::compiler) fn parse_loop(
    tokenized_line: &TokenizedLine,
    i: usize,
    line_number: LineNumber,
    compiled_bytes: &mut Vec<u8>,
    compiled_bytes_ref: &mut Vec<CompiledBytesReference>,
    label_idx_map: &mut HashMap<String, (Token, usize)>,
    compiled_line_ref_with_offset_maps : Option<&CompiledLineLabelRef>,

) -> Result<usize, CompilationError> {
    let token = tokenized_line.get(
        i,
        "This shouldn't happen, Please report this".to_string(),
        None,
    )?;
    let label_token = tokenized_line.get(
        i + 1,
        "Expected label after Loop got nothing".to_string(),
        // Some(vec![get_all_labels_suggestions(&label_idx_map)]),
        None,
    )?;

    match &label_token.token_type {
        Assembly8086Tokens::Character(label) => {
            let offset_num_bytes = match compiled_line_ref_with_offset_maps {
                None => None,
                Some(compiled_line_ref_with_offset_maps) => compiled_line_ref_with_offset_maps.find_label_offset(label, line_number ),
            };
            match calc_offset(offset_num_bytes) {
                Some(Either::Right(num)) => {
                    // E9 Offset
                    convert_and_push_instructions!(
                        compiled_bytes,
                        compiled_bytes_ref,
                        (
                            // here 0x49 -> dec cx
                            // 0xE3 0x03 -> jcxz 0x03
                            // 0xE9 -> jmp
                            token => vec![0x49, 0xE3, 0x03, 0xE9],
                            // 0x00 0x00 -> 0x0000
                            label_token => num.to_le_bytes().to_vec()
                        )
                    );
                    Ok(i + 1)
                }
                Some(Either::Left(num)) => {
                    convert_and_push_instructions!(
                        compiled_bytes,
                        compiled_bytes_ref,
                        (
                            token => vec![0xE2],
                            label_token => vec![num]
                        )
                    );
                    Ok(i + 1)
                }
                None => {
                    convert_and_push_instructions!(
                        compiled_bytes,
                        compiled_bytes_ref,
                        (
                            token => vec![0xE2],
                            label_token => vec![0x00]
                        )
                    );
                    // this represents that this variable is used and needs to be defined
                    label_idx_map.insert(label.to_string(), (label_token.clone(), i + 1));
                    Ok(i + 1)
                }
            }
        }
        _ => Err(CompilationError::error_with_token(
            token,
            &format!("Expected label after Loop got {:?}", label_token),
        )),
    }
}

fn calc_offset(offset_num_bytes: Option<(u16, bool)>) -> Option<Either<u8, u16>> {
    match offset_num_bytes {
        None => None,
        Some((offset, true)) => {
            let offset = 0xFF - offset - 1;
            if offset > 0x7F && offset < 0x100 {
                Some(Either::Left(offset as u8))
            } else {
                Some(Either::Right(0xFFFF - offset - 7)) // 6 is the length of the instruction
            }
        }
        Some((offset, false)) => {
            if offset < 0x80 {
                Some(Either::Left(offset as u8))
            } else {
                Some(Either::Right(offset))
            }
        }
    }
}

#[cfg(test)]
mod test_variable_declaration {
    use crate::{
        compiler::{compile_str, tests::generate_num_ins},
        test_compile,
    };

    test_compile!(
        loop_var_1,
        "
        label1: 
        inc ax
        loop label1
        ",
        |instructions: &Vec<u8>| {
            assert_eq!(instructions, &vec![0x40, 0xE2, 0xFD]);
        }
    );

    test_compile!(
        loop_0x80_var_1,
        &format!(
            "
        mov cx, 0x02
        label1:
        {}
        loop label1
        ",
            generate_num_ins(0x80)
        ),
        |instructions: &Vec<u8>| {
            assert_eq!(
                instructions[instructions.len() - 6..],
                vec![0x49, 0xE3, 0x03, 0xE9, 0x7A, 0xFF]
            );
        }
    );
}
