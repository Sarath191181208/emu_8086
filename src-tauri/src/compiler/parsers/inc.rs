use crate::compiler::{
    compilation_error::CompilationError, tokenized_line::TokenizedLine, tokens::Assembly8086Tokens,
    CompiledBytesReference, suggestions_utils::get_all_registers_and_variable_suggestions, types_structs::{VariableReferenceMap, VariableAddressMap},
};

use super::utils::{get_idx_from_reg, push_instruction};

pub(in crate::compiler) fn parse_inc(
    tokenized_line: &TokenizedLine,
    i: usize,
    compiled_bytes: &mut Vec<u8>,
    compiled_bytes_ref: &mut Vec<CompiledBytesReference>,
    variable_address_map: Option<&VariableAddressMap>,
) -> Result<usize, CompilationError> {
    let token = tokenized_line.get(i, "This shouldn't happen, Please report this".to_string(), None)?;
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
}
