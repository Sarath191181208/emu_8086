use crate::compiler::{
    compilation_error::CompilationError, tokenized_line::TokenizedLine, CompiledBytesReference,
};

use super::pattern_extractors::{
    compile_tow_args_whole_ins::{compile_two_args_whole_ins, CompilingBytesForInstruction},
    AddressingMode,
};

pub(in crate::compiler) fn parse_xor(
    tokenized_line: &TokenizedLine,
    i: usize,
    compiled_bytes: &mut Vec<u8>,
    compiled_bytes_ref: &mut Vec<CompiledBytesReference>,
    addressing_mode: AddressingMode,
) -> Result<usize, CompilationError> {
    let ins = CompilingBytesForInstruction {
        reg_16bit_and_anything_ins: 0x33,
        reg_8bit_and_anything_ins: 0x32,
        indexed_addressing_and_anyting_ins: 0x31,
        addr_and_8bit_reg: 0x30,

        al_and_num_ins: Some(0x34),
        ax_and_num_ins: Some(0x35),
        reg16bit_and_16bit_num: 0x81,
        reg16bit_and_8bit_num: Some(0x83),
        reg8bit_and_num: 0x80,
        reg_num_sub_ins: 0xF0,

        addr16bit_and_16bit_num: 0x81,
        addr16bit_and_8bit_num: Some(0x83),
        addr8bit_and_num: 0x80,
        addr_num_sub_ins: 0x36,
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
mod xor_ins_tests {

    use crate::{compile_and_compare_ins, compiler::compile_str, test_compile};
    use pretty_assertions::assert_eq;

    compile_and_compare_ins!(
        test_or_reg16bit_and_anything,
        "
        xOR SP, 0x10
        xOR SI, 0x100 
        xOR BX, [BX]
        xOR SI, [0x100]
        xOR AX, [SI+BP+0x10]
        xOR BP, [BP+0x100]
        xOR CX, DI 

        xOR AX, 0x10
        xOR AX, 0x100
        ",
        vec![
            0x83, 0xF4, 0x10, 0x81, 0xF6, 0x00, 0x01, 0x33, 0x1F, 0x33, 0x36, 0x00, 0x01, 0x33,
            0x42, 0x10, 0x33, 0xAE, 0x00, 0x01, 0x33, 0xCF, 0x35, 0x10, 0x00, 0x35, 0x00, 0x01,
        ]
    );

    compile_and_compare_ins!(
        test_or_8bitreg_and_anything,
        "
        xor ah, dh
        xor dl, 0x10 
        xor bh, [0x100]
        ",
        vec![0x032, 0xE6, 0x80, 0xF2, 0x10, 0x32, 0x3E, 0x00, 0x01]
    );

    compile_and_compare_ins!(
        test_or_addr_and_anything,
        "
        xor b.[0x100], 0x10
        xor [0x100], 0x100

        xor [0x100], bp
        xor [bp], cx
        xor [si+0x10], ax
        xor [bp+0x100], bx

        xor [0x100], bl

        xor al, 0x10
        ",
        vec![
            0x80, 0x36, 0x00, 0x01, 0x10, 0x81, 0x36, 0x00, 0x01, 0x00, 0x01, 0x31, 0x2E, 0x00,
            0x01, 0x31, 0x4E, 0x00, 0x31, 0x44, 0x10, 0x31, 0x9E, 0x00, 0x01, 0x30, 0x1E, 0x00,
            0x01, 0x34, 0x10
        ]
    );
}
