use std::vec;

use crate::{
    compiler::{
        compilation_error::CompilationError,
        parsers::utils::{get_token_as_label, is_variable_defined_as_16bit},
        tokens::{registers16bit::Registers16bit, registers8bit::Registers8bit},
        types_structs::VariableAddressMap,
        CompiledBytesReference, TokenizedLine,
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

pub(in crate::compiler) fn parse_mov(
    tokenized_line: &TokenizedLine,
    i: usize,
    compiled_bytes: &mut Vec<u8>,
    compiled_bytes_ref: &mut Vec<CompiledBytesReference>,
    variable_abs_offset_map: Option<&VariableAddressMap>,
    addressing_mode: AddressingMode,
) -> Result<usize, CompilationError> {
    let token = tokenized_line.get(
        i,
        "This shouldn't happen, Please report this".to_string(),
        None,
    )?;
    match addressing_mode {
        // MOV AX..DI, AX..DI
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
                    token => vec![0x8B],
                   &low_token=> vec![ins]
                )
            );
            Ok(i + 3)
        }
        // MOV AL..BH, AL..BH
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
                    token => vec![0x8A],
                   &high_token=> vec![ins]
                )
            );
            Ok(i + 3)
        }
        // MOV AX..DI, 0x00..0xFF | 0x0000..0xFFFF
        AddressingMode::Registers16bitNumber {
            high_token,
            low_token,
            num,
        } => {
            let high_reg_idx = get_idx_from_token(&high_token)?;
            let num = num.get_as_u16();
            let ins = (num & 0xFF) as u8;
            let ins2 = (num >> 8) as u8;
            convert_and_push_instructions!(
                compiled_bytes,
                compiled_bytes_ref,
                (
                    token => vec![0xB8 | high_reg_idx],
                   &low_token=> vec![ins, ins2]
                )
            );
            Ok(tokenized_line.len())
        }
        // MOV AL..BH, 0x00..0xFF
        AddressingMode::Register8bitNumber {
            high_token,
            low_token,
            num,
        } => {
            let high_reg = get_8bit_register(&high_token);
            convert_and_push_instructions!(
                compiled_bytes,
                compiled_bytes_ref,
                (
                    token => vec![0xB0 | high_reg.get_as_idx()],
                   &low_token=> vec![num]
                )
            );

            Ok(tokenized_line.len())
        }
        // MOV AX..DI, 0x100
        AddressingMode::Register16bitAndAddress {
            high_token,
            low_token,
            address_bytes,
            register_type,
        } => match &register_type {
            Registers16bit::AX => {
                convert_and_push_instructions!(
                    compiled_bytes,
                    compiled_bytes_ref,
                    (
                       &high_token=> vec![0xA1],
                       &low_token=> address_bytes.to_vec()
                    )
                );
                Ok(tokenized_line.len())
            }
            _ => {
                let high_reg_idx = get_idx_from_token(&high_token)?;
                convert_and_push_instructions!(
                    compiled_bytes,
                    compiled_bytes_ref,
                    (
                        token => vec![0x8B],
                       &high_token=> vec![0x0E | (high_reg_idx-1) << 3],
                       &low_token=> address_bytes.to_vec()
                    )
                );
                Ok(tokenized_line.len())
            }
        },
        // MOV 0x100, AX..DI
        AddressingMode::AddressAnd16bitRegister {
            high_token,
            low_token,
            address_bytes,
            register_type,
        } => match &register_type {
            Registers16bit::AX => {
                convert_and_push_instructions!(
                    compiled_bytes,
                    compiled_bytes_ref,
                    (
                       &high_token=> vec![0xA3],
                        &low_token => address_bytes.to_vec()
                    )
                );
                Ok(i + 3)
            }
            _ => {
                let reg_idx = get_idx_from_token(&low_token)?;
                convert_and_push_instructions!(
                    compiled_bytes,
                    compiled_bytes_ref,
                    (
                        token => vec![0x89],
                       &high_token=> vec![0x06
                         | reg_idx << 3],
                        &low_token => address_bytes.to_vec()
                    )
                );
                Ok(i + 3)
            }
        },
        // MOV AX..DI, var
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
                    token => vec![0xC7, 0x06],
                   &high_token=> address_bytes.to_vec(),
                    &low_token => vec![(num & 0xFF) as u8, (num >> 8) as u8]
                )
            );
            Ok(tokenized_line.len())
        }
        // MOV AL..BH, 0x100
        AddressingMode::Register8bitAndAddress {
            high_token,
            low_token,
            address_bytes,
            register_type,
        } => match &register_type {
            Registers8bit::AL => {
                convert_and_push_instructions!(
                    compiled_bytes,
                    compiled_bytes_ref,
                    (
                        token => vec![0xA0],
                       &low_token=> address_bytes.to_vec()
                    )
                );
                Ok(tokenized_line.len())
            }
            _ => {
                let reg_idx = get_8bit_register(&high_token).get_as_idx();
                convert_and_push_instructions!(
                    compiled_bytes,
                    compiled_bytes_ref,
                    (
                        token => vec![0x8A],
                       &high_token=> vec![0x06 | reg_idx << 3],
                       &low_token=> address_bytes.to_vec()
                    )
                );
                Ok(tokenized_line.len())
            }
        },
        // MOV 0x100, AL..BH
        AddressingMode::AddressAnd8bitRegister {
            high_token,
            low_token,
            address_bytes,
            register_type,
        } => match &register_type {
            Registers8bit::AL => {
                convert_and_push_instructions!(
                    compiled_bytes,
                    compiled_bytes_ref,
                    (
                        token => vec![0xA2],
                       &high_token=> address_bytes.to_vec()
                    )
                );
                Ok(i + 3)
            }
            _ => {
                let reg_idx = get_8bit_register(&low_token).get_as_idx();
                convert_and_push_instructions!(
                    compiled_bytes,
                    compiled_bytes_ref,
                    (
                        token => vec![0x88],
                       &high_token=> vec![0x06 | reg_idx << 3],
                       &low_token=> address_bytes.to_vec()
                    )
                );
                Ok(i + 3)
            }
        },
        // 0x100, 0x20
        AddressingMode::AddressAnd8bitNumber {
            high_token,
            low_token,
            address_bytes,
            num,
        } => {
            let mov_ins = if is_variable_defined_as_16bit(
                &variable_abs_offset_map,
                get_token_as_label(&high_token),
            ) {
                0xC7
            } else {
                0xC6
            };
            convert_and_push_instructions!(
                compiled_bytes,
                compiled_bytes_ref,
                (
                    token => vec![mov_ins, 0x06],
                   &high_token=> address_bytes.to_vec(),
                   &low_token=> vec![num]
                )
            );
            Ok(tokenized_line.len())
        }
        // MOV AX..DI, [BX] | [BP] | [SI] | [DI] | [BX + SI] | [BX + DI] | [BP + SI] | [BP + DI]
        AddressingMode::Register16bitAndIndexedAddress {
            high_token,
            low_token,
        } => {
            parse_register_16bit_and_indexed_registers_without_offset(
                0x8B,
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
                0x8B,
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
                0x8A,
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
                0x8A,
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
mod tests {
    use crate::{compiler::compile_str, test_compile};

    test_compile!(
        test_compile_str_mov_ax_cx,
        "MOV \t AX, CX",
        |compiled_instructions: &Vec<u8>| {
            assert_eq!(compiled_instructions, &[0x8b, 0xc1]);
        }
    );

    test_compile!(
        test_compile_str_mov_ax_sp,
        "MOV \t AX, SP",
        |compiled_instructions: &Vec<u8>| {
            assert_eq!(compiled_instructions, &[0x8b, 0xc4]);
        }
    );

    test_compile!(
        test_compile_str_mov_bx_dx,
        "MOV \t BX, DX",
        |compiled_instructions: &Vec<u8>| {
            assert_eq!(compiled_instructions, &[0x8b, 0xda]);
        }
    );

    test_compile!(
        test_compile_str_mov_sp_bp,
        "MOV \t SP, BP",
        |compiled_instructions: &Vec<u8>| {
            assert_eq!(compiled_instructions, &[0x8b, 0xe5]);
        }
    );

    // write tests for 16 bit registers but with a instant mov value
    test_compile!(
        test_compile_str_mov_ax_0x1234,
        "MOV \t AX, 0x1234",
        |compiled_instructions: &Vec<u8>| {
            assert_eq!(compiled_instructions, &[0xb8, 0x34, 0x12]);
        }
    );

    test_compile!(
        test_compile_str_mov_bx_0x1234,
        "MOV \t BX, 0x1234",
        |compiled_instructions: &Vec<u8>| {
            assert_eq!(compiled_instructions, &[0xbb, 0x34, 0x12]);
        }
    );

    test_compile!(
        test_compile_str_mov_cx_0x1234,
        "MOV \t CX, 0x1234",
        |compiled_instructions: &Vec<u8>| {
            assert_eq!(compiled_instructions, &[0xb9, 0x34, 0x12]);
        }
    );

    test_compile!(
        test_compile_str_mov_si_0x1234,
        "MOV \t SI, 0x1234",
        |compiled_instructions: &Vec<u8>| {
            assert_eq!(compiled_instructions, &[0xbe, 0x34, 0x12]);
        }
    );

    test_compile!(
        test_compile_str_mov_sp_0x1234,
        "MOV \t SP, 0x1234",
        |compiled_instructions: &Vec<u8>| {
            assert_eq!(compiled_instructions, &[0xbc, 0x34, 0x12]);
        }
    );

    test_compile!(
        test_compile_str_mov_ax_var,
        "
        org 100h 
        .data 
        var dw 10h
        code:
        mov ax, var
        ",
        |compiled_instructions: &Vec<u8>| {
            assert_eq!(
                compiled_instructions,
                &[0xEB, 0x02, 0x10, 0x00, 0xA1, 0x02, 0x01]
            )
        }
    );

    test_compile!(
        test_compile_str_mov_var_ax,
        "
        org 100h 
        .data 
        var dw 10h
        code:
        mov var, ax
        ",
        |compiled_instructions: &Vec<u8>| {
            assert_eq!(
                compiled_instructions,
                &[0xEB, 0x02, 0x10, 0x00, 0xA3, 0x02, 0x01]
            )
        }
    );

    test_compile!(
        test_mov_bx_var,
        "
        org 100h 
        .data 
        var dw 10h
        code:
        mov bx, var
        ",
        |compiled_instructions: &Vec<u8>| {
            assert_eq!(
                compiled_instructions,
                &[0xEB, 0x02, 0x10, 0x00, 0x8B, 0x1E, 0x02, 0x01]
            )
        }
    );

    test_compile!(
        test_mov_var_sp,
        "
        org 100h 
        .data 
        var dw 10h
        code:
        mov var, SP
        ",
        |compiled_instructions: &Vec<u8>| {
            assert_eq!(
                compiled_instructions,
                &[0xEB, 0x02, 0x10, 0x00, 0x89, 0x26, 0x02, 0x01]
            )
        }
    );

    test_compile!(
        test_mov_var_0x01,
        "
        org 100h 
        .data 
        var dw 10h
        code:
        mov var, 0x01
        ",
        |compiled_instructions: &Vec<u8>| {
            assert_eq!(
                compiled_instructions,
                &[0xEB, 0x02, 0x10, 0x00, 0xC7, 0x06, 0x02, 0x01, 0x01]
            )
        }
    );

    test_compile!(
        test_mov_ax_offset_var,
        "
        org 100h 
        .data 
        var dw 1_0h
        code:
        mov ax, offset var
        ",
        |compiled_instructions: &Vec<u8>| {
            assert_eq!(
                compiled_instructions,
                &[0xEB, 0x02, 0x10, 0x00, 0xB8, 0x02, 0x01]
            )
        }
    );

    test_compile!(
        test_mov_bx_bp,
        "mov bx, [bp]",
        |compiled_instructions: &Vec<u8>| {
            assert_eq!(compiled_instructions, &[0x8B, 0x5E, 0x00]);
        }
    );

    test_compile!(
        test_mov_ax_ref_bx_plus_si,
        "
        mov ax, [bx + si]
        ",
        |compiled_instructions: &Vec<u8>| { assert_eq!(compiled_instructions, &[0x8B, 0x00]) }
    );

    test_compile!(
        test_mov_ax_ref_bx_plus_di,
        "
        mov ax, [bx + di + 0x20
        ",
        |compiled_instructions: &Vec<u8>| {
            assert_eq!(compiled_instructions, &[0x8B, 0x41, 0x20])
        }
    );

    test_compile!(
        test_mov_ax_ref_0x3020,
        "
        mov AX, [0x3020]
        ",
        |compiled_instructions: &Vec<u8>| {
            assert_eq!(compiled_instructions, &[0xA1, 0x20, 0x30])
        }
    );

    test_compile!(
        test_mov_ax_ref_0x30,
        "
        mov AX, [0x30]
        ",
        |compiled_instructions: &Vec<u8>| {
            assert_eq!(compiled_instructions, &[0xA1, 0x30, 0x00])
        }
    );

    test_compile!(
        test_mov_bp_ref_0x1000,
        "
        mov bp, [0x1000]
        ",
        |compiled_instructions: &Vec<u8>| {
            assert_eq!(compiled_instructions, &[0x8B, 0x2E, 0x00, 0x10])
        }
    );

    test_compile!(
        test_mov_cx_label_without_org,
        "
        code:
        mov cx, code

        ",
        |compiled_instructions: &Vec<u8>| {
            assert_eq!(compiled_instructions, &[0xB9, 0x00, 0x00])
        }
    );

    test_compile!(
        test_mov_cx_label_with_org,
        "
        org 100h
        code:
        mov cx, code

        ",
        |compiled_instructions: &Vec<u8>| {
            assert_eq!(compiled_instructions, &[0xB9, 0x00, 0x01])
        }
    );

    test_compile!(
        test_mov_bp_bin_number,
        "MOV BP, 01000b",
        |compiled_instructions: &Vec<u8>| {
            assert_eq!(compiled_instructions, &[0xBD, 0b1000, 0x00])
        }
    );

    test_compile!(
        mov_sp_0x10_0x20_0x30,
        "MOV SP, 0x10 - 0x20 + 0x30",
        |instructions: &Vec<u8>| {
            assert_eq!(instructions, &[0xBC, 0x20, 0x00]);
        }
    );
}

#[cfg(test)]
mod tests8bit {
    use crate::{compiler::compile_str, test_compile};

    test_compile!(
        test_compile_str_mov_al_cl,
        "MOV \t AL, CL",
        |compiled_instructions: &Vec<u8>| {
            assert_eq!(compiled_instructions, &[0x8a, 0xc1]);
        }
    );

    test_compile!(
        test_compile_str_mov_bl_bh,
        "MOV \t BL, BH",
        |compiled_instructions: &Vec<u8>| {
            assert_eq!(compiled_instructions, &[0x8a, 0xdf]);
        }
    );

    test_compile!(
        test_compile_str_mov_dl_ah,
        "MOV DL, BH",
        |compiled_instructions: &Vec<u8>| {
            assert_eq!(compiled_instructions, &[0x8a, 0xd7]);
        }
    );

    test_compile!(
        test_mov_al_0x08,
        "MOV AL, 0x08",
        |compiled_instructions: &Vec<u8>| {
            assert_eq!(compiled_instructions, &[0xb0, 0x08]);
        }
    );

    test_compile!(
        test_mov_bl_10,
        "MOV BL, 10",
        |compiled_instructions: &Vec<u8>| {
            assert_eq!(compiled_instructions, &[0xb3, 10]);
        }
    );

    test_compile!(
        test_mov_ch_0x08,
        "MOV CH, 0x08",
        |compiled_instructions: &Vec<u8>| {
            assert_eq!(compiled_instructions, &[0xb5, 0x08]);
        }
    );

    test_compile!(
        test_mov_dh_var,
        "
        org 100h 
        .data 
        var db 10h
        code: 
        MOV DH, var
        ",
        |compiled_instructions: &Vec<u8>| {
            assert_eq!(
                compiled_instructions,
                &[0xEB, 0x01, 0x10, 0x8A, 0x36, 0x02, 0x01]
            );
        }
    );

    test_compile!(
        test_mov_var_bh,
        "
        org 100h 
        .data 
        var db 10h, 20h, 0x30
        code: 
        MOV var, BH
        ",
        |compiled_instructions: &Vec<u8>| {
            assert_eq!(
                compiled_instructions,
                &[0xEB, 0x03, 0x10, 0x20, 0x30, 0x88, 0x3E, 0x02, 0x01]
            );
        }
    );

    test_compile!(
        test_mov_var_0x08,
        "
        org 100h 
        .data 
        var db 10h
        var2 db 0x20
        code: 
        MOV var, 0x08
        ",
        |compiled_instructions: &Vec<u8>| {
            assert_eq!(
                compiled_instructions,
                &[0xEB, 0x02, 0x10, 0x20, 0xC6, 0x06, 0x02, 0x01, 0x08]
            );
        }
    );

    test_compile!(
        test_mov_al_ref_0x3020,
        "
        mov AL, [0x3020]
        ",
        |compiled_instructions: &Vec<u8>| {
            assert_eq!(compiled_instructions, &[0xA0, 0x20, 0x30])
        }
    );

    test_compile!(
        test_mov_ah_ref_0x30,
        "
        mov AH, [0x30]
        ",
        |compiled_instructions: &Vec<u8>| {
            assert_eq!(compiled_instructions, &[0x8A, 0x26, 0x30, 0x00])
        }
    );

    test_compile!(
        test_mov_bl_ref_0x1000,
        "
        mov bl, [0x1000]
        ",
        |compiled_instructions: &Vec<u8>| {
            assert_eq!(compiled_instructions, &[0x8A, 0x1E, 0x00, 0x10])
        }
    );

    test_compile!(
        test_cl_bx_di_ref_0x10,
        "mov cl, [bx + di + 0x10]",
        |compiled_instructions: &Vec<u8>| {
            assert_eq!(compiled_instructions, &[0x8A, 0x49, 0x10])
        }
    );

    test_compile!(
        mov_dh_bin_num,
        "MOV DH, 01000b",
        |compiled_instructions: &Vec<u8>| { assert_eq!(compiled_instructions, &[0xB6, 0b1000]) }
    );

    test_compile!(
        mov_ah_0x10_0x20_0x30,
        "MOV Ah, 0x10 - 0x20 + 0x30 - 0x20",
        |instructions: &Vec<u8>| {
            assert_eq!(instructions, &[0xB4, 0x00]);
        }
    );
}
