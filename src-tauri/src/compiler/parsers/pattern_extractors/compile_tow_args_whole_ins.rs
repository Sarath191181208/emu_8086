use crate::{
    compiler::{
        compilation_error::CompilationError,
        parsers::utils::{get_8bit_register, get_idx_from_token, push_instruction},
        tokenized_line::TokenizedLine,
        types_structs::CompiledBytesReference,
    },
    convert_and_push_instructions,
    utils::Either,
};

use super::{
    compile_first_ins_reg_pattern::{
        parse_16bitreg_first_addr_mode, parse_8bitreg_first_addr_mode,
    },
    compile_two_arguments_patterns::parse_indexed_addr_and_reg,
    AddressingMode,
};

pub(in crate::compiler) struct CompilingBytesForInstruction {
    pub reg_16bit_and_anything_ins: u8,
    pub reg_8bit_and_anything_ins: u8,
    pub indexed_addressing_and_anyting_ins: u8,
    pub addr_and_8bit_reg: u8,

    pub al_and_num_ins: Option<u8>,
    pub ax_and_num_ins: Option<u8>,

    pub reg16bit_and_16bit_num: u8,
    pub reg16bit_and_8bit_num: Option<u8>,
    pub reg8bit_and_num: u8,
    pub reg_num_sub_ins: u8,

    pub addr16bit_and_16bit_num: u8,
    pub addr16bit_and_8bit_num: Option<u8>,
    pub addr8bit_and_num: u8,
    pub addr_num_sub_ins: u8,
}
pub(in crate::compiler) fn compile_two_args_whole_ins(
    tokenized_line: &TokenizedLine,
    i: usize,
    compiling_bytes_for_instruction: CompilingBytesForInstruction,
    compiled_bytes: &mut Vec<u8>,
    compiled_bytes_ref: &mut Vec<CompiledBytesReference>,
    addressing_mode: AddressingMode,
) -> Result<usize, CompilationError> {
    let token = tokenized_line.get(
        i,
        "This shouldn't happen, Please report this".to_string(),
        None,
    )?;

    let CompilingBytesForInstruction {
        reg_16bit_and_anything_ins,
        reg_8bit_and_anything_ins,
        indexed_addressing_and_anyting_ins,
        addr_and_8bit_reg,

        al_and_num_ins,
        ax_and_num_ins,
        reg16bit_and_16bit_num,
        reg16bit_and_8bit_num,
        reg8bit_and_num,
        reg_num_sub_ins,

        addr16bit_and_16bit_num,
        addr16bit_and_8bit_num,
        addr8bit_and_num,
        addr_num_sub_ins,
    } = compiling_bytes_for_instruction;

    match addressing_mode {
        AddressingMode::Registers16bitNumber {
            high_token,
            low_token,
            num,
        } => {
            let high_reg_idx = get_idx_from_token(&high_token)?;
            let is_ax = high_reg_idx == 0;
            if is_ax && ax_and_num_ins.is_some() {
                convert_and_push_instructions!(
                    compiled_bytes,
                    compiled_bytes_ref,
                    (
                        token => vec![ax_and_num_ins.unwrap()],
                        &low_token => num.get_as_u16().to_le_bytes().to_vec()
                    )
                );
            } else {
                let is_8_bit_ins = if reg16bit_and_8bit_num.is_some() {
                    match num {
                        Either::Left(num_u8) => num_u8 <= 0x7F,
                        Either::Right(_) => false,
                    }
                } else {
                    false
                };
                let and_ins = if is_8_bit_ins {
                    vec![reg16bit_and_8bit_num.unwrap()]
                } else {
                    vec![reg16bit_and_16bit_num]
                };
                let num_vec = if is_8_bit_ins {
                    num.get_as_bytes()
                } else {
                    num.get_as_u16().to_le_bytes().to_vec()
                };
                convert_and_push_instructions!(
                    compiled_bytes,
                    compiled_bytes_ref,
                    (
                        token => and_ins,
                        &high_token => vec![reg_num_sub_ins | high_reg_idx],
                        &low_token => num_vec
                    )
                );
            }

            Ok(tokenized_line.len())
        }

        AddressingMode::Register8bitNumber {
            high_token,
            low_token,
            num,
        } => {
            let high_reg = get_8bit_register(&high_token);
            let is_al = high_reg.get_as_idx() == 0;
            if is_al && al_and_num_ins.is_some() {
                convert_and_push_instructions!(
                    compiled_bytes,
                    compiled_bytes_ref,
                    (
                        token => vec![al_and_num_ins.unwrap()],
                        &low_token => vec![num]
                    )
                );
            } else {
                convert_and_push_instructions!(
                    compiled_bytes,
                    compiled_bytes_ref,
                    (
                        token => vec![reg8bit_and_num],
                        &high_token => vec![reg_num_sub_ins | high_reg.get_as_idx()],
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
                    token => vec![addr16bit_and_16bit_num, addr_num_sub_ins],
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
                    token => vec![addr_and_8bit_reg],
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
            let is_ins_u8 = addr16bit_and_8bit_num.is_some() && (num <= 0x7F);
            let ins = match is_ins_u8 {
                true => addr16bit_and_8bit_num.unwrap(),
                false => addr16bit_and_16bit_num,
            };
            let num = match is_ins_u8 {
                true => num.to_le_bytes().to_vec(),
                false => (num as u16).to_le_bytes().to_vec(),
            };
            convert_and_push_instructions!(
                compiled_bytes,
                compiled_bytes_ref,
                (
                    token => vec![ins, addr_num_sub_ins],
                   &high_token=> address_bytes.to_vec(),
                   &low_token=> num
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
                    token => vec![addr8bit_and_num, addr_num_sub_ins],
                   &high_token=> address_bytes.to_vec(),
                   &low_token=> num.to_le_bytes().to_vec()
                )
            );
            Ok(tokenized_line.len())
        }

        AddressingMode::Registers16bit {
            high_token: _,
            low_token: _,
        }
        | AddressingMode::Register16bitAndIndexedAddressing {
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
    }
}
