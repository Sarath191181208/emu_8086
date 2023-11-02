use crate::{
    compiler::{
        compilation_error::CompilationError,
        parsers::utils::{
            get_8bit_register, get_as_0xc0_0xff_pattern, get_idx_from_reg, get_idx_from_token,
            push_instruction,
        },
        tokenized_line::TokenizedLine,
        tokens::Token,
        types_structs::CompiledBytesReference,
    },
    convert_and_push_instructions,
};

use super::{
    compile_two_arguments_patterns::{
        parse_register_16bit_and_indexed_registers_with_offset,
        parse_register_16bit_and_indexed_registers_without_offset,
        parse_register_8bit_and_indexed_registers_with_offset,
        parse_register_8bit_and_indexed_registers_without_offset, parse_indexed_addr_and_reg,
    },
    AddressingMode,
};

pub(in crate::compiler) fn parse_8bitreg_first_addr_mode(
    i: usize,
    addressing_mode: AddressingMode,
    root_ins: u8,
    tokenized_line: &TokenizedLine,
    token: &Token,
    compiled_bytes: &mut Vec<u8>,
    compiled_bytes_ref: &mut Vec<CompiledBytesReference>,
) -> Result<usize, CompilationError> {
    match addressing_mode {
        AddressingMode::Register8bitAndAddress {
            high_token,
            low_token,
            address_bytes,
            register_type,
        } => {
            // 0x84 0x06 | 0x0E | 0x16 | 0x1E | 0x26 | 0x2E | 0x36 | 0x3E
            let reg_idx = register_type.get_as_idx();
            convert_and_push_instructions!(
                compiled_bytes,
                compiled_bytes_ref,
                (
                    token => vec![root_ins],
                    &high_token => vec![0x06 | reg_idx << 3],
                    &low_token => address_bytes.to_vec()
                )
            );
            Ok(tokenized_line.len())
        }

        AddressingMode::Register8bitAndIndexedAddress {
            high_token,
            low_token,
            register_type,
        } => {
            // 0x84 0x00..0x3F
            parse_register_8bit_and_indexed_registers_without_offset(
                root_ins,
                register_type,
                token,
                &high_token,
                &low_token,
                compiled_bytes,
                compiled_bytes_ref,
            )?;

            Ok(tokenized_line.len())
        }
        AddressingMode::Register8bitAndIndexedAddressWithOffset {
            high_token,
            low_token,
            offset,
            register_type,
        } => {
            // 0x84 0x40..0x7F/0x80..0xBF
            parse_register_8bit_and_indexed_registers_with_offset(
                root_ins,
                register_type,
                token,
                &high_token,
                &low_token,
                &offset,
                compiled_bytes,
                compiled_bytes_ref,
            )?;
            Ok(tokenized_line.len())
        }
        AddressingMode::Registers8bit {
            high_token,
            low_token,
        } => {
            // 0x84 0xC0..0xFF
            let high_reg = get_8bit_register(&high_token);
            let low_reg = get_8bit_register(&low_token);
            let ins = get_as_0xc0_0xff_pattern(high_reg.get_as_idx(), low_reg.get_as_idx());
            convert_and_push_instructions!(
                compiled_bytes,
                compiled_bytes_ref,
                (
                    token => vec![root_ins],
                    &high_token => vec![ins]
                )
            );
            Ok(i + 3)
        }
        _ => panic!("Invalid use of the `parse_reg_first_addr_mode` function"),
    }
}

pub(in crate::compiler) fn parse_16bitreg_first_addr_mode(
    i: usize,
    addressing_mode: AddressingMode,
    root_ins: u8,
    tokenized_line: &TokenizedLine,
    token: &Token,
    compiled_bytes: &mut Vec<u8>,
    compiled_bytes_ref: &mut Vec<CompiledBytesReference>,
) -> Result<usize, CompilationError> {
    match addressing_mode {
        AddressingMode::Register16bitAndIndexedAddressing { high_token, low_token, register_type, addr_type } => {
            parse_indexed_addr_and_reg(
                root_ins,
                token,
                &low_token,
                &high_token,
                register_type,
                addr_type,
                compiled_bytes,
                compiled_bytes_ref,
            )?;
            Ok(tokenized_line.len())
        }

        AddressingMode::Registers16bit {
            high_token,
            low_token,
        } => {
            // 0x85 0xC0..0xFF
            let high_reg_idx = get_idx_from_token(&high_token)?;
            let low_reg_idx = get_idx_from_token(&low_token)?;
            let ins = get_as_0xc0_0xff_pattern(high_reg_idx, low_reg_idx);
            convert_and_push_instructions!(
                compiled_bytes,
                compiled_bytes_ref,
                (
                    token => vec![root_ins],
                    &high_token => vec![ins]
                )
            );
            Ok(i + 3)
        }

        _ => {
            Err(CompilationError::error_line(
                token.line_number ,
                &format!(
                    "Invalid use of the `parse_16bitreg_first_addr_mode` function, Please report this issue!",
                )))
            
        },
    }
}
