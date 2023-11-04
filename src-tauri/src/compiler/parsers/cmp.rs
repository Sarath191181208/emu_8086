use crate::compiler::{
    compilation_error::CompilationError, tokenized_line::TokenizedLine, CompiledBytesReference,
};

use super::pattern_extractors::{
    compile_tow_args_whole_ins::{compile_two_args_whole_ins, CompilingBytesForInstruction},
    AddressingMode,
};

pub(in crate::compiler) fn parse_cmp(
    tokenized_line: &TokenizedLine,
    i: usize,
    compiled_bytes: &mut Vec<u8>,
    compiled_bytes_ref: &mut Vec<CompiledBytesReference>,
    addressing_mode: AddressingMode,
) -> Result<usize, CompilationError> {
    let ins = CompilingBytesForInstruction {
        reg_16bit_and_anything_ins: 0x3B,
        reg_8bit_and_anything_ins: 0x3A,
        indexed_addressing_and_anyting_ins: 0x39,
        addr_and_8bit_reg: 0x38,

        al_and_num_ins: Some(0x3C),
        ax_and_num_ins: Some(0x3D),
        reg16bit_and_16bit_num: 0x81,
        reg16bit_and_8bit_num: Some(0x83),
        reg8bit_and_num: 0x80,
        reg_num_sub_ins: 0xF8,

        addr16bit_and_16bit_num: 0x81,
        addr16bit_and_8bit_num: Some(0x83),
        addr8bit_and_num: 0x80,
        addr_num_sub_ins: 0x3E,
    };

    compile_two_args_whole_ins(
        tokenized_line,
        i,
        ins,
        compiled_bytes,
        compiled_bytes_ref,
        addressing_mode,
    )
}

#[cfg(test)]
mod compile_cmp_tests {

    use crate::{compile_and_compare_ins, compiler::compile_str, test_compile};
    use pretty_assertions::assert_eq;

    compile_and_compare_ins!(
        test_or_reg16bit_and_anything,
        "
        CMP ax, bx
        CMP bl, cl

        CMP b.[0x100], al
        CMP w.[0x100], ax

        CMP AX, 0x100
        CMP BX, 0x10
        CMP CX, 0x100
        CMP al, 0x10    
        CMP BL, 0x10

        CMP b.[0x100], 0x10 
        CMP [0x100], 0x100

        CMP b.[0x100], 0x10
        ",
        vec![
            0x3B, 0xC3, 0x3A, 0xD9, 0x38, 0x06, 0x00, 0x01, 0x39, 0x06, 0x00, 0x01, 0x3D, 0x00,
            0x01, 0x83, 0xFB, 0x10, 0x81, 0xF9, 0x00, 0x01, 0x3C, 0x10, 0x80, 0xFB, 0x10, 0x80,
            0x3E, 0x00, 0x01, 0x10, 0x81, 0x3E, 0x00, 0x01, 0x00, 0x01, 0x80, 0x3E, 0x00, 0x01,
            0x10
        ]
    );
}
