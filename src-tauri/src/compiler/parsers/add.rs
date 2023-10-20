use crate::{
    compiler::{
        compilation_error::CompilationError, parsers::utils::get_idx_from_reg,
        tokenized_line::TokenizedLine, types_structs::VariableAddressMap, CompiledBytesReference,
    },
    convert_and_push_instructions,
    utils::Either,
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
    utils::{
        get_8bit_register, get_as_0xc0_0xff_pattern, get_idx_from_token, get_token_as_label,
        is_variable_defined_as_16bit, push_instruction,
    },
};

pub(in crate::compiler) fn parse_add(
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
        AddressingMode::Registers16bit {
            high_token,
            low_token,
        } => {
            let high_reg_idx = get_idx_from_token(&high_token)?;
            let low_reg_idx = get_idx_from_token(&low_token)?;
            let ins = get_as_0xc0_0xff_pattern(high_reg_idx, low_reg_idx);
            push_instruction(compiled_bytes, vec![0x03], token, compiled_bytes_ref);
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
            push_instruction(compiled_bytes, vec![0x02], token, compiled_bytes_ref);
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
                push_instruction(compiled_bytes, vec![0x05], token, compiled_bytes_ref);
                push_instruction(
                    compiled_bytes,
                    vec![(number & 0xFF) as u8, (number >> 8) as u8],
                    &low_token,
                    compiled_bytes_ref,
                );
            } else {
                let number = num.get_as_u16();
                let add_ins = match num {
                    Either::Right(_) => 0x81,
                    Either::Left(_) => 0x83,
                };
                let data_ins = match num {
                    Either::Right(_) => vec![(number & 0xFF) as u8, (number >> 8) as u8],
                    Either::Left(x) => vec![x],
                };

                push_instruction(compiled_bytes, vec![add_ins], token, compiled_bytes_ref);
                push_instruction(
                    compiled_bytes,
                    vec![0xC0 | high_reg_idx],
                    &high_token,
                    compiled_bytes_ref,
                );
                push_instruction(compiled_bytes, data_ins, &low_token, compiled_bytes_ref);
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
                push_instruction(compiled_bytes, vec![0x04], token, compiled_bytes_ref);
                push_instruction(compiled_bytes, vec![number], &low_token, compiled_bytes_ref);
            } else {
                push_instruction(compiled_bytes, vec![0x82], token, compiled_bytes_ref);
                push_instruction(
                    compiled_bytes,
                    vec![0xC0 | high_reg.get_as_idx()],
                    &high_token,
                    compiled_bytes_ref,
                );
                push_instruction(compiled_bytes, vec![number], &low_token, compiled_bytes_ref);
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
                    token => vec![0x03],
                    &high_token => vec![0x06 | reg_idx << 3],
                    &low_token => address_bytes.to_vec()
                )
            );
            Ok(tokenized_line.len())
        }
        AddressingMode::AddressAnd16bitRegister {
            high_token,
            low_token,
            address_bytes,
            register_type,
        } => {
            let reg_idx = get_idx_from_reg(&low_token, &register_type)?;
            convert_and_push_instructions!(
                compiled_bytes,
                compiled_bytes_ref,
                (
                    token => vec![0x01],
                   &high_token=> vec![0x06 | reg_idx << 3],
                    &low_token => address_bytes.to_vec()
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
                    token => vec![0x81, 0x06],
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
                    token => vec![0x02],
                    &high_token => vec![0x06 | reg_idx << 3],
                    &low_token => address_bytes.to_vec()
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
                    token => vec![0x00],
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
                    token => vec![add_ins, 0x06],
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
                0x03,
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
                0x03,
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
                0x02,
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
                0x02,
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
mod tests16bit {
    use crate::{compiler::compile_str, test_compile};

    test_compile!(add_ax_sp, "ADD AX, SP", |instructions: &Vec<u8>| {
        assert_eq!(instructions, &[0x03, 0xC4]);
    });

    test_compile!(add_ax_0x1234, "ADD AX, 0x1234", |instructions: &Vec<u8>| {
        assert_eq!(instructions, &[0x05, 0x34, 0x12]);
    });

    test_compile!(add_bx_0xff12, "ADD BX, 0xff12", |instructions: &Vec<u8>| {
        assert_eq!(instructions, &[0x81, 0xC3, 0x12, 0xff]);
    });

    // test cx + 0x1234
    test_compile!(add_cx_0x12, "ADD CX, 0x12", |instructions: &Vec<u8>| {
        assert_eq!(instructions, &[0x83, 0xC1, 0x12]);
    });

    // test bx + var
    test_compile!(
        add_bx_var,
        "
    org 100h 
    .data 
    var dw 0x12
    code: 
    ADD BX, var",
        |instructions: &Vec<u8>| {
            assert_eq!(
                instructions,
                &[0xEB, 0x02, 0x12, 0x00, 0x03, 0x1E, 0x02, 0x01]
            );
        }
    );

    // test var + bx
    test_compile!(
        add_var_bx,
        "
    org 100h
    .data
    var dw 0x12
    code:
    ADD var, BX",
        |instructions: &Vec<u8>| {
            assert_eq!(
                instructions,
                &[0xEB, 0x02, 0x12, 0x00, 0x01, 0x1E, 0x02, 0x01]
            );
        }
    );

    // test var + 0x2000
    test_compile!(
        add_var_0x2000,
        "
    org 100h
    .data
    var dw 0x12
    code:
    ADD var, 0x2000",
        |instructions: &Vec<u8>| {
            assert_eq!(
                instructions,
                &[0xEB, 0x02, 0x12, 0x00, 0x81, 0x06, 0x02, 0x01, 0x00, 0x20]
            );
        }
    );

    // test var + 0x10
    test_compile!(
        add_var_0x10,
        "
    org 100h
    .data
    var dw 0x12
    code:
    ADD var, 0x10",
        |instructions: &Vec<u8>| {
            assert_eq!(
                instructions,
                &[0xEB, 0x02, 0x12, 0x00, 0x83, 0x06, 0x02, 0x01, 0x10]
            );
        }
    );

    test_compile!(add_dx_di_ref, "ADD DX, [DI", |instructions: &Vec<u8>| {
        assert_eq!(instructions, &[0x03, 0x15]);
    });

    test_compile!(
        add_dx_di_bx_ref_0x70,
        "ADD DX, Di + 0x20 + 0x30 + BX + 0x10 + BX []+ 0x10",
        |instructions: &Vec<u8>| {
            assert_eq!(instructions, &[0x03, 0x51, 0x70]);
        }
    );

    test_compile!(
        add_dx_si_value_ref,
        "ADD DX, SI + 0x2000",
        |instructions: &Vec<u8>| {
            assert_eq!(instructions, &[0x03, 0x94, 0x00, 0x20]);
        }
    );

    test_compile!(
        add_ax_proc_ref,
        "
        ADD AX, main 
        inc ax

        proc main 
            mov ax, bx 
        endp main
        ",
        |instructions: &Vec<u8>| {
            assert_eq!(instructions, &[0x05, 0x05, 0x00, 0x40, 0x8B, 0xC3]);
        }
    );

    // Add bp, 0100o
    test_compile!(add_bp_0100o, "ADD BP, 0100o", |instructions: &Vec<u8>| {
        assert_eq!(instructions, &[0x83, 0xC5, 0x40]);
    });
}

#[cfg(test)]
mod test8bit {
    use crate::{compiler::compile_str, test_compile};

    test_compile!(add_al_0x12, "ADD AL, 0x12", |instructions: &Vec<u8>| {
        assert_eq!(instructions, &[0x04, 0x12]);
    });

    // add bl and cl
    test_compile!(add_bl_cl, "ADD BL, CL", |instructions: &Vec<u8>| {
        assert_eq!(instructions, &[0x02, 0xD9]);
    });

    // add ah and bl
    test_compile!(add_ah_bl, "ADD AH, BL", |instructions: &Vec<u8>| {
        assert_eq!(instructions, &[0x02, 0xE3]);
    });

    // add ah and 0x12
    test_compile!(add_ah_0x12, "ADD AH, 0x12", |instructions: &Vec<u8>| {
        assert_eq!(instructions, &[0x82, 0xC4, 0x12]);
    });

    // test bl + var
    test_compile!(
        add_bl_var,
        "
    org 100h
    .data
    var db 0x12
    code:
    add al, var
    ",
        |instructions: &Vec<u8>| {
            assert_eq!(instructions, &[0xEB, 0x01, 0x12, 0x2, 0x06, 0x02, 0x01]);
        }
    );

    // test var + cl
    test_compile!(
        add_var_cl,
        "
    org 100h
    .data
    var db 0x12
    var2 db 0x11
    code:
    add var, cl
    ",
        |instructions: &Vec<u8>| {
            assert_eq!(
                instructions,
                &[0xEB, 0x02, 0x12, 0x11, 0x00, 0x0E, 0x02, 0x01]
            );
        }
    );

    // test var + 0x20
    test_compile!(
        add_var_0x20,
        "
    org 100h
    .data
    var db 0x12
    var2 db 0x11
    code:
    add var, 0x20
    ",
        |instructions: &Vec<u8>| {
            assert_eq!(
                instructions,
                &[0xEB, 0x02, 0x12, 0x11, 0x80, 0x06, 0x02, 0x01, 0x20]
            );
        }
    );

    test_compile!(add_dl_0x100_ref, "ADD DL, [0x100]", |instructions: &Vec<
        u8,
    >| {
        assert_eq!(instructions, &[0x02, 0x16, 0x00, 0x01]);
    });

    test_compile!(add_dx_di_ref, "ADD DL, [DI", |instructions: &Vec<u8>| {
        assert_eq!(instructions, &[0x02, 0x15]);
    });

    test_compile!(
        add_dl_di_bx_ref_0x70,
        "ADD Dl, Di + 0x20 + 0x30 + BX + 0x10 + BX []+ 0x10",
        |instructions: &Vec<u8>| {
            assert_eq!(instructions, &[0x02, 0x51, 0x70]);
        }
    );

    test_compile!(
        add_dx_si_value_ref,
        "ADD Dh, SI + 0x2000",
        |instructions: &Vec<u8>| {
            assert_eq!(instructions, &[0x02, 0xB4, 0x00, 0x20]);
        }
    );

    // test add al, 010o
    test_compile!(add_al_010o, "ADD AL, 010o", |instructions: &Vec<u8>| {
        assert_eq!(instructions, &[0x04, 0x08]);
    });
}
