use std::collections::HashMap;

use crate::compiler::{
    compilation_error::CompilationError,
    tokenized_line::TokenizedLine,
    tokens::Token,
    types_structs::{LineNumber, VariableAddressMap},
    CompiledBytesReference, CompiledLineLabelRef,
};

use super::pattern_extractors::offset_label_pattern::{
    parse_labeled_relative_offset, LabeledInstructionCompileData,
};

#[allow(clippy::too_many_arguments)]
pub(in crate::compiler) fn parse_jmp(
    tokenized_line: &TokenizedLine,
    i: usize,
    line_number: LineNumber,
    compiled_bytes: &mut Vec<u8>,
    compiled_bytes_ref: &mut Vec<CompiledBytesReference>,
    variable_address_map: Option<&VariableAddressMap>,
    label_idx_map: &mut HashMap<String, (Token, usize, bool)>,
    compiled_line_ref_with_offset_maps: Option<&CompiledLineLabelRef>,
) -> Result<usize, CompilationError> {
    let mut instruction_compile_data = LabeledInstructionCompileData {
        pointer_offset_instruction: vec![0xFF, 0x26],
        ins_8bit: vec![0xEB],
        ins_16bit: vec![0xE9],
        bytes_of_8bit_ins: 1,
        bytes_of_16bit_ins: 2,
        is_offset: false,
        segmented_indexing_instruction: vec![0xEA],
    };

    parse_labeled_relative_offset(
        tokenized_line,
        i,
        line_number,
        "JMP",
        compiled_bytes,
        compiled_bytes_ref,
        variable_address_map,
        label_idx_map,
        compiled_line_ref_with_offset_maps,
        &mut instruction_compile_data,
    )
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
    use crate::{compile_and_compare_ins, compiler::compile_str, test_compile};
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

    test_compile!(
        jmp_offset_var16bit_with_org,
        "
        org 100h
        .data  
        var dw 0x1000 
        code:            
        jmp var  
        jmp var2         
        
        mov ax, bx + 0x100
           
        var2 dw 0x200
        ",
        |instructions: &Vec<u8>| {
            assert_eq!(
                instructions,
                &[
                    0xEB, 0x02, // org
                    0x00, 0x10, // Var
                    0xFF, 0x26, 0x02, 0x01, // JMP
                    0xFF, 0x26, 0x10, 0x01, // JMP
                    0x8B, 0x87, 0x00, 0x01, // MOV
                    0x00, 0x02, // Var2
                ]
            );
        }
    );

    test_compile!(
        jmp_offset_var16bit,
        "
        Var dw 0x10
        JMP offset var
        ",
        |instructions: &Vec<u8>| {
            assert_eq!(instructions, &[0x10, 0x00, 0xEB, 0xFC]);
        }
    );

    test_compile!(
        jmp_offset_var8bit,
        "
        Var db 0x10
        JMP offset var
        ",
        |instructions: &Vec<u8>| {
            assert_eq!(instructions, &[0x10, 0xEB, 0xFD]);
        }
    );

    compile_and_compare_ins!(
        jmp_segmented_indexing,
        "
        JMP 0x200:0x100
        ",
        &[0xEA, 0x00, 0x01, 0x00, 0x02]
    );
}
