use std::collections::HashMap;

use crate::compiler::{
    compilation_error::CompilationError,
    tokenized_line::TokenizedLine,
    tokens::{Assembly8086Tokens, Token},
    CompiledBytes,
};

use super::utils::push_instruction;

enum Offset {
    U8(u8),
    U16(u16),
}

pub(in crate::compiler) fn parse_jmp(
    tokenized_line: &TokenizedLine,
    i: usize,
    compiled_bytes: &mut Vec<u8>,
    compiled_bytes_ref: &mut Vec<CompiledBytes>,
    label_idx_map: &mut HashMap<String, (Token, u16)>,
    offset_bytes_from_line_and_is_label_before_ref: Option<(u16, bool)>,
) -> Result<usize, CompilationError> {
    let token = tokenized_line.get(i, "This shouldn't happen, Please report this".to_string())?;
    let high_token = tokenized_line.get(
        i + 1,
        "Expected arguments after JMP got nothing".to_string(),
    )?;
    match &high_token.token_type {
        Assembly8086Tokens::Character(label) => {
            match offset_bytes_from_line_and_is_label_before_ref {
                None => {
                    // placeholder instruction
                    push_instruction(compiled_bytes, vec![0xEB], token, compiled_bytes_ref);
                    push_instruction(compiled_bytes, vec![0x00], high_token, compiled_bytes_ref);
                    let ins_idx = (compiled_bytes.len() - 1) as u16;
                    label_idx_map.insert(label.to_string(), (high_token.clone(), ins_idx));
                    Ok(i + 2)
                }
                Some((offset_bytes, is_jmp_after_label)) => {
                    match calc_offset(offset_bytes,is_jmp_after_label) {
                        Offset::U8(offset) => {
                            push_instruction(compiled_bytes, vec![0xEB], token, compiled_bytes_ref);
                            push_instruction(
                                compiled_bytes,
                                vec![offset],
                                high_token,
                                compiled_bytes_ref,
                            );
                            Ok(i + 2)
                        }
                        Offset::U16(offset) => {
                            push_instruction(compiled_bytes, vec![0xE9], token, compiled_bytes_ref);
                            push_instruction(
                                compiled_bytes,
                                vec![(offset & 0xFF) as u8, (offset >> 8) as u8],
                                high_token,
                                compiled_bytes_ref,
                            );
                            Ok(i + 3)
                        }
                    }
                }
            }
        }

        _ => Err(CompilationError::new(
            high_token.line_number,
            high_token.column_number,
            high_token.token_length,
            &format!(
                "Can't compile {:?} as the first argument to JMP, Expected a label, offset",
                high_token.token_type
            ),
        )),
    }
}

fn calc_offset(offset_bytes: u16, is_jmp_after_label: bool) -> Offset {
    // TODO: handle overflow of offset_bytes i.e line limit exceed
    if is_jmp_after_label {
        let offset = 0xFF - offset_bytes - 1;
        if offset > 0x7F && offset_bytes < 0x100 {
            Offset::U8(offset as u8)
        } else {
            Offset::U16(0xFFFF - offset_bytes - 1)
        }
    } else {
        let offset = offset_bytes;
        if offset < 0x80 {
            Offset::U8(offset as u8)
        } else {
            Offset::U16(offset_bytes)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{compiler::compile_str, test_compile};

    test_compile!(
        test_jmp_label,
        "MOV BX, CX\nlabel1: \nSUB CX, AX \n\nJMP label1",
        |compiled_instructions: &Vec<u8>| {
            assert_eq!(compiled_instructions, &[139, 217, 43, 200, 235, 252]);
        }
    );

    test_compile!(
        test_jmp_label_back,
        "JMP label

MOV AX, BX
MOV BX, CX

label:
    MOV AX, BX
    MOV CX, DX",
        |compiled_instructions: &Vec<u8>| {
            assert_eq!(compiled_instructions, &[0xEB, 0x04, 0x8B, 0xC3, 0x8B, 0xD9, 0x8B, 0xC3, 0x8B, 0xCA]);
        }
    );
}
