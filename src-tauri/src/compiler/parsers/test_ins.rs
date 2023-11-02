use crate::compiler::{
    compilation_error::CompilationError, tokenized_line::TokenizedLine, CompiledBytesReference,
};

use super::pattern_extractors::{
    compile_tow_args_whole_ins::{compile_two_args_whole_ins, CompilingBytesForInstruction},
    AddressingMode,
};

pub(in crate::compiler) fn parse_test(
    tokenized_line: &TokenizedLine,
    i: usize,
    compiled_bytes: &mut Vec<u8>,
    compiled_bytes_ref: &mut Vec<CompiledBytesReference>,
    addressing_mode: AddressingMode,
) -> Result<usize, CompilationError> {
    let _token = tokenized_line.get(
        i,
        "This shouldn't happen, Please report this".to_string(),
        None,
    )?;
    // let reg_16bit_and_anything_ins = 0x85;
    // let reg_8bit_and_anything_ins = 0x84;

    let ins = CompilingBytesForInstruction {
        reg_16bit_and_anything_ins: 0x85,
        reg_8bit_and_anything_ins: 0x84,
        indexed_addressing_and_anyting_ins: 0x85,
        addr_and_8bit_reg: 0x84,

        al_and_num_ins: Some(0xA8),
        ax_and_num_ins: Some(0xA9),
        reg16bit_and_16bit_num: 0xF7,
        reg16bit_and_8bit_num: None,
        reg8bit_and_num: 0xF6,
        reg_num_sub_ins: 0xC0,

        addr16bit_and_16bit_num: 0xF7,
        addr16bit_and_8bit_num: None,
        addr8bit_and_num: 0xF6,
        addr_num_sub_ins: 0x06,
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
mod test_ins_tests {
    use crate::{compile_and_compare_ins, compiler::compile_str, test_compile};
    use pretty_assertions::assert_eq;

    compile_and_compare_ins!(
        test_test_reg_reg,
        "
        test ax, bx
        test ch, bl
        ",
        vec![0x85, 0xC3, 0x84, 0xEB]
    );

    compile_and_compare_ins!(
        test_test_reg_num,
        "
        test ax, 0x10  
        test cx, 0x10
        test cx, 0x20_0
        test bx, 0x200    

        test aL, 0x10 
        test Ah  , 0x20 
        ",
        vec![
            0xA9, 0x10, 0x00, 0xF7, 0xC1, 0x10, 0x00, 0xF7, 0xC1, 0x00, 0x02, 0xF7, 0xC3, 0x00,
            0x02, 0xA8, 0x10, 0xF6, 0xC4, 0x20
        ]
    );

    compile_and_compare_ins!(
        test_test_variable_addressing,
        "
        org 100h 
        .data 
        var dw 0x20
        var2 db 0x20

        code: 
        test ax, var
        test bx, var 
        test ax, [0x102]  
        test [0x102] , ax 
                    
        test var, 0x10
        test var, 0x100


        test al, var2 
        test cl, b.[var]   

        test var, 0x10
        test b.[var], 0x10
        ",
        vec![
            0xEB, 0x03, 0x20, 0x00, 0x20, 0x85, 0x06, 0x02, 0x01, 0x85, 0x1E, 0x02, 0x01, 0x85,
            0x06, 0x02, 0x01, 0x85, 0x06, 0x02, 0x01, 0xF7, 0x06, 0x02, 0x01, 0x10, 0x00, 0xF7,
            0x06, 0x02, 0x01, 0x00, 0x01, 0x84, 0x06, 0x04, 0x01, 0x84, 0x0E, 0x02, 0x01, 0xF7,
            0x06, 0x02, 0x01, 0x10, 0x00, 0xF6, 0x06, 0x02, 0x01, 0x10
        ]
    );

    compile_and_compare_ins!(
        test_test_reg_idx_addr,
        "
        test ax, [bx]
        test cx, [bx]

        test bx, [bx+si+0x10]

        test al, [bx]
        test cl, [bx]

        test ah, [bp]
        ",
        vec![0x85, 0x07, 0x85, 0x0F, 0x85, 0x58, 0x10, 0x84, 0x07, 0x84, 0x0F, 0x84, 0x66, 0x00]
    );

    compile_and_compare_ins!(
        test_test_mem_reg,
        "
        test [bx+di+0xbe25], sp
        test bp+di, di
        test [bx+si+0xc6f4], sp
        ",
        vec![0x85, 0xA1, 0x25, 0xBE, 0x85, 0x3B, 0x85, 0xA0, 0xF4, 0xC6]
    );
}
