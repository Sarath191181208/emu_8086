use crate::{
    compiler::{
        compilation_error::CompilationError,
        parsers::utils::{get_as_0xc0_0xff_pattern, push_instruction},
        tokenized_line::TokenizedLine,
        CompiledBytesReference,
    },
    convert_and_push_instructions,
};

use super::{
    pattern_extractors::{
        compile_first_ins_reg_pattern::{
            parse_16bitreg_first_addr_mode, parse_8bitreg_first_addr_mode,
        },
        compile_two_arguments_patterns::parse_indexed_addr_and_reg,
        AddressingMode,
    },
    utils::get_idx_from_token,
};

pub(in crate::compiler) fn parse_xchg(
    tokenized_line: &TokenizedLine,
    i: usize,
    compiled_bytes: &mut Vec<u8>,
    compiled_bytes_ref: &mut Vec<CompiledBytesReference>,
    addressing_mode: AddressingMode,
) -> Result<usize, CompilationError> {
    let token = tokenized_line.get(
        i,
        "This shouldn't happen, Please report this bug to the developer".to_string(),
        None,
    )?;

    let reg_8bit_and_anything_ins = 0x86;
    let reg_16bit_and_anything_ins = 0x87;
    let indexed_addressing_and_anyting_ins = 0x87;
    let byte_indexed_addressing_and_anyting_ins = 0x86;

    match addressing_mode.clone() {
        AddressingMode::Registers16bitNumber {
            high_token: _,
            low_token,
            num: _,
        }
        | AddressingMode::Register8bitNumber {
            high_token: _,
            low_token,
            num: _,
        }
        | AddressingMode::AddressAnd16bitNumber {
            high_token: _,
            low_token,
            address_bytes: _,
            num: _,
        }
        | AddressingMode::AddressAnd8bitNumber {
            high_token: _,
            low_token,
            address_bytes: _,
            num: _,
        }
        | AddressingMode::ByteAddressAnd8bitNumber {
            high_token: _,
            low_token,
            address_bytes: _,
            num: _,
        } => Err(CompilationError::error_with_token(
            &low_token,
            "Invalid addressing mode for XCHG instruction, only supports registers and indexed addressing as second argument",
        )),

        AddressingMode::Registers16bit {
            high_token,
            low_token,
        } => {
            let high_reg_idx = get_idx_from_token(&high_token)?;
            let low_reg_idx = get_idx_from_token(&low_token)?;
            let is_ax_in_low_or_high = high_reg_idx == 0 || low_reg_idx == 0;
            if is_ax_in_low_or_high {
                let ins = 0x90 + high_reg_idx + low_reg_idx;
                convert_and_push_instructions!(
                    compiled_bytes,
                    compiled_bytes_ref,
                    (
                        &low_token => vec![ins]
                    )
                );
            } else {
                let ins = get_as_0xc0_0xff_pattern(high_reg_idx, low_reg_idx);
                convert_and_push_instructions!(
                    compiled_bytes,
                    compiled_bytes_ref,
                    (
                        token => vec![0x87],
                        &low_token => vec![ins]
                    )
                );
            }
            Ok(i + 3)
        }

        AddressingMode::Register16bitAndIndexedAddressing {
            high_token: _,
            low_token: _,
            register_type: _,
            addr_type: _,
        } => parse_16bitreg_first_addr_mode(
            i,
            addressing_mode,
            reg_16bit_and_anything_ins,
            tokenized_line,
            token,
            compiled_bytes,
            compiled_bytes_ref,
        ),

        AddressingMode::AddressAnd8bitRegister {
            high_token,
            low_token,
            address_bytes,
            register_type,
        } => {
            // 0x86 0x00..0x07
            let reg_idx = register_type.get_as_idx();
            convert_and_push_instructions!(
                compiled_bytes,
                compiled_bytes_ref,
                (
                    token => vec![byte_indexed_addressing_and_anyting_ins],
                    &low_token => vec![0x06 | reg_idx << 3],
                    &high_token => address_bytes.to_vec()
                )
            );
            Ok(tokenized_line.len())
        }

        AddressingMode::IndexedAddressingAndRegister {
            high_token,
            low_token,
            register_type,
            addr_type,
        } => {
            parse_indexed_addr_and_reg(
                indexed_addressing_and_anyting_ins,
                token,
                &high_token,
                &low_token,
                register_type,
                addr_type,
                compiled_bytes,
                compiled_bytes_ref,
            )?;
            Ok(tokenized_line.len())
        }

        AddressingMode::Registers8bit {
            high_token: _,
            low_token: _,
        }
        | AddressingMode::Register8bitAndIndexedAddressing {
            high_token: _,
            low_token: _,
            register_type: _,
            addr_type: _,
        } => parse_8bitreg_first_addr_mode(
            i,
            addressing_mode,
            reg_8bit_and_anything_ins,
            tokenized_line,
            token,
            compiled_bytes,
            compiled_bytes_ref,
        ),
    }
}

#[cfg(test)]
mod xor_ins_tests {

    use crate::{compile_and_compare_ins, compiler::compile_str, test_compile};
    use pretty_assertions::assert_eq;

    compile_and_compare_ins!(
        test_xchg_reg16bit_and_anything,
        "
        XCHG BX, [BX]
        XCHG SI, [0x100]
        XCHG AX, [SI+BP+0x10]
        XCHG BP, [BP+0x100]
        XCHG CX, DI
        xchg ax, sp 
        xchg bp, di 
        ",
        vec![
            0x87, 0x1F, 0x87, 0x36, 0x00, 0x01, 0x87, 0x42, 0x10, 0x87, 0xAE, 0x00, 0x01, 0x87,
            0xCF, 0x94, 0x87, 0xEF
        ]
    );

    compile_and_compare_ins!(
        test_xchg_8bitreg_and_anything,
        "
        xchg ah, dh
        xchg bh, [0x100]
        ",
        vec![
            0x86, 0xE6, 
            0x86, 0x3E, 0x00, 0x01
        ]
    );

        compile_and_compare_ins!(
        test_or_addr_and_anything,
        "
        XCHG [0x100], bp
        XCHG [bp], cx
        XCHG [0x100], bl

        ",
        vec![
            0x87, 0x2E, 0x00, 0x01, 
            0x87, 0x4E, 0x00, 
            0x86, 0x1E, 0x00, 0x01
        ]
    );
}
