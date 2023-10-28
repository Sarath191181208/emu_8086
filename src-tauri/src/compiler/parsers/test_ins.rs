use crate::{
    compiler::{
        compilation_error::CompilationError, parsers::utils::get_idx_from_reg,
        tokenized_line::TokenizedLine, CompiledBytesReference,
    },
    convert_and_push_instructions,
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
        AddressingMode::Registers16bit {
            high_token,
            low_token,
        } => {
            let high_reg_idx = get_idx_from_token(&high_token)?;
            let low_reg_idx = get_idx_from_token(&low_token)?;
            let ins = get_as_0xc0_0xff_pattern(high_reg_idx, low_reg_idx);
            convert_and_push_instructions!(
                compiled_bytes,
                compiled_bytes_ref,
                (
                    token => vec![0x85],
                    &high_token => vec![ins]
                )
            );
            Ok(i + 3)
        }
        AddressingMode::Registers8bit {
            high_token,
            low_token,
        } => {
            let high_reg = get_8bit_register(&high_token);
            let low_reg = get_8bit_register(&low_token);
            let ins = get_as_0xc0_0xff_pattern(high_reg.get_as_idx(), low_reg.get_as_idx());
            convert_and_push_instructions!(
                compiled_bytes,
                compiled_bytes_ref,
                (
                    token => vec![0x84],
                    &high_token => vec![ins]
                )
            );
            Ok(i + 3)
        }
        AddressingMode::Registers16bitNumber {
            high_token,
            low_token,
            num,
        } => {
            let high_reg_idx = get_idx_from_token(&high_token)?;
            let is_ax = high_reg_idx == 0;
            if is_ax {
                convert_and_push_instructions!(
                    compiled_bytes,
                    compiled_bytes_ref,
                    (
                        token => vec![0xA9],
                        &low_token => num.get_as_u16().to_le_bytes().to_vec()
                    )
                );
            } else {
                convert_and_push_instructions!(
                    compiled_bytes,
                    compiled_bytes_ref,
                    (
                        token => vec![0xF7],
                        &high_token => vec![0xC0 | high_reg_idx],
                        &low_token => num.get_as_u16().to_le_bytes().to_vec()
                    )
                );
            }

            Ok(tokenized_line.len())
        }
        AddressingMode::Register8bitNumber {
            high_token,
            low_token,
            num: number,
        } => {
            let high_reg = get_8bit_register(&high_token);
            let is_al = high_reg.get_as_idx() == 0;
            if is_al {
                convert_and_push_instructions!(
                    compiled_bytes,
                    compiled_bytes_ref,
                    (
                        token => vec![0xA8],
                        &low_token => vec![number]
                    )
                );
            } else {
                convert_and_push_instructions!(
                    compiled_bytes,
                    compiled_bytes_ref,
                    (
                        token => vec![0xF6],
                        &high_token => vec![0xC0 | high_reg.get_as_idx()],
                        &low_token => vec![number]
                    )
                );
            }
            Ok(tokenized_line.len())
        }
        AddressingMode::AddressAnd16bitRegister {
            high_token,
            low_token,
            address_bytes,
            register_type,
        }
        | AddressingMode::Register16bitAndAddress {
            high_token,
            low_token,
            address_bytes,
            register_type,
        } => {
            let reg_idx = get_idx_from_reg(&high_token, &register_type)?;
            convert_and_push_instructions!(
                compiled_bytes,
                compiled_bytes_ref,
                (
                    token => vec![0x85],
                    &high_token => vec![0x06 | reg_idx << 3],
                    &low_token => address_bytes.to_vec()
                )
            );
            Ok(tokenized_line.len())
        }
        AddressingMode::AddressAnd16bitNumber {
            high_token,
            low_token,
            address_bytes,
            num,
        } => {
            convert_and_push_instructions!(
                compiled_bytes,
                compiled_bytes_ref,
                (
                    token => vec![0xF7, 0x06],
                   &high_token=> address_bytes.to_vec(),
                    &low_token => num.to_le_bytes().to_vec()
                )
            );
            Ok(tokenized_line.len())
        }
        AddressingMode::AddressAnd8bitRegister {
            high_token,
            low_token,
            address_bytes,
            register_type,
        }
        | AddressingMode::Register8bitAndAddress {
            high_token,
            low_token,
            address_bytes,
            register_type,
        } => {
            let reg_idx = register_type.get_as_idx();
            convert_and_push_instructions!(
                compiled_bytes,
                compiled_bytes_ref,
                (
                    token => vec![0x84],
                    &high_token => vec![0x06 | reg_idx << 3],
                    &low_token => address_bytes.to_vec()
                )
            );
            Ok(tokenized_line.len())
        }
        AddressingMode::AddressAnd8bitNumber {
            high_token,
            low_token,
            address_bytes,
            num,
        } => {
            convert_and_push_instructions!(
                compiled_bytes,
                compiled_bytes_ref,
                (
                    token => vec![0xF7, 0x06],
                   &high_token=> address_bytes.to_vec(),
                   &low_token=> (num as u16).to_le_bytes().to_vec()
                )
            );
            Ok(tokenized_line.len())
        }

        AddressingMode::ByteAddressAnd8bitNumber {
            high_token,
            low_token,
            address_bytes,
            num,
        } => {
            convert_and_push_instructions!(
                compiled_bytes,
                compiled_bytes_ref,
                (
                    token => vec![0xF6, 0x06],
                   &high_token=> address_bytes.to_vec(),
                   &low_token=> vec![num]
                )
            );
            Ok(tokenized_line.len())
        }

        AddressingMode::Register16bitAndIndexedAddress {
            high_token,
            low_token,
        } => {
            parse_register_16bit_and_indexed_registers_without_offset(
                0x85,
                token,
                &high_token,
                &low_token,
                compiled_bytes,
                compiled_bytes_ref,
            )?;
            Ok(tokenized_line.len())
        }
        AddressingMode::Register16bitAndIndexedAddressWithOffset {
            high_token,
            low_token,
            offset,
        } => {
            parse_register_16bit_and_indexed_registers_with_offset(
                0x85,
                token,
                &high_token,
                &low_token,
                &offset,
                compiled_bytes,
                compiled_bytes_ref,
            )?;
            Ok(tokenized_line.len())
        }
        AddressingMode::Register8bitAndIndexedAddress {
            high_token,
            low_token,
            register_type,
        } => {
            parse_register_8bit_and_indexed_registers_without_offset(
                0x84,
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
            parse_register_8bit_and_indexed_registers_with_offset(
                0x84,
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
    }
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
}
