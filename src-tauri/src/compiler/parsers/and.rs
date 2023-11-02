use crate::compiler::{
    compilation_error::CompilationError, tokenized_line::TokenizedLine, CompiledBytesReference,
};

use super::pattern_extractors::{
    compile_tow_args_whole_ins::{compile_two_args_whole_ins, CompilingBytesForInstruction},
    AddressingMode,
};

pub(in crate::compiler) fn parse_and(
    tokenized_line: &TokenizedLine,
    i: usize,
    compiled_bytes: &mut Vec<u8>,
    compiled_bytes_ref: &mut Vec<CompiledBytesReference>,
    addressing_mode: AddressingMode,
) -> Result<usize, CompilationError> {
    let ins = CompilingBytesForInstruction {
        reg_16bit_and_anything_ins: 0x23,
        reg_8bit_and_anything_ins: 0x22,
        indexed_addressing_and_anyting_ins: 0x21,
        addr_and_8bit_reg: 0x20,

        al_and_num_ins: Some(0x24),
        ax_and_num_ins: Some(0x25),
        reg16bit_and_16bit_num: 0x81,
        reg16bit_and_8bit_num: 0x83,
        reg8bit_and_num: 0x80,
        reg_num_sub_ins: 0xE0,

        addr16bit_and_16bit_num: 0x81,
        addr16bit_and_8bit_num: 0x83,
        addr8bit_and_num: 0x80,
        addr_num_sub_ins: 0x26,
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
mod and_ins_compilation_tests {
    use crate::{compile_and_compare_ins, compiler::compile_str, test_compile};
    use pretty_assertions::assert_eq;

    compile_and_compare_ins!(
        test_register_as_first_ins,
        "
        and cx, bx
        and dx, [bx]
        and sp, [0x1234]
        and di, [0x12 + si]
        and ax, [0x1234 + bx]
        ",
        vec![
            0x23, 0xCB, 0x23, 0x17, 0x23, 0x26, 0x34, 0x12, 0x23, 0x7C, 0x12, 0x23, 0x87, 0x34,
            0x12
        ]
    );

    compile_and_compare_ins!(
        test_register_8bit_as_first_ins,
        "
        and cl, bl
        and dl, [bx]
        and bh, [0x1234]
        and dh, [0x12 + si]
        and al, [0x1234 + bx]
        ",
        vec![
            0x22, 0xCB, 0x22, 0x17, 0x22, 0x3E, 0x34, 0x12, 0x22, 0x74, 0x12, 0x22, 0x87, 0x34,
            0x12
        ]
    );

    compile_and_compare_ins!(
        test_reg_and_number,
        "
    and ax, 0x10
    and bx, 0x10
    and ax, 0x1234
    and si, 0x245

    and al, 0x12
    and ch, 0x24
    ",
        vec![
            0x25, 0x10, 0x00, 0x83, 0xE3, 0x10, 0x25, 0x34, 0x12, 0x81, 0xE6, 0x45, 0x02, 0x24,
            0x12, 0x80, 0xE5, 0x24
        ]
    );

    compile_and_compare_ins!(
        test_addr_and_reg_16bit,
        "
    and [0x1234], ax
    and [0x1234], bx

    and [0x123], al 
    and [0x123], cl
    ",
        vec![
            0x21, 0x06, 0x34, 0x12, 0x21, 0x1E, 0x34, 0x12, 0x20, 0x06, 0x23, 0x01, 0x20, 0x0E,
            0x23, 0x01
        ]
    );

    compile_and_compare_ins!(
        test_addr_and_num,
        "
        and [0x234], 0x123
        and [0x234], 0x100
        and w.[0x1234], 0x12
        and b.[0x1234], 0x12
        ",
        vec![
            0x81, 0x26, 0x34, 0x02, 0x23, 0x01, 0x81, 0x26, 0x34, 0x02, 0x00, 0x01, 0x83, 0x26,
            0x34, 0x12, 0x12, 0x80, 0x26, 0x34, 0x12, 0x12
        ]
    );

    compile_and_compare_ins!(
        test_mem_indexed_addr_and_reg,
        "
        AND [bx+si+0xd9], ax
        AND [bx+0x80], sp
        AND [bp], di
        AND [si+0x1d], cx
        AND [bp+0x6c9a], cx
        ",
        vec![
            0x21, 0x80, 0xD9, 0x00, 0x21, 0xA7, 0x80, 0x00, 0x21, 0x7E, 0x00, 0x21, 0x4C, 0x1D,
            0x21, 0x8E, 0x9A, 0x6C
        ]
    );
}
