use crate::compiler::{
    compilation_error::CompilationError, tokenized_line::TokenizedLine, CompiledBytesReference,
};

use super::pattern_extractors::{
    compile_tow_args_whole_ins::{compile_two_args_whole_ins, CompilingBytesForInstruction},
    AddressingMode,
};

pub(in crate::compiler) fn parse_or(
    tokenized_line: &TokenizedLine,
    i: usize,
    compiled_bytes: &mut Vec<u8>,
    compiled_bytes_ref: &mut Vec<CompiledBytesReference>,
    addressing_mode: AddressingMode,
) -> Result<usize, CompilationError> {
    let ins = CompilingBytesForInstruction {
        reg_16bit_and_anything_ins: 0x0B,
        reg_8bit_and_anything_ins: 0x0A,
        indexed_addressing_and_anyting_ins: 0x09,
        addr_and_8bit_reg: 0x08,

        al_and_num_ins: Some(0x0C),
        ax_and_num_ins: Some(0x0D),
        reg16bit_and_16bit_num: 0x81,
        reg16bit_and_8bit_num: 0x83,
        reg8bit_and_num: 0x80,
        reg_num_sub_ins: 0xC8,

        addr16bit_and_16bit_num: 0x81,
        addr16bit_and_8bit_num: 0x83,
        addr8bit_and_num: 0x80,
        addr_num_sub_ins: 0x0E,
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
mod or_ins_tests {

    use crate::{compile_and_compare_ins, compiler::compile_str, test_compile};
    use pretty_assertions::assert_eq;

    compile_and_compare_ins!(
        test_or_reg16bit_and_anything,
        "
        OR SP, 0x10
        OR SI, 0x100 
        OR BX, [BX]
        OR SI, [0x100]
        OR AX, [SI+BP+0x10]
        OR BP, [BP+0x100]
        OR CX, DI 

        OR AX, 0x10
        OR AX, 0x100
        ",
        vec![
            0x83, 0xCC, 0x10, 0x81, 0xCE, 0x00, 0x01, 0x0B, 0x1F, 0x0B, 0x36, 0x00, 0x01, 0x0B,
            0x42, 0x10, 0x0B, 0xAE, 0x00, 0x01, 0x0B, 0xCF, 0x0D, 0x10, 0x00, 0x0D, 0x00, 0x01,
        ]
    );

    compile_and_compare_ins!(
        test_or_8bitreg_and_anything,
        "
        or ah, dh
        or dl, 0x10 
        or bh, [0x100]
        ",
        vec![0x0A, 0xE6, 0x80, 0xCA, 0x10, 0x0A, 0x3E, 0x00, 0x01,]
    );

    compile_and_compare_ins!(
        test_or_addr_and_anything,
        "
        or b.[0x100], 0x10
        or [0x100], 0x100

        or [0x100], bp
        or [bp], cx
        or [si+0x10], ax
        or [bp+0x100], bx

        or [0x100], bl

        or al, 0x10
        ",
        vec![
            0x80, 0x0E, 0x00, 0x01, 0x10, 0x81, 0x0E, 0x00, 0x01, 0x00, 0x01, 0x9, 0x2E, 0x00,
            0x01, 0x09, 0x4E, 0x00, 0x09, 0x44, 0x10, 0x09, 0x9E, 0x00, 0x01, 0x08, 0x1E, 0x00,
            0x01, 0x0C, 0x10
        ]
    );
}
