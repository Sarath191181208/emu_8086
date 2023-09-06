use crate::compiler::{
    compilation_error::CompilationError,
    tokens::Assembly8086Tokens,
    CompiledBytes, tokenized_line::TokenizedLine,
};

use super::utils::{get_idx_from_reg, push_instruction};

pub(in crate::compiler) fn parse_dec(
    tokenized_line: &TokenizedLine,
    i: usize,
    compiled_bytes: &mut Vec<u8>,
    compiled_bytes_ref: &mut Vec<CompiledBytes>,
) -> Result<usize, CompilationError> {
    let token = tokenized_line.get(i, "This shouldn't happen, Please report this".to_string())?;
    let high_token = tokenized_line.get(
        i + 1,
        "Expected arguments after DEC got nothing".to_string(),
    )?;
    match &high_token.token_type {
        Assembly8086Tokens::Register16bit(high_reg) => {
            let high_reg_idx = get_idx_from_reg(high_token, high_reg)?;
            push_instruction(
                compiled_bytes,
                vec![0x48 + high_reg_idx],
                high_token,
                compiled_bytes_ref,
            );
            Ok(i + 2)
        }
        Assembly8086Tokens::Register8bit(high_reg) => {
            push_instruction(compiled_bytes, vec![0xFE], token, compiled_bytes_ref);
            push_instruction(
                compiled_bytes,
                vec![0xC8 + high_reg.get_as_idx()],
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
                "Can't compile {:?} as the first argument to DEC",
                high_token.token_type
            ),
        )),
    }
}

#[cfg(test)]
mod test_inc_16bit {
    use crate::{compiler::compile_str, test_compile};

    test_compile!(test_dec_ax, "DEC AX", |compiled_instructions: &Vec<u8>| {
        assert_eq!(compiled_instructions, &[0x48]);
    });

    test_compile!(test_dec_sp, "dec SP", |compiled_instructions: &Vec<u8>| {
        assert_eq!(compiled_instructions, &[0x4C]);
    });
}

#[cfg(test)]
mod test_dec_8bit {
    use crate::{compiler::compile_str, test_compile};

    test_compile!(test_dec_al, "dec AL", |compiled_instructions: &Vec<u8>| {
        assert_eq!(compiled_instructions, &[0xFE, 0xc8]);
    });

    test_compile!(test_dec_bl, "dec BL", |compiled_instructions: &Vec<u8>| {
        assert_eq!(compiled_instructions, &[0xFE, 0xCB]);
    });
}
