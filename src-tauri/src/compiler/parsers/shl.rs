use std::collections::HashMap;

use crate::{
    compiler::{
        compilation_error::CompilationError,
        parsers::utils::push_instruction,
        tokenized_line::TokenizedLine,
        tokens::{registers8bit::Registers8bit, Assembly8086Tokens, Token},
        types_structs::{VariableAddressMap, VariableReferenceMap},
        CompiledBytesReference, CompiledLineLabelRef,
    },
    convert_and_push_instructions,
};

use super::pattern_extractors::parse_high_low_tokens;

#[allow(clippy::too_many_arguments)]
pub(in crate::compiler) fn parse_shl<'a>(
    tokenized_line: &'a TokenizedLine<'a>,
    i: usize,
    is_org_defined: bool,
    label_idx_map: &mut HashMap<String, (Token, usize, bool)>,
    variable_ref_map: &mut VariableReferenceMap,
    variable_abs_address_map: &VariableAddressMap,
    compiled_line_offset_maps: Option<&CompiledLineLabelRef>,
    compiled_bytes: &mut Vec<u8>,
    compiled_bytes_ref: &mut Vec<CompiledBytesReference>,
) -> Result<usize, CompilationError> {
    let instruction_name = "SHL";
    let first_mem16_and_immediate_byte_ins = 0xD1;
    let first_mem8_and_immediate_byte_ins = 0xD0;
    let mem16_cl_ins = 0xD3;
    let mem8_cl_ins = 0xD2;
    let sub_idx_offset = 0x20;

    let token = tokenized_line.get(
        i,
        "This shouldn't happen, Please report this bug to the developer".to_string(),
        None,
    )?;

    let (high_token, low_token) = parse_high_low_tokens(
        tokenized_line,
        i,
        is_org_defined,
        instruction_name,
        label_idx_map,
        variable_ref_map,
        variable_abs_address_map,
        compiled_line_offset_maps,
    )?;

    if low_token.is_none() {
        return Err(CompilationError::error_with_token(
            &high_token,
            &format!(
                "Expected a register or memory address after {} instruction, got none.",
                instruction_name
            ),
        ));
    }
    let low_token = low_token.unwrap();

    convert_andconvert_and_push_bytes_mem_cl_addressing(
        token,
        &high_token,
        &low_token,
        instruction_name,
        MemAndCLAddressingTypeBytesConversionInfo {
            first_mem16_and_immediate_byte_ins,
            first_mem8_and_immediate_byte_ins,
            mem16_cl_ins,
            mem8_cl_ins,
            sub_idx_offset,
        },
        compiled_bytes,
        compiled_bytes_ref,
    )?;

    Ok(tokenized_line.len())
}

struct MemAndCLAddressingTypeBytesConversionInfo {
    first_mem16_and_immediate_byte_ins: u8,
    first_mem8_and_immediate_byte_ins: u8,
    mem16_cl_ins: u8,
    mem8_cl_ins: u8,
    sub_idx_offset: u8,
}

fn convert_andconvert_and_push_bytes_mem_cl_addressing(
    token: &Token,
    high_token: &Token,
    low_token: &Token,
    instruction_name: &str,
    converting_bytes_info: MemAndCLAddressingTypeBytesConversionInfo,
    compiled_bytes: &mut Vec<u8>,
    compiled_bytes_ref: &mut Vec<CompiledBytesReference>,
) -> Result<(), CompilationError> {
    let MemAndCLAddressingTypeBytesConversionInfo {
        first_mem16_and_immediate_byte_ins,
        first_mem8_and_immediate_byte_ins,
        mem16_cl_ins,
        mem8_cl_ins,
        sub_idx_offset,
    } = converting_bytes_info;

    match (&high_token.token_type, &low_token.token_type) {
        ( Assembly8086Tokens::IndexedAddressing(addr_type), Assembly8086Tokens::Number8bit(num) ) => {
            for _ in 0..*num{
            let prefix_instruction_and_mem_addr = addr_type.get_instruction_prefixed_with_offset_and_address(sub_idx_offset);
            convert_and_push_instructions!(
                compiled_bytes,
                compiled_bytes_ref,
                (
                    token => vec![first_mem16_and_immediate_byte_ins],
                    &high_token => prefix_instruction_and_mem_addr
                )
            );
            }
            Ok(())
        }
    ( Assembly8086Tokens::ByteIndexedAddressing(addr_type), Assembly8086Tokens::Number8bit(num) ) => {
            for _ in 0..*num{
                let prefix_instruction_and_mem_addr = addr_type.get_instruction_prefixed_with_offset_and_address(sub_idx_offset);
                convert_and_push_instructions!(
                    compiled_bytes,
                    compiled_bytes_ref,
                    (
                        token => vec![first_mem8_and_immediate_byte_ins],
                        &high_token => prefix_instruction_and_mem_addr
                    )
                );
            }
        Ok(())
    }
    ( Assembly8086Tokens::Register16bit(reg), Assembly8086Tokens::Number8bit(num) ) => {
            let idx = reg.get_index_or_err(&high_token)?;
            let ins_offset = 0xC0 + sub_idx_offset + idx;
            for _ in 0..*num{
                convert_and_push_instructions!(
                    compiled_bytes,
                    compiled_bytes_ref,
                    (
                        token => vec![first_mem16_and_immediate_byte_ins],
                        &high_token => vec![ins_offset]
                    )
                );
            }
            Ok(())
        }
    ( Assembly8086Tokens::Register8bit(reg), Assembly8086Tokens::Number8bit(num) ) => {
            let idx = reg.get_as_idx();
            let ins_offset = 0xC0 + sub_idx_offset + idx;
            for _ in 0..*num{
                convert_and_push_instructions!(
                    compiled_bytes,
                    compiled_bytes_ref,
                    (
                        token => vec![first_mem8_and_immediate_byte_ins],
                        &high_token => vec![ins_offset]
                    )
                );
            }
            Ok(())
        }

    (Assembly8086Tokens::IndexedAddressing(addr_type), Assembly8086Tokens::Register8bit(Registers8bit::CL)) => {
            let  prefix_instruction_and_mem_addr = addr_type.get_instruction_prefixed_with_offset_and_address(sub_idx_offset);
            convert_and_push_instructions!(
                compiled_bytes,
                compiled_bytes_ref,
                    (
                        token => vec![mem16_cl_ins],
                        &high_token => prefix_instruction_and_mem_addr
                    )
            );
            Ok(())
    }

    (Assembly8086Tokens::ByteIndexedAddressing(addr_type), Assembly8086Tokens::Register8bit(Registers8bit::CL)) => {
            let  prefix_instruction_and_mem_addr = addr_type.get_instruction_prefixed_with_offset_and_address(sub_idx_offset);
            convert_and_push_instructions!(
            compiled_bytes,
                compiled_bytes_ref,
            (
                token => vec![mem8_cl_ins],
                &high_token => prefix_instruction_and_mem_addr
            )
        );
            Ok(())
    }

    (Assembly8086Tokens::Register16bit(reg), Assembly8086Tokens::Register8bit(Registers8bit::CL)) => {
            let idx = reg.get_index_or_err(&high_token)?;
            let ins_offset = 0xC0 + sub_idx_offset + idx;
            convert_and_push_instructions!(
                compiled_bytes,
                compiled_bytes_ref,
                (
                    token => vec![mem16_cl_ins],
                    &high_token => vec![ins_offset]
                )
            );
            Ok(())
    }

    (Assembly8086Tokens::Register8bit(reg), Assembly8086Tokens::Register8bit(Registers8bit::CL)) => {
            let idx = reg.get_as_idx();
            let ins_offset = 0xC0 + sub_idx_offset + idx;
            convert_and_push_instructions!(
                compiled_bytes,
                compiled_bytes_ref,
                (
                    token => vec![mem8_cl_ins],
                    &high_token => vec![ins_offset]
                )
            );
            Ok(())
        }

        // TODO: Add better error messages
        _ => {
            return Err(CompilationError::error_with_token(
                &high_token,
                &format!(
                    "Invalid operands. MEM/Reg CL/8bit-number are only supported, check the manual for the correct syntax for {} instruction",
                    instruction_name
                ),
            ))
        }
    }
}

macro_rules! merge_vecs {
    ($($arr:expr),*) =>{
        let mut merged_vec: Vec<u8> = Vec::new();
        merged_vec.push(0x90);
        $(
            for item in $arr {
                merged_vec.extend_from_slice(*item);
            }
        )*
        merged_vec
    }
}

#[cfg(test)]
mod shl_ins_compilation_tests {

    use crate::{compile_and_compare_ins, compiler::compile_str, test_compile};
    use pretty_assertions::assert_eq;

    const first_ins_compiled_bytes: Vec<u8> = merge_vecs!(
        [[0xD1, 0xE3]; 0x10],
        [[0xD1, 0xE6]; 0x10],
        [[0xD1, 0xE0]; 0x10],
        [[0xD1, 0xE5]; 0x10],
        [[0xD1, 0xE1]; 0x10],
        [[0xD1, 0xE0]; 0x10],
        [[0xD1, 0xE5]; 0x10]
    );

    compile_and_compare_ins!(
        test_xchg_reg16bit_and_anything,
        "
        SHL BX, 0x10
        SHL SI, 0x10
        SHL AX, 0x10
        SHL BP, 0x10
        SHL CX, 0x10
        SHL ax, 0x10
        SHL bp, 0x10
        ",
        first_ins_compiled_bytes
    );

    compile_and_compare_ins!(
        test_xchg_8bitreg_and_anything,
        "
        xchg ah, dh
        xchg bh, [0x100]
        ",
        vec![0x86, 0xE6, 0x86, 0x3E, 0x00, 0x01]
    );

    compile_and_compare_ins!(
        test_or_addr_and_anything,
        "
        XCHG [0x100], bp
        XCHG [bp], cx
        XCHG [0x100], bl

        ",
        vec![0x87, 0x2E, 0x00, 0x01, 0x87, 0x4E, 0x00, 0x86, 0x1E, 0x00, 0x01]
    );
}
