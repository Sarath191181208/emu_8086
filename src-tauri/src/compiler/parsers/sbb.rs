use crate::compiler::{
    compilation_error::CompilationError, tokenized_line::TokenizedLine,
    types_structs::CompiledBytesReference,
};

use super::pattern_extractors::{
    compile_tow_args_whole_ins::{compile_two_args_whole_ins, CompilingBytesForInstruction},
    AddressingMode,
};

pub(in crate::compiler) fn parse_sbb(
    tokenized_line: &TokenizedLine,
    i: usize,
    compiled_bytes: &mut Vec<u8>,
    compiled_bytes_ref: &mut Vec<CompiledBytesReference>,
    addressing_mode: AddressingMode,
) -> Result<usize, CompilationError> {
    let ins = CompilingBytesForInstruction {
        addr_and_8bit_reg: 0x18,
        indexed_addressing_and_anyting_ins: 0x19,
        reg_8bit_and_anything_ins: 0x1A,
        reg_16bit_and_anything_ins: 0x1B,

        al_and_num_ins: Some(0x1C),
        ax_and_num_ins: Some(0x1D),
        reg16bit_and_16bit_num: 0x81,
        reg16bit_and_8bit_num: Some(0x83),
        reg8bit_and_num: 0x80,
        reg_num_sub_ins: 0xD8,

        addr8bit_and_num: 0x80,
        addr16bit_and_16bit_num: 0x81,
        addr16bit_and_8bit_num: Some(0x83),
        addr_num_sub_ins: 0x1E,
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
mod tests16bit {

    use crate::{compile_and_compare_ins, compiler::compile_str, test_compile};
    use pretty_assertions::assert_eq;

    compile_and_compare_ins!(
        test_sbb_reg16bit_and_anything,
        "
        SBB SP, 0x10
        SBB SI, 0x100 
        SBB BX, [BX]
        SBB SI, [0x100]
        SBB AX, [SI+BP+0x10]
        SBB BP, [BP+0x100]
        SBB CX, DI 

        SBB AX, 0x10
        SBB AX, 0x100
        ",
        vec![
            0x83, 0xDC, 0x10, 0x81, 0xDE, 0x00, 0x01, 0x1B, 0x1F, 0x1B, 0x36, 0x00, 0x01, 0x1B,
            0x42, 0x10, 0x1B, 0xAE, 0x00, 0x01, 0x1B, 0xCF, 0x1D, 0x10, 0x00, 0x1D, 0x00, 0x01
        ]
    );

    compile_and_compare_ins!(
        test_sbb_8bitreg_and_anything,
        "
        SBB ah, dh
        SBB dl, 0x10 
        SBB bh, [0x100]
        ",
        vec![0x1A, 0xe6, 0x80, 0xDA, 0x10, 0x1A, 0x3E, 0x00, 0x01]
    );

    compile_and_compare_ins!(
        test_sbb_addr_and_anything,
        "
        SBB b.[0x100], 0x10
        SBB [0x100], 0x100

        SBB [0x100], bp
        SBB [bp], cx
        SBB [si+0x10], ax
        SBB [bp+0x100], bx

        SBB [0x100], bl

        SBB al, 0x10
        ",
        vec![
            0x80, 0x1E, 0x00, 0x01, 0x10, 0x81, 0x1E, 0x00, 0x01, 0x00, 0x01, 0x19, 0x2E, 0x00,
            0x01, 0x19, 0x4E, 0x00, 0x19, 0x44, 0x10, 0x19, 0x9E, 0x00, 0x01, 0x18, 0x1E, 0x00,
            0x01, 0x1C, 0x10
        ]
    );
}
