use std::collections::HashMap;

use crate::compiler::{
    compilation_error::CompilationError, tokenized_line::TokenizedLine, tokens::Token,
    types_structs::LineNumber, CompiledBytesReference, CompiledLineLabelRef,
};

use super::pattern_extractors::offset_label_pattern::parse_label_pattern_full;

#[allow(clippy::too_many_arguments)]
pub(in crate::compiler) fn parse_ja(
    tokenized_line: &TokenizedLine,
    i: usize,
    line_number: LineNumber,
    compiled_bytes: &mut Vec<u8>,
    compiled_bytes_ref: &mut Vec<CompiledBytesReference>,
    label_idx_map: &mut HashMap<String, (Token, usize, bool)>,
    compiled_line_ref_with_offset_maps: Option<&CompiledLineLabelRef>,
) -> Result<usize, CompilationError> {
    let ins_8bit = vec![0x77];
    let ins_16bit = vec![0x76, 0x03, 0xE9]; // JNBE ins

    parse_label_pattern_full(
        "JA",
        ins_8bit,
        ins_16bit,
        tokenized_line,
        i,
        line_number,
        compiled_bytes,
        compiled_bytes_ref,
        label_idx_map,
        compiled_line_ref_with_offset_maps,
    )
}

#[cfg(test)]
mod tests {
    use crate::{compile_and_compare_ins, compiler::compile_str, test_compile};
    use pretty_assertions::assert_eq;

    fn get_compiled_bytes() -> Vec<u8> {
        let starting_bytes = [0x76, 0x03, 0xE9, 0x80, 0x00];

        let mid_bytes_repeat = vec![0xC7, 0x06, 0x00, 0x01, 0x00, 0x01].repeat(21);

        let after_bytes = [
            0x77, 0x00, 0x77, 0xFE, 0x77, 0x0E, 0x76, 0x03, 0xE9, 0x72, 0xFF, 0x77, 0xFE,
        ];

        let mut compiled_bytes = starting_bytes.to_vec();
        compiled_bytes.extend(mid_bytes_repeat);
        compiled_bytes.extend(after_bytes);
        compiled_bytes
    }

    compile_and_compare_ins!(
        test_ja,
        "
        org 100h

        start:
        ja label 

        mov [0x100], 0x100 
        mov [0x100], 0x100 
        mov [0x100], 0x100 
        mov [0x100], 0x100 
        mov [0x100], 0x100 
        mov [0x100], 0x100  
        mov [0x100], 0x100 
        mov [0x100], 0x100 
        mov [0x100], 0x100 
        mov [0x100], 0x100 
        mov [0x100], 0x100 
        mov [0x100], 0x100 
        mov [0x100], 0x100 
        mov [0x100], 0x100 
        mov [0x100], 0x100 
        mov [0x100], 0x100 
        mov [0x100], 0x100 
        mov [0x100], 0x100           
        mov [0x100], 0x100 
        mov [0x100], 0x100 
        mov [0x100], 0x100  

        ja label

        label:  

        ja label
         
        ja 0x10   
        ja start
        ja 0x00 
        ",
        get_compiled_bytes()
    );
}
