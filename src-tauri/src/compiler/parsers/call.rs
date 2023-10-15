use crate::{
    compiler::{
        compilation_error::CompilationError, parsers::utils::push_instruction,
        tokenized_line::TokenizedLine, tokens::Assembly8086Tokens, types_structs::{ProcReferenceMap, LineNumber},
        CompiledBytesReference, CompiledLineLabelRef,
    },
    convert_and_push_instructions,
};

pub(in crate::compiler) fn parse_call(
    tokenized_line: &TokenizedLine,
    i: usize,
    line_number: LineNumber,
    compiled_bytes: &mut Vec<u8>,
    compiled_bytes_ref: &mut Vec<CompiledBytesReference>,
    proc_ref_map: &mut ProcReferenceMap,

    compiled_line_ref_with_offset_maps : Option<&CompiledLineLabelRef>,

) -> Result<usize, CompilationError> {
    let token = tokenized_line.get(
        i,
        "This shouldn't happen, Please report this".to_string(),
        None,
    )?;

    let high_token = tokenized_line.get(
        i + 1,
        "Expected arguments after CALL got nothing".to_string(),
        None,
    )?;

    match &high_token.token_type {
        Assembly8086Tokens::Character(label) => {
            let offset_bytes_from_line_and_is_label_before_ref = match compiled_line_ref_with_offset_maps {
                None => None,
                Some(compiled_line_ref_with_offset_maps) => compiled_line_ref_with_offset_maps.find_label_offset(label, line_number ),
            };
            let proc_offset = match  compiled_line_ref_with_offset_maps{
                None => None,
                Some(proc_ref_map) => proc_ref_map.find_proc_offset(label, line_number),
            };
            let addr = get_address_from_defined_maps(
                proc_offset,
                offset_bytes_from_line_and_is_label_before_ref,
            );

            let addr = match addr {
                Some(addr) => addr,
                None => {
                    proc_ref_map.insert(label.clone(), i + 1);
                    0_u16
                }
            };

            let ins_bytes = vec![0xE8];

            convert_and_push_instructions!(
                compiled_bytes,
                compiled_bytes_ref,
                (
                    token => ins_bytes,
                    high_token => addr.to_le_bytes().to_vec()
                )
            );

            Ok(i + 1)
        }
        _ => Err(CompilationError::error_with_token(
            high_token,
            &format!("Expected a label got {} insted", high_token.token_type),
        )),
    }
}

fn get_address_from_defined_maps(
    proc_offset: Option<i16>,
    offset_bytes_from_line_and_is_label_before_ref: Option<(u16, bool)>,
) -> Option<u16> {
    let offset_from_label = offset_bytes_from_line_and_is_label_before_ref
        .map(|(offset, is_label_before_ref)| calc_offset(offset, is_label_before_ref));

    let offset_from_proc_offset = match proc_offset {
        None => None,
        Some(offset) => {
            // check if offset is negative if it is -3 from it to manage the offset of call ins
            // i.e. call ins is 3 bytes long
            let offset = if offset < 0 { offset - 3 } else { offset };
            Some(offset as u16)
        }
    };

    match (offset_from_label, offset_from_proc_offset) {
        (None, None) => None,
        (Some(offset), None) => Some(offset),
        (None, Some(offset)) => Some(offset),
        // _ => panic!("The same label is defined in both label and proc macro this shouldn't happen check your compile function, Please report this"),
        (Some(_), Some(offset2)) => {
            // if offset != offset2 {
            //     panic!("The same label is defined in both label and proc macro this shouldn't happen check your compile function, Please report this");
            // }
            Some(offset2)
        }
    }
}

fn calc_offset(offset_bytes: u16, is_jmp_after_label: bool) -> u16 {
    // TODO: handle overflow of offset_bytes i.e line limit exceed
    if is_jmp_after_label {
        0xFFFD - offset_bytes
    } else {
        offset_bytes
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        compiler::{compile_str, tests::generate_num_ins},
        test_compile,
    };

    test_compile!(
        test_call_0x80_ins,
        &format!(
            "
    PROC main 
    ADD AX, SP 
    RET
    ENDP main 
    {}
    CALL main 
    inc ax 
    ",
            generate_num_ins(0x80)
        ),
        |instructions: &Vec<u8>| {
            // take the last three ins
            let start_idx = instructions.len() - 4;
            assert_eq!(instructions[start_idx..], [0xE8, 0x7A, 0xFF, 0x40]);
        }
    );

    test_compile!(
        test_call_0x80_ins_before_defn,
        &format!(
            "
    CALL main 
    {}
    PROC main 
    ADD AX, SP 
    RET
    ENDP main 
    inc ax 
    ",
            generate_num_ins(0x80)
        ),
        |instructions: &Vec<u8>| {
            // take the last three ins
            assert_eq!(instructions[00..3], [0xE8, 0x80, 0x00]);
        }
    );

    test_compile!(
        call_main_after_main_def,
        "
    PROC main 
        ADD AX, SP 
    RET
    ENDP main 

    CALL main 
    ; this is a commet 
    ; ahh! 
    inc ax 
    ",
        |instructions: &Vec<u8>| {
            assert_eq!(instructions, &[0x03, 0xC4, 0xC3, 0xE8, 0xFA, 0xFF, 0x40]);
        }
    );

    test_compile!(
        call_main_before_main_def,
        "
    CALL main 
    inc ax 
    mov ax, bx

    label: 

    add ax, sp

    PROC main 
    ADD AX, SP ; this is a commet
    RET
    ENDP main 
    inc ax 
    jmp label
    ",
        |instructions: &Vec<u8>| {
            assert_eq!(
                instructions,
                &[
                    0xE8, 0x05, 0x00, 0x40, 0x8B, 0xC3,
                    //
                    // Label
                    //
                    0x03, 0xC4, //
                    // proc main
                    0x03, 0xC4, 0xC3, // end main
                    0x40, 0xEb, 0xF8
                ]
            );
        }
    );
}
