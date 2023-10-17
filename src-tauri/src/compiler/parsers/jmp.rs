use std::collections::HashMap;

use crate::compiler::{
    compilation_error::CompilationError,
    suggestions_utils::get_all_registers_and_variable_suggestions,
    tokenized_line::TokenizedLine,
    tokens::{Assembly8086Tokens, Token},
    types_structs::{Label, LineNumber, VariableAddressMap},
    CompiledBytesReference, CompiledLineLabelRef,
};

use super::utils::push_instruction;

enum Offset {
    U8(u8),
    U16(u16),
}

#[allow(clippy::too_many_arguments)]
pub(in crate::compiler) fn parse_jmp(
    tokenized_line: &TokenizedLine,
    i: usize,
    line_number: LineNumber,
    compiled_bytes: &mut Vec<u8>,
    compiled_bytes_ref: &mut Vec<CompiledBytesReference>,
    variable_address_map: Option<&VariableAddressMap>,
    label_idx_map: &mut HashMap<String, (Token, usize)>,
    compiled_line_ref_with_offset_maps: Option<&CompiledLineLabelRef>,
) -> Result<usize, CompilationError> {
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
    match &high_token.token_type {
        Assembly8086Tokens::Character(label) => {
            // let offset_bytes_from_line_and_is_label_before_ref = compiled_line_ref_with_offset_maps.find_label_offset(label, line_number );

            let offset_bytes_from_line_and_is_label_before_ref =
                match compiled_line_ref_with_offset_maps {
                    None => None,
                    Some(compiled_line_ref_with_offset_maps) => {
                        compiled_line_ref_with_offset_maps.find_label_offset(label, line_number)
                    }
                };

            match offset_bytes_from_line_and_is_label_before_ref {
                None => {
                    // try to get the bytes from the variable_address_map
                    let address = get_address(label, variable_address_map);
                    let placeholder_or_variable_val = match address {
                        None => vec![0x00],
                        Some(address) => address.to_vec(),
                    };
                    let placeholder_or_variable_ins = match address {
                        None => vec![0xEB],
                        Some(_) => vec![0xFF, 0x26],
                    };
                    let placeholder_or_variable_val_len =
                        placeholder_or_variable_val.len() + 1 - placeholder_or_variable_ins.len();
                    push_instruction(
                        compiled_bytes,
                        placeholder_or_variable_ins,
                        token,
                        compiled_bytes_ref,
                    );
                    push_instruction(
                        compiled_bytes,
                        placeholder_or_variable_val,
                        high_token,
                        compiled_bytes_ref,
                    );
                    let ins_idx = compiled_bytes.len() - placeholder_or_variable_val_len;
                    label_idx_map.insert(label.to_string(), (high_token.clone(), ins_idx));
                    Ok(i + 2)
                }
                Some((offset_bytes, is_jmp_after_label)) => {
                    match calc_offset(offset_bytes, is_jmp_after_label) {
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

        _ => Err(CompilationError::new_without_suggestions(
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
            Offset::U16(0xFFFF - offset_bytes - 2)
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

fn get_address(
    label: &Label,
    variable_address_map: Option<&VariableAddressMap>,
) -> Option<[u8; 2]> {
    match variable_address_map {
        None => None,
        Some(variable_address_map) => variable_address_map
            .get(label)
            .map(|(_, address)| [(address & 0xFF) as u8, (address >> 8) as u8]),
    }
}

#[cfg(test)]
mod tests {
    use crate::{compiler::compile_str, test_compile};
    fn generate_inc_ins(size: u16) -> String {
        let mut ins = String::new();
        for _ in 0..size {
            ins.push_str("INC AX\n");
        }
        ins
    }

    test_compile!(
        test_jmp_label,
        "MOV BX, CX
        label1: 
        SUB CX, AX 
        JMP label1
        ",
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
            assert_eq!(
                compiled_instructions,
                &[0xEB, 0x04, 0x8B, 0xC3, 0x8B, 0xD9, 0x8B, 0xC3, 0x8B, 0xCA]
            );
        }
    );

    test_compile!(
        test_jmp_x80_bit_positive,
        &format!(
            "
            label:
                {}
            jmp label
            ",
            generate_inc_ins(0x7E)
        ),
        |compiled_instructions: &Vec<u8>| {
            let len_ins = compiled_instructions.len();
            let last_ins = compiled_instructions[len_ins - 1];
            // let before_last_ins = compiled_instructions[len_ins - 2];
            // assert_eq!(last_ins, 0xFF);
            assert_eq!(last_ins, 0x80)
        }
    );

    test_compile!(
        test_jmp_x7f_bit_negative,
        &format!(
            "
            jmp label
                {}
            label:
            ",
            generate_inc_ins(0x7F)
        ),
        |compiled_instructions: &Vec<u8>| {
            let ins = compiled_instructions[1];
            // let before_last_ins = compiled_instructions[len_ins - 2];
            // assert_eq!(last_ins, 0xFF);
            assert_eq!(ins, 0x7F)
        }
    );

    test_compile!(
        test_jmp_var,
        "
        org 100h
        .data 
            var1 dw 0x1000
        code:
        jmp var1
        ",
        |compiled_instructions: &Vec<u8>| {
            assert_eq!(
                compiled_instructions,
                &[0xEB, 0x02, 0x00, 0x10, 0xFF, 0x26, 0x02, 0x01]
            );
        }
    );
}

#[cfg(test)]
mod test_16_bit_jmp_compile {
    use crate::{compiler::compile_str, test_compile};
    fn generate_inc_ins(size: u16) -> String {
        let mut ins = String::new();
        for _ in 0..size {
            ins.push_str("INC AX\n");
        }
        ins
    }

    test_compile!(
        test_jmp_x80_bit_negative,
        &format!(
            "
            jmp label
                {}
            label:
            ",
            generate_inc_ins(0x80)
        ),
        |compiled_instructions: &Vec<u8>| {
            let low_bits = compiled_instructions[1];
            let high_bits = compiled_instructions[2];
            assert_eq!(low_bits, 0x80);
            assert_eq!(high_bits, 0x00);
        }
    );

    test_compile!(
        test_jmp_x81_bit_positive,
        &format!(
            "
            label:
                {}
            JMP label
            ",
            generate_inc_ins(0x7F)
        ),
        |compiled_instructions: &Vec<u8>| {
            let len_ins = compiled_instructions.len();
            let low_bits = compiled_instructions[len_ins - 2];
            let high_bits = compiled_instructions[len_ins - 1];
            assert_eq!(low_bits, 0x7E);
            assert_eq!(high_bits, 0xFF);
        }
    );
}
