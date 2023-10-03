use crate::{
    compiler::{
        compilation_error::CompilationError,
        parsers::utils::{get_idx_from_reg, get_token_as_label, is_variable_defined_as_16bit},
        tokenized_line::TokenizedLine,
        types_structs::{CompiledBytesReference, VariableAddressMap, VariableReferenceMap},
    },
    convert_and_push_instructions,
    utils::Either,
};

use super::{
    pattern_extractors::{parse_two_arguments_line, AddressingMode, compile_two_arguments_patterns::{parse_register_16bit_and_indexed_registers_without_offset, parse_register_16bit_and_indexed_registers_with_offset}},
    utils::{get_8bit_register, get_as_0xc0_0xff_pattern, get_idx_from_token, push_instruction},
};

pub(in crate::compiler) fn parse_sub(
    tokenized_line: &TokenizedLine,
    i: usize,
    compiled_bytes: &mut Vec<u8>,
    compiled_bytes_ref: &mut Vec<CompiledBytesReference>,
    variable_ref_map: &mut VariableReferenceMap,
    variable_abs_offset_map: Option<&VariableAddressMap>,
) -> Result<usize, CompilationError> {
    let token = tokenized_line.get(
        i,
        "This shouldn't happen, Please report this".to_string(),
        None,
    )?;
    match parse_two_arguments_line(
        tokenized_line,
        i,
        "SUB",
        variable_ref_map,
        variable_abs_offset_map.unwrap_or(&VariableAddressMap::new()),
    )? {
        AddressingMode::Registers16bit {
            high_token,
            low_token,
        } => {
            let high_reg_idx = get_idx_from_token(&high_token)?;
            let low_reg_idx = get_idx_from_token(&low_token)?;
            let ins = get_as_0xc0_0xff_pattern(high_reg_idx, low_reg_idx);
            push_instruction(compiled_bytes, vec![0x2B], token, compiled_bytes_ref);
            push_instruction(compiled_bytes, vec![ins], &low_token, compiled_bytes_ref);
            Ok(i + 3)
        }
        AddressingMode::Registers8bit {
            high_token,
            low_token,
        } => {
            let high_reg = get_8bit_register(&high_token);
            let low_reg = get_8bit_register(&low_token);
            let ins = get_as_0xc0_0xff_pattern(high_reg.get_as_idx(), low_reg.get_as_idx());
            push_instruction(compiled_bytes, vec![0x2A], token, compiled_bytes_ref);
            push_instruction(compiled_bytes, vec![ins], &low_token, compiled_bytes_ref);
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
                let number = num.get_as_u16();
                push_instruction(compiled_bytes, vec![0x2D], token, compiled_bytes_ref);
                push_instruction(
                    compiled_bytes,
                    vec![(number & 0xFF) as u8, (number >> 8) as u8],
                    &low_token,
                    compiled_bytes_ref,
                );
            } else {
                let number = num.get_as_u16();
                let sub_ins = match num {
                    Either::Right(_) => 0x81,
                    Either::Left(_) => 0x83,
                };
                let data_ins = match num {
                    Either::Right(_) => vec![(number & 0xFF) as u8, (number >> 8) as u8],
                    Either::Left(x) => vec![x],
                };

                convert_and_push_instructions!(
                    compiled_bytes,
                    compiled_bytes_ref,
                    (
                        token => vec![sub_ins],
                       &high_token=> vec![0xE8 + high_reg_idx],
                       &low_token=> data_ins
                    )
                );
            }

            Ok(i + 3)
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
                        token => vec![0x2C],
                       &low_token=> vec![number]
                    )
                );
            } else {
                convert_and_push_instructions!(
                    compiled_bytes,
                    compiled_bytes_ref,
                    (
                        token => vec![0x80],
                       &high_token=> vec![0xE8 + high_reg.get_as_idx()],
                       &low_token=> vec![number]
                    )
                );
            }
            Ok(i + 3)
        }
        AddressingMode::Register16bitAndAddress {
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
                    token => vec![0x2B],
                   &high_token=> vec![0x06 | reg_idx << 3],
                   &low_token=> address_bytes.to_vec()
                )
            );
            Ok(i + 3)
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
                    token => vec![0x29],
                    &low_token => vec![0x06 | reg_idx << 3],
                   &high_token=> address_bytes.to_vec()
                )
            );
            Ok(i + 3)
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
                    token => vec![0x81, 0x2E],
                   &high_token=> address_bytes.to_vec(),
                    &low_token => vec![(num & 0xFF) as u8, (num >> 8) as u8]
                )
            );
            Ok(i + 3)
        }
        AddressingMode::Register8bitAndAddress {
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
                    token => vec![0x2A],
                   &high_token=> vec![0x06 | reg_idx << 3],
                   &low_token=> address_bytes.to_vec()
                )
            );
            Ok(i + 3)
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
                    token => vec![0x28],
                   &high_token=> vec![0x06 | reg_idx << 3],
                   &low_token=> address_bytes.to_vec()
                )
            );
            Ok(i + 3)
        }
        AddressingMode::AddressAnd8bitNumber {
            high_token,
            low_token,
            address_bytes,
            num,
        } => {
            let add_ins = if is_variable_defined_as_16bit(
                &variable_abs_offset_map,
                get_token_as_label(&high_token),
            ) {
                0x83
            } else {
                0x80
            };

            convert_and_push_instructions!(
                compiled_bytes,
                compiled_bytes_ref,
                (
                    token => vec![add_ins, 0x2E],
                   &high_token=> address_bytes.to_vec(),
                   &low_token=> vec![num]
                )
            );
            Ok(i + 3)
        }
        AddressingMode::Register16bitAndIndexedAddress {
            high_token,
            low_token,
        } => {
            parse_register_16bit_and_indexed_registers_without_offset(
                0x2B,
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
                0x2B,
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
mod tests16bit {
    use crate::{compiler::compile_str, test_compile};

    test_compile!(add_ax_sp, "SUB AX, SP", |instructions: &Vec<u8>| {
        assert_eq!(instructions, &[0x2b, 0xC4]);
    });

    test_compile!(add_sp_ax, "SUB SP, AX", |instructions: &Vec<u8>| {
        assert_eq!(instructions, &[0x2b, 0xE0]);
    });

    // sub ax, 0x1234
    test_compile!(sub_ax_0x1234, "SUB AX, 0x1234", |instructions: &Vec<u8>| {
        assert_eq!(instructions, &[0x2D, 0x34, 0x12]);
    });

    // sub dx, 0x0f0f
    test_compile!(sub_dx_0x0f0f, "SUB DX, 0x0f0f", |instructions: &Vec<u8>| {
        assert_eq!(instructions, &[0x81, 0xEA, 0x0F, 0x0F]);
    });

    // sub di, 0xff
    test_compile!(sub_di_0xff, "SUB DI, 0xff", |instructions: &Vec<u8>| {
        assert_eq!(instructions, &[0x83, 0xEF, 0xff]);
    });

    // sub cx, var
    test_compile!(
        sub_cx_var,
        "
    org 100h 
    .data 
    var dw 0x1234
    code: 
    SUB CX, var",
        |instructions: &Vec<u8>| {
            assert_eq!(
                instructions,
                &[0xEB, 0x02, 0x34, 0x12, 0x2B, 0x0E, 0x02, 0x01]
            );
        }
    );

    // sub var, sp
    test_compile!(
        sub_var_sp,
        "
    org 100h 
    .data 
    var dw 0x1234
    code: 
    SUB var, SP",
        |instructions: &Vec<u8>| {
            assert_eq!(
                instructions,
                &[0xEB, 0x02, 0x34, 0x12, 0x29, 0x26, 0x02, 0x01]
            );
        }
    );

    // sub var, 0x12
    test_compile!(
        sub_var_0x12,
        "
    org 100h
    .data
    var dw 0x1234
    code:
    SUB var, 0x12",
        |instructions: &Vec<u8>| {
            assert_eq!(
                instructions,
                &[0xEB, 0x02, 0x34, 0x12, 0x83, 0x2E, 0x02, 0x01, 0x12]
            );
        }
    );

    // sub var, 0x1234
    test_compile!(
        sub_var_0x1234,
        "
    org 100h
    .data 
    var dw 0x1234 
    code: 
    SUB var, 0x1234",
        |instructions: &Vec<u8>| {
            assert_eq!(
                instructions,
                &[0xEB, 0x02, 0x34, 0x12, 0x81, 0x2E, 0x02, 0x01, 0x34, 0x12]
            );
        }
    );

    test_compile!(sub_dx_bp_di, "SUB DX, [BP] + DI]]", |instructions: &Vec<
        u8,
    >| {
        assert_eq!(instructions, &[0x2B, 0x13]);
    });

    test_compile!(
        sub_dx_bp_di_offset,
        "SUB DX, [BP] + [0x10] + DI + 0x20 + 0x30",
        |instructions: &Vec<u8>| {
            assert_eq!(instructions, &[0x2B, 0x53, 0x60]);
        }
    );
}

#[cfg(test)]
mod tests8bit {
    use crate::{compiler::compile_str, test_compile};

    test_compile!(sub_al_0x12, "SUB AL, 0x12", |instructions: &Vec<u8>| {
        assert_eq!(instructions, &[0x2C, 0x12]);
    });

    test_compile!(sub_al_cl, "SUB AL, CL", |instructions: &Vec<u8>| {
        assert_eq!(instructions, &[0x2A, 0xC1]);
    });

    test_compile!(sub_dl_bh, "SUB DL, BH", |instructions: &Vec<u8>| {
        assert_eq!(instructions, &[0x2A, 0xD7]);
    });

    test_compile!(sub_bh_dl, "SUB BH, DL", |instructions: &Vec<u8>| {
        assert_eq!(instructions, &[0x2A, 0xFA]);
    });

    test_compile!(sub_bh_0x12, "SUB BH, 0x12", |instructions: &Vec<u8>| {
        assert_eq!(instructions, &[0x80, 0xEF, 0x12]);
    });

    test_compile!(sub_al_0x0a, "SUB AL, 0x0a", |instructions: &Vec<u8>| {
        assert_eq!(instructions, &[0x2C, 0x0a]);
    });

    // sub bl, var
    test_compile!(
        sub_bl_var,
        "
    org 100h
    .data 
    var db 0x12
    code:
    SUB bL, var",
        |instructions: &Vec<u8>| {
            assert_eq!(instructions, &[0xEB, 0x01, 0x12, 0x2A, 0x1E, 0x02, 0x01]);
        }
    );

    // sub var, bl
    test_compile!(
        sub_var_bl,
        "
    org 100h
    .data
    var db 0x12
    code:
    SUB var, bL",
        |instructions: &Vec<u8>| {
            assert_eq!(instructions, &[0xEB, 0x01, 0x12, 0x28, 0x1E, 0x02, 0x01]);
        }
    );

    // sub var, 0x12
    test_compile!(
        sub_var_0x12,
        "
    org 100h
    .data
    var db 0x12
    code:
    SUB var, 0x12",
        |instructions: &Vec<u8>| {
            assert_eq!(
                instructions,
                &[0xEB, 0x01, 0x12, 0x80, 0x2E, 0x02, 0x01, 0x12]
            );
        }
    );
}
