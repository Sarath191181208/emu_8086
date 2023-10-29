use crate::{
    compiler::{
        compilation_error::CompilationError, parsers::utils::get_idx_from_reg,
        tokenized_line::TokenizedLine, CompiledBytesReference,
    },
    convert_and_push_instructions,
    cpu::instructions::add,
};

use super::{
    pattern_extractors::{
        compile_first_ins_reg_pattern::{
            parse_16bitreg_first_addr_mode, parse_8bitreg_first_addr_mode,
        },
        AddressingMode,
    },
    utils::{get_8bit_register, get_idx_from_token, push_instruction},
};

pub(in crate::compiler) fn parse_and(
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

    let reg_16bit_and_anything_ins = 0x23;
    let reg_8bit_and_anything_ins = 0x84;

    match addressing_mode {
        AddressingMode::Registers16bit {
            high_token: _,
            low_token: _,
        }
        | AddressingMode::Register16bitAndAddress {
            high_token: _,
            low_token: _,
            address_bytes: _,
            register_type: _,
        }
        | AddressingMode::Register16bitAndIndexedAddress {
            high_token: _,
            low_token: _,
        }
        | AddressingMode::Register16bitAndIndexedAddressWithOffset {
            high_token: _,
            low_token: _,
            offset: _,
        } => parse_16bitreg_first_addr_mode(
            i,
            addressing_mode,
            reg_16bit_and_anything_ins,
            tokenized_line,
            token,
            compiled_bytes,
            compiled_bytes_ref,
        ),
        AddressingMode::Registers8bit { high_token, low_token } => todo!(),
        AddressingMode::Registers16bitNumber { high_token, low_token, num } => todo!(),
        AddressingMode::Register8bitNumber { high_token, low_token, num } => todo!(),
        AddressingMode::Register8bitAndIndexedAddress { high_token, low_token, register_type } => todo!(),
        AddressingMode::Register8bitAndIndexedAddressWithOffset { high_token, low_token, register_type, offset } => todo!(),
        AddressingMode::AddressAnd16bitRegister { high_token, low_token, address_bytes, register_type } => todo!(),
        AddressingMode::AddressAnd16bitNumber { high_token, low_token, address_bytes, num } => todo!(),
        AddressingMode::Register8bitAndAddress { high_token, low_token, address_bytes, register_type } => todo!(),
        AddressingMode::AddressAnd8bitRegister { high_token, low_token, address_bytes, register_type } => todo!(),
        AddressingMode::AddressAnd8bitNumber { high_token, low_token, address_bytes, num } => todo!(),
        AddressingMode::ByteAddressAnd8bitNumber { high_token, low_token, address_bytes, num } => todo!(),
    }
}

#[cfg(test)]
mod and_ins_compilation_tests{
        use crate::{compile_and_compare_ins, compiler::compile_str, test_compile};
    use pretty_assertions::assert_eq;

    compile_and_compare_ins!(
        test_test_reg_reg,
        "
        and cx, bx
        ",
        vec![0x23, 0xCB]
    );
}
