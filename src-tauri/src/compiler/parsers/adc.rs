use crate::compiler::{
    compilation_error::CompilationError, tokenized_line::TokenizedLine, CompiledBytesReference,
};

use super::pattern_extractors::{
    compile_tow_args_whole_ins::{compile_two_args_whole_ins, CompilingBytesForInstruction},
    AddressingMode,
};

pub(in crate::compiler) fn parse_adc(
    tokenized_line: &TokenizedLine,
    i: usize,
    compiled_bytes: &mut Vec<u8>,
    compiled_bytes_ref: &mut Vec<CompiledBytesReference>,
    addressing_mode: AddressingMode,
) -> Result<usize, CompilationError> {
    let ins = CompilingBytesForInstruction {
        reg_16bit_and_anything_ins: 0x13,
        reg_8bit_and_anything_ins: 0x12,
        indexed_addressing_and_anyting_ins: 0x11,
        addr_and_8bit_reg: 0x10,

        al_and_num_ins: Some(0x14),
        ax_and_num_ins: Some(0x15),
        reg16bit_and_16bit_num: 0x81,
        reg16bit_and_8bit_num: Some(0x83),
        reg8bit_and_num: 0x80,
        reg_num_sub_ins: 0xD0,

        addr16bit_and_16bit_num: 0x81,
        addr16bit_and_8bit_num: Some(0x83),
        addr8bit_and_num: 0x80,
        addr_num_sub_ins: 0x16,
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
mod compile_adc_tests {

    use crate::{compile_and_compare_ins, compiler::compile_str, test_compile};
    use pretty_assertions::assert_eq;

    compile_and_compare_ins!(
        test_or_reg16bit_and_anything,
        "
        ADC ax, bx
        ADC bl, cl

        ADC b.[0x100], al
        ADC w.[0x100], ax

        ADC AX, 0x100
        ADC BX, 0x10
        ADC CX, 0x100
        ADC al, 0x10    
        ADC BL, 0x10

        ADC b.[0x100], 0x10 
        ADC [0x100], 0x100

        ADC b.[0x100], 0x10
        ",
        vec![
            0x13, 0xC3, 0x12, 0xD9, 0x10, 0x06, 0x00, 0x01, 0x11, 0x06, 0x00, 0x01, 0x15, 0x00,
            0x01, 0x83, 0xD3, 0x10, 0x81, 0xD1, 0x00, 0x01, 0x14, 0x10, 0x80, 0xD3, 0x10, 0x80,
            0x16, 0x00, 0x01, 0x10, 0x81, 0x16, 0x00, 0x01, 0x00, 0x01, 0x80, 0x16, 0x00, 0x01,
            0x10
        ]
    );
}
