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
    let reg_8bit_and_anything_ins = 0x22;
    let indexed_addressing_and_anyting_ins = 0x21;

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
        AddressingMode::Registers8bit {
            high_token: _,
            low_token: _,
        }
        | AddressingMode::Register8bitAndIndexedAddress {
            high_token: _,
            low_token: _,
            register_type: _,
        }
        | AddressingMode::Register8bitAndIndexedAddressWithOffset {
            high_token: _,
            low_token: _,
            register_type: _,
            offset: _,
        }
        | AddressingMode::Register8bitAndAddress {
            high_token: _,
            low_token: _,
            address_bytes: _,
            register_type: _,
        } => parse_8bitreg_first_addr_mode(
            i,
            addressing_mode,
            reg_8bit_and_anything_ins,
            tokenized_line,
            token,
            compiled_bytes,
            compiled_bytes_ref,
        ),

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
                        token => vec![0x25],
                        &low_token => num.get_as_u16().to_le_bytes().to_vec()
                    )
                );
            } else {
                let and_ins = match num {
                    Either::Left(_) => vec![0x83],  // for 8 bit numbers
                    Either::Right(_) => vec![0x81], // for 16 bit numbers
                };
                let num_vec = match num {
                    Either::Left(val) => vec![val],
                    Either::Right(val) => val.to_le_bytes().to_vec(),
                };
                convert_and_push_instructions!(
                    compiled_bytes,
                    compiled_bytes_ref,
                    (
                        token => and_ins,
                        &high_token => vec![0xE0 | high_reg_idx],
                        &low_token => num_vec
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
        } => {
            let reg_idx = get_idx_from_reg(&high_token, &register_type)?;
            convert_and_push_instructions!(
                compiled_bytes,
                compiled_bytes_ref,
                (
                    token => vec![indexed_addressing_and_anyting_ins],
                    &high_token => vec![0x06 | reg_idx << 3],
                    &low_token => address_bytes.to_vec()
                )
            );
            Ok(tokenized_line.len())
        }

        AddressingMode::Register8bitNumber {
            high_token,
            low_token,
            num,
        } => {
            let high_reg = get_8bit_register(&high_token);
            let is_al = high_reg.get_as_idx() == 0;
            if is_al {
                convert_and_push_instructions!(
                    compiled_bytes,
                    compiled_bytes_ref,
                    (
                        token => vec![0x24],
                        &low_token => vec![num]
                    )
                );
            } else {
                convert_and_push_instructions!(
                    compiled_bytes,
                    compiled_bytes_ref,
                    (
                        token => vec![0x80],
                        &high_token => vec![0xE0 | high_reg.get_as_idx()],
                        &low_token => vec![num]
                    )
                );
            }
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
                    token => vec![0x81, 0x26],
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
        } => {
            let reg_idx = register_type.get_as_idx();
            convert_and_push_instructions!(
                compiled_bytes,
                compiled_bytes_ref,
                (
                    token => vec![0x20],
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
                    token => vec![0x83, 0x26],
                   &high_token=> address_bytes.to_vec(),
                   &low_token=> num.to_le_bytes().to_vec()
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
                    token => vec![0x80, 0x26],
                   &high_token=> address_bytes.to_vec(),
                   &low_token=> num.to_le_bytes().to_vec()
                )
            );
            Ok(tokenized_line.len())
        }
    }
}

#[cfg(test)]
mod and_ins_compilation_tests {
    use crate::{compile_and_compare_ins, compiler::compile_str, test_compile};
    use pretty_assertions::assert_eq;

    compile_and_compare_ins!(
        test_register_as_first_ins,
        "
        and cx, bx
        and dx, [bx]
        and sp, [0x1234]
        and di, [0x12 + si]
        and ax, [0x1234 + bx]
        ",
        vec![
            0x23, 0xCB, 0x23, 0x17, 0x23, 0x26, 0x34, 0x12, 0x23, 0x7C, 0x12, 0x23, 0x87, 0x34,
            0x12
        ]
    );

    compile_and_compare_ins!(
        test_register_8bit_as_first_ins,
        "
        and cl, bl
        and dl, [bx]
        and bh, [0x1234]
        and dh, [0x12 + si]
        and al, [0x1234 + bx]
        ",
        vec![
            0x22, 0xCB, 0x22, 0x17, 0x22, 0x3E, 0x34, 0x12, 0x22, 0x74, 0x12, 0x22, 0x87, 0x34,
            0x12
        ]
    );

    compile_and_compare_ins!(
        test_reg_and_number,
        "
    and ax, 0x10
    and bx, 0x10
    and ax, 0x1234
    and si, 0x245

    and al, 0x12
    and ch, 0x24
    ",
        vec![
            0x25, 0x10, 0x00, 0x83, 0xE3, 0x10, 0x25, 0x34, 0x12, 0x81, 0xE6, 0x45, 0x02, 0x24,
            0x12, 0x80, 0xE5, 0x24
        ]
    );

    compile_and_compare_ins!(
        test_addr_and_reg_16bit,
        "
    and [0x1234], ax
    and [0x1234], bx

    and [0x123], al 
    and [0x123], cl
    ",
        vec![
            0x21, 0x06, 0x34, 0x12, 0x21, 0x1E, 0x34, 0x12, 0x20, 0x06, 0x23, 0x01, 0x20, 0x0E,
            0x23, 0x01
        ]
    );

    compile_and_compare_ins!(
        test_addr_and_num,
        "
        and [0x234], 0x123
        and [0x234], 0x100
        and w.[0x1234], 0x12
        and b.[0x1234], 0x12
        ",
        vec![
            0x81, 0x26, 0x34, 0x02, 0x23, 0x01, 0x81, 0x26, 0x34, 0x02, 0x00, 0x01, 0x83, 0x26,
            0x34, 0x12, 0x12, 0x80, 0x26, 0x34, 0x12, 0x12
        ]
    );
}
