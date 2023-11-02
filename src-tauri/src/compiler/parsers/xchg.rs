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
    let indexed_addressing_and_anyting_ins = 0x86;

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
            "Invalid addressing mode for XCHG instruction",
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
                    token => vec![0x86],
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
        }=> parse_8bitreg_first_addr_mode(
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
