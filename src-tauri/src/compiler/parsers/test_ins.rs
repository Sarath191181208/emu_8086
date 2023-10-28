use crate::{
    compiler::{
        compilation_error::CompilationError, parsers::utils::get_idx_from_reg,
        tokenized_line::TokenizedLine, CompiledBytesReference,
    },
    convert_and_push_instructions,
    utils::Either,
};

use super::{
    pattern_extractors::{
        compile_two_arguments_patterns::{
            parse_register_16bit_and_indexed_registers_with_offset,
            parse_register_16bit_and_indexed_registers_without_offset,
            parse_register_8bit_and_indexed_registers_with_offset,
            parse_register_8bit_and_indexed_registers_without_offset,
        },
        AddressingMode,
    },
    utils::{get_8bit_register, get_as_0xc0_0xff_pattern, get_idx_from_token, push_instruction},
};

pub(in crate::compiler) fn parse_test(
    tokenized_line: &TokenizedLine,
    i: usize,
    compiled_bytes: &mut Vec<u8>,
    compiled_bytes_ref: &mut Vec<CompiledBytesReference>,
    addressing_mode: AddressingMode,
) -> Result<usize, CompilationError> {
    let token = tokenized_line.get(
        i,
        "This shouldn't happen, Please report this".to_string(),
        None,
    )?;
    match addressing_mode {
        AddressingMode::Registers16bit { high_token, low_token } => todo!(),
        AddressingMode::Registers8bit { high_token, low_token } => todo!(),
        AddressingMode::Registers16bitNumber { high_token, low_token, num } => todo!(),
        AddressingMode::Register8bitNumber { high_token, low_token, num } => todo!(),
        AddressingMode::Register16bitAndAddress { high_token, low_token, address_bytes, register_type } => todo!(),
        AddressingMode::Register16bitAndIndexedAddress { high_token, low_token } => todo!(),
        AddressingMode::Register8bitAndIndexedAddress { high_token, low_token, register_type } => todo!(),
        AddressingMode::Register16bitAndIndexedAddressWithOffset { high_token, low_token, offset } => todo!(),
        AddressingMode::Register8bitAndIndexedAddressWithOffset { high_token, low_token, register_type, offset } => todo!(),
        AddressingMode::AddressAnd16bitRegister { high_token, low_token, address_bytes, register_type } => todo!(),
        AddressingMode::AddressAnd16bitNumber { high_token, low_token, address_bytes, num } => todo!(),
        AddressingMode::Register8bitAndAddress { high_token, low_token, address_bytes, register_type } => todo!(),
        AddressingMode::AddressAnd8bitRegister { high_token, low_token, address_bytes, register_type } => todo!(),
        AddressingMode::AddressAnd8bitNumber { high_token, low_token, address_bytes, num } => todo!(),
        AddressingMode::ByteAddressAnd8bitNumber { high_token, low_token, address_bytes, num } => todo!(),
    }
}