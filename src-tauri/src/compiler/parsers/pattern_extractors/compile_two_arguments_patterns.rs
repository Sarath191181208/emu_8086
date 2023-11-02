use crate::{
    compiler::{
        compilation_error::CompilationError,
        parsers::utils::{
            get_as_0x00_0x3f_pattern, get_as_0x40_0x7f_pattern, get_as_0x80_0xbf_pattern,
            push_instruction,
        },
        tokens::{
            indexed_addressing_types::IndexedAddressingTypes, registers16bit::Registers16bit,
            registers8bit::Registers8bit, Token,
        },
        types_structs::CompiledBytesReference,
    },
    convert_and_push_instructions,
    utils::Either,
};

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
pub(in crate::compiler) fn parse_byte_indexed_addr_and_8bit_reg(
    base_instruction: u8,
    token: &Token,
    high_token: &Token,
    low_token: &Token,
    reg_type: Registers8bit,
    idx_addr_type: IndexedAddressingTypes,
    compiled_bytes: &mut Vec<u8>,
    compiled_bytes_ref: &mut Vec<CompiledBytesReference>,
) -> Result<(), CompilationError> {
    let reg_idx = reg_type.get_as_idx();
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
