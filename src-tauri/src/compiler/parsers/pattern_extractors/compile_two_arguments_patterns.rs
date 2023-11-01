use crate::{
    compiler::{
        compilation_error::CompilationError,
        parsers::utils::{
            get_as_0x00_0x3f_pattern, get_as_0x40_0x7f_pattern, get_as_0x80_0xbf_pattern,
            get_idx_from_token, get_index_addr_as_idx, push_instruction,
        },
        tokens::{
            indexed_addressing_types::IndexedAddressingTypes, registers16bit::Registers16bit,
            registers8bit::Registers8bit, Assembly8086Tokens, SignedU16, Token,
        },
        types_structs::CompiledBytesReference,
    },
    convert_and_push_instructions,
    utils::Either,
};

fn parse_reg_and_indexed_reg_without_offset(
    high_reg_idx: u8,
    base_instruction: u8,
    token: &Token,
    high_token: &Token,
    low_token: &Token,
    compiled_bytes: &mut Vec<u8>,
    compiled_bytes_ref: &mut Vec<CompiledBytesReference>,
) -> Result<(), CompilationError> {
    let low_reg_idx = get_index_addr_as_idx(low_token)?;
    let ins = get_as_0x00_0x3f_pattern(high_reg_idx, low_reg_idx);
    match &low_token.token_type {
        Assembly8086Tokens::IndexedAddressing(IndexedAddressingTypes::BP(_)) => {
            convert_and_push_instructions!(
                compiled_bytes,
                compiled_bytes_ref,
                (
                    token => vec![base_instruction],
                   high_token=> vec![0x46 | (high_reg_idx << 3), 0x00]
                )
            );
            Ok(())
        }
        _ => {
            convert_and_push_instructions!(
                compiled_bytes,
                compiled_bytes_ref,
                (
                    token => vec![base_instruction],
                   low_token=> vec![ins]
                )
            );
            Ok(())
        }
    }
}

pub(crate) fn parse_register_8bit_and_indexed_registers_without_offset(
    base_instruction: u8,
    register: Registers8bit,
    token: &Token,
    high_token: &Token,
    low_token: &Token,
    compiled_bytes: &mut Vec<u8>,
    compiled_bytes_ref: &mut Vec<CompiledBytesReference>,
) -> Result<(), CompilationError> {
    let high_reg_idx = register.get_as_idx();
    parse_reg_and_indexed_reg_without_offset(
        high_reg_idx,
        base_instruction,
        token,
        high_token,
        low_token,
        compiled_bytes,
        compiled_bytes_ref,
    )
}

pub(crate) fn parse_register_16bit_and_indexed_registers_without_offset(
    base_instruction: u8,
    token: &Token,
    high_token: &Token,
    low_token: &Token,
    compiled_bytes: &mut Vec<u8>,
    compiled_bytes_ref: &mut Vec<CompiledBytesReference>,
) -> Result<(), CompilationError> {
    let high_reg_idx = get_idx_from_token(high_token)?;
    parse_reg_and_indexed_reg_without_offset(
        high_reg_idx,
        base_instruction,
        token,
        high_token,
        low_token,
        compiled_bytes,
        compiled_bytes_ref,
    )
}

#[allow(clippy::too_many_arguments)]
pub(in super::super) fn parse_register_and_indexed_registers_with_offset(
    base_instruction: u8,
    high_reg_idx: u8,
    token: &Token,
    high_token: &Token,
    low_token: &Token,
    offset: &SignedU16,
    compiled_bytes: &mut Vec<u8>,
    compiled_bytes_ref: &mut Vec<CompiledBytesReference>,
) -> Result<(), CompilationError> {
    let offset = offset.as_either_u8_or_u16(low_token)?;
    let low_reg_idx = get_index_addr_as_idx(low_token)?;
    let ins = match &offset {
        Either::Left(_) => vec![get_as_0x40_0x7f_pattern(high_reg_idx, low_reg_idx)],
        Either::Right(_) => vec![get_as_0x80_0xbf_pattern(high_reg_idx, low_reg_idx)],
    };
    let offset_bytes = offset.get_as_bytes();

    convert_and_push_instructions!(
        compiled_bytes,
        compiled_bytes_ref,
        (
            token => vec![base_instruction],
            high_token=> ins,
            low_token=> offset_bytes
        )
    );

    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub(in super::super) fn parse_register_8bit_and_indexed_registers_with_offset(
    base_instruction: u8,
    register: Registers8bit,
    token: &Token,
    high_token: &Token,
    low_token: &Token,
    offset: &SignedU16,
    compiled_bytes: &mut Vec<u8>,
    compiled_bytes_ref: &mut Vec<CompiledBytesReference>,
) -> Result<(), CompilationError> {
    let high_reg_idx = register.get_as_idx();
    parse_register_and_indexed_registers_with_offset(
        base_instruction,
        high_reg_idx,
        token,
        high_token,
        low_token,
        offset,
        compiled_bytes,
        compiled_bytes_ref,
    )
}

pub(crate) fn parse_register_16bit_and_indexed_registers_with_offset(
    base_instruction: u8,
    token: &Token,
    high_token: &Token,
    low_token: &Token,
    offset: &SignedU16,
    compiled_bytes: &mut Vec<u8>,
    compiled_bytes_ref: &mut Vec<CompiledBytesReference>,
) -> Result<(), CompilationError> {
    let high_reg_idx = get_idx_from_token(high_token)?;
    parse_register_and_indexed_registers_with_offset(
        base_instruction,
        high_reg_idx,
        token,
        high_token,
        low_token,
        offset,
        compiled_bytes,
        compiled_bytes_ref,
    )
}

#[allow(clippy::too_many_arguments)]
pub(in crate::compiler) fn parse_indexed_addr_and_reg(
    base_instruction: u8,
    token: &Token,
    high_token: &Token,
    low_token: &Token,
    reg_type: Registers16bit,
    idx_addr_type: IndexedAddressingTypes,
    compiled_bytes: &mut Vec<u8>,
    compiled_bytes_ref: &mut Vec<CompiledBytesReference>,
) -> Result<(), CompilationError> {
    let reg_idx = reg_type.get_index_or_err(low_token)?;
    parse_index_addr_and_reg_idx(
        base_instruction,
        reg_idx,
        token,
        high_token,
        low_token,
        idx_addr_type,
        compiled_bytes,
        compiled_bytes_ref,
    )
}

#[allow(clippy::too_many_arguments)]
fn parse_index_addr_and_reg_idx(
    base_instruction: u8,
    reg_idx: u8,
    token: &Token,
    high_token: &Token,
    low_token: &Token,
    idx_addr_type: IndexedAddressingTypes,
    compiled_bytes: &mut Vec<u8>,
    compiled_bytes_ref: &mut Vec<CompiledBytesReference>,
) -> Result<(), CompilationError> {
    let offset = idx_addr_type.get_offset_and_default_bp_to_0();

    if let IndexedAddressingTypes::Offset(address) = idx_addr_type {
        convert_and_push_instructions!(
            compiled_bytes,
            compiled_bytes_ref,
            (
                token => vec![base_instruction],
                high_token=> vec![0x6  | reg_idx << 3],
                low_token=> address.as_u16().to_le_bytes().to_vec()
            )
        );
        return Ok(());
    }

    let addr_type_idx = idx_addr_type.get_index_or_err(high_token)?;

    match offset {
        Some(offset) => {
            let offset = offset.as_either_u8_or_u16(low_token)?;
            match offset {
                Either::Left(val) => {
                    let ins = get_as_0x40_0x7f_pattern(reg_idx, addr_type_idx);
                    convert_and_push_instructions!(
                        compiled_bytes,
                        compiled_bytes_ref,
                        (
                            token => vec![base_instruction],
                            high_token=> vec![ins],
                            low_token=> val.to_le_bytes().to_vec()
                        )
                    );
                }
                Either::Right(val) => {
                    let ins = get_as_0x80_0xbf_pattern(reg_idx, addr_type_idx);
                    convert_and_push_instructions!(
                        compiled_bytes,
                        compiled_bytes_ref,
                        (
                            token => vec![base_instruction],
                            high_token=> vec![ins],
                            low_token=> val.to_le_bytes().to_vec()
                        )
                    );
                }
            }
        }
        None => {
            let ins = get_as_0x00_0x3f_pattern(reg_idx, addr_type_idx);
            convert_and_push_instructions!(
                compiled_bytes,
                compiled_bytes_ref,
                (
                    token => vec![base_instruction],
                    high_token=> vec![ins]
                )
            );
        }
    }

    Ok(())
}
