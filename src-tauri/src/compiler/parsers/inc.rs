use crate::{
    compiler::{
        compilation_error::CompilationError,
        parsers::utils::get_label_address_or_push_into_ref,
        suggestions_utils::get_all_registers_and_variable_suggestions,
        tokenized_line::TokenizedLine,
        tokens::Assembly8086Tokens,
        types_structs::{VariableAddressMap, VariableReferenceMap, VariableType},
        CompiledBytesReference,
    },
    convert_and_push_instructions,
};

use super::utils::{get_idx_from_reg, push_instruction};

pub(in crate::compiler) fn parse_inc(
    tokenized_line: &TokenizedLine,
    i: usize,
    compiled_bytes: &mut Vec<u8>,
    compiled_bytes_ref: &mut Vec<CompiledBytesReference>,

    var_ref_map: &mut VariableReferenceMap,
    variable_address_map: Option<&VariableAddressMap>,
) -> Result<usize, CompilationError> {
    let token = tokenized_line.get(
        i,
        "This shouldn't happen, Please report this".to_string(),
        None,
    )?;
    let high_token = tokenized_line.get(
        i + 1,
        "Expected arguments after INC got nothing".to_string(),
        Some(vec![get_all_registers_and_variable_suggestions(
            variable_address_map,
        )]),
    )?;
    match &high_token.token_type {
        Assembly8086Tokens::Register16bit(high_reg) => {
            let high_reg_idx = get_idx_from_reg(high_token, high_reg)?;
            push_instruction(
                compiled_bytes,
                vec![0x40 + high_reg_idx],
                high_token,
                compiled_bytes_ref,
            );
            Ok(i + 2)
        }
        Assembly8086Tokens::Register8bit(high_reg) => {
            push_instruction(compiled_bytes, vec![0xFE], token, compiled_bytes_ref);
            push_instruction(
                compiled_bytes,
                vec![0xC0 + high_reg.get_as_idx()],
                high_token,
                compiled_bytes_ref,
            );

            Ok(i + 2)
        }

        Assembly8086Tokens::Character(high_char) => {
            let abs_addr = get_label_address_or_push_into_ref(
                i + 1,
                high_char,
                VariableType::Byte,
                variable_address_map.unwrap_or(&VariableAddressMap::new()),
                var_ref_map,
            );
            let variable_type = variable_address_map
                .unwrap_or(&VariableAddressMap::new())
                .get(high_char)
                .unwrap_or(&(VariableType::Byte, 0_u16))
                .0;

            let instruction = match variable_type {
                VariableType::Byte => vec![0xFE, 0x06],
                VariableType::Word => vec![0xFF, 0x06],
            };

            convert_and_push_instructions!(
                compiled_bytes,
                compiled_bytes_ref,
                (
                    token => instruction,
                    high_token => abs_addr.to_vec()
                )
            );

            Ok(i + 2)
        }

        _ => Err(CompilationError::new_without_suggestions(
            high_token.line_number,
            high_token.column_number,
            high_token.token_length,
            &format!(
                "Can't compile {:?} as the first argument to INC, Expected a register",
                high_token.token_type
            ),
        )),
    }
}

#[cfg(test)]
mod test_inc_16bit {
    use crate::{compiler::compile_str, test_compile};

    test_compile!(test_inc_ax, "INC AX", |compiled_instructions: &Vec<u8>| {
        assert_eq!(compiled_instructions, &[0x40]);
    });

    test_compile!(test_inc_sp, "INC SP", |compiled_instructions: &Vec<u8>| {
        assert_eq!(compiled_instructions, &[0x44]);
    });

    test_compile!(
        test_inc_var,
        "
    org 100h 
    .data 
    var dw 0x0001
    code:
    inc var
    ",
        |compiled_instructions: &Vec<u8>| {
            assert_eq!(
                compiled_instructions,
                &[0xEB, 0x02, 0x01, 0x00, 0xFF, 0x06, 0x02, 0x01]
            );
        }
    );
}

#[cfg(test)]
mod test_inc_8bit {
    use crate::{compiler::compile_str, test_compile};

    test_compile!(test_inc_al, "INC AL", |compiled_instructions: &Vec<u8>| {
        assert_eq!(compiled_instructions, &[0xFE, 0xc0]);
    });

    test_compile!(test_inc_bl, "INC BL", |compiled_instructions: &Vec<u8>| {
        assert_eq!(compiled_instructions, &[0xFE, 0xc3]);
    });

    test_compile!(
        test_inc_var,
        "
    org 100h
    .data
    var db 0x01
    code:
    inc var
    ",
        |compiled_instructions: &Vec<u8>| {
            assert_eq!(
                compiled_instructions,
                &[0xEB, 0x01, 0x01, 0xFE, 0x06, 0x02, 0x01]
            );
        }
    );
}
