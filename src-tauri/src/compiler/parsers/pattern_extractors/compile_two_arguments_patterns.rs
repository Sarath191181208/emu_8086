use crate::{
    compiler::{
        compilation_error::CompilationError,
        parsers::utils::{
            get_as_0x00_0x3f_pattern, get_as_0x40_0x7f_pattern, get_as_0x80_0xbf_pattern,
            get_idx_from_token, get_index_addr_as_idx, push_instruction,
        },
        tokens::{
            indexed_addressing_types::IndexedAddressingTypes, Assembly8086Tokens, SignedU16, Token,
        },
        types_structs::CompiledBytesReference,
    },
    convert_and_push_instructions,
    utils::Either,
};

pub(crate) fn parse_register_16bit_and_indexed_registers_without_offset(
    base_instruction: u8,
    token: &Token,
    high_token: &Token,
    low_token: &Token,
    compiled_bytes: &mut Vec<u8>,
    compiled_bytes_ref: &mut Vec<CompiledBytesReference>,
) -> Result<(), CompilationError> {
    let high_reg_idx = get_idx_from_token(high_token)?;
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

pub(crate) fn parse_register_16bit_and_indexed_registers_with_offset(
    base_instruction: u8,
    token: &Token,
    high_token: &Token,
    low_token: &Token,
    offset: &SignedU16,
    compiled_bytes: &mut Vec<u8>,
    compiled_bytes_ref: &mut Vec<CompiledBytesReference>,
) -> Result<(), CompilationError> {
    let offset = offset.as_either_u8_or_u16(low_token)?;
    let high_reg_idx = get_idx_from_token(high_token)?;
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
