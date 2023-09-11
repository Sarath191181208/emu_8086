use crate::compiler::{
    compilation_error::CompilationError, tokenized_line::TokenizedLine, tokens::Assembly8086Tokens,
    CompiledBytes,
};

use super::utils::{get_idx_from_reg, push_instruction};

pub(in crate::compiler) fn parse_mul(
    tokenized_line: &TokenizedLine,
    i: usize,
    compiled_bytes: &mut Vec<u8>,
    compiled_bytes_ref: &mut Vec<CompiledBytes>,
) -> Result<usize, CompilationError> {
    let token = tokenized_line.get(i, "This shouldn't happen, Please report this".to_string())?;
    let high_token = tokenized_line.get(
        i + 1,
        "Expected arguments after MUL got nothing".to_string(),
    )?;
    match &high_token.token_type {
        Assembly8086Tokens::Register16bit(high_reg) => {
            let high_reg_idx = get_idx_from_reg(high_token, high_reg)?;
            push_instruction(compiled_bytes, vec![0xF7], token, compiled_bytes_ref);
            push_instruction(
                compiled_bytes,
                vec![0xE0 + high_reg_idx],
                high_token,
                compiled_bytes_ref,
            );
            Ok(i + 2)
        }
        Assembly8086Tokens::Register8bit(high_reg) => {
            push_instruction(compiled_bytes, vec![0xF6], token, compiled_bytes_ref);
            push_instruction(
                compiled_bytes,
                vec![0xE0 + high_reg.get_as_idx()],
                high_token,
                compiled_bytes_ref,
            );

            Ok(i + 2)
        }
        _ => Err(CompilationError::new(
            high_token.line_number,
            high_token.column_number,
            high_token.token_length,
            &format!(
                "Can't compile {:?} as the first argument to MUL, Expected a register",
                high_token.token_type
            ),
        )),
    }
}

#[cfg(test)]
mod test_mul_16bit {
    use crate::{compiler::compile_str, test_compile};

    test_compile!(test_mul_ax, "MUL AX", |compiled_instructions: &Vec<u8>| {
        assert_eq!(compiled_instructions, &[0xF7, 0xE0]);
    });

    test_compile!(test_mul_sp, "mul SP", |compiled_instructions: &Vec<u8>| {
        assert_eq!(compiled_instructions, &[0xF7, 0xE4]);
    });
}

#[cfg(test)]
mod test_mul_8bit {
    use crate::{compiler::compile_str, test_compile};

    test_compile!(test_mul_al, "MUL AL", |compiled_instructions: &Vec<u8>| {
        assert_eq!(compiled_instructions, &[0xF6, 0xE0]);
    });

    test_compile!(test_mul_bl, "mul BL", |compiled_instructions: &Vec<u8>| {
        assert_eq!(compiled_instructions, &[0xF6, 0xE3]);
    });
}
