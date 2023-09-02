use crate::compiler::{
    compilation_error::CompilationError,
    tokens::{Assembly8086Tokens, Token},
    CompiledBytes,
};

pub(in crate::compiler) fn parse_inc(
    lexed_str_without_spaces: &Vec<&Token>,
    token: &Token,
    i: usize,
    len_lexed_strings: u32,
    compiled_bytes: &mut Vec<u8>,
    compiled_bytes_ref: &mut Vec<CompiledBytes>,
) -> Result<usize, CompilationError> {
    if lexed_str_without_spaces.len() - 1 < i + 1 {
        return Err(CompilationError::new(
            token.line_number,
            token.column_number + token.token_length,
            (len_lexed_strings + 1) as u32,
            "Insufficient arguments to MOV",
        ));
    }
    let high_token = lexed_str_without_spaces[i + 1];
    match &high_token.token_type {
        Assembly8086Tokens::Register16bit(high_reg) => {
            let high_reg_idx = match high_reg.get_as_idx() {
                Ok(idx) => idx,
                Err(err) => {
                    return Err(CompilationError::new(
                        high_token.line_number,
                        high_token.column_number,
                        high_token.token_length,
                        err,
                    ));
                }
            };
            compiled_bytes.push(40 + high_reg_idx);
            compiled_bytes_ref.push(CompiledBytes::new(
                vec![40+high_reg_idx],
                high_token.line_number,
                high_token.column_number,
            ));
            Ok(i + 2)
        }
        Assembly8086Tokens::Register8bit(high_reg) => {
            compiled_bytes.push(0xFE);
            compiled_bytes.push(0xc0 + high_reg.get_as_idx());
            compiled_bytes_ref.push(CompiledBytes::new(
                vec![0xFE, 0xc0 + high_reg.get_as_idx()],
                token.line_number,
                token.column_number,
            ));
            Ok(i + 2)
        }

        _ => {
            return Err(CompilationError::new(
                high_token.line_number,
                high_token.column_number,
                high_token.token_length,
                &format!(
                    "Can't compile {:?} as the first argument to INC",
                    high_token.token_type
                ),
            ));
        }
    }
}

#[cfg(test)]
mod test_inc_16bit{
    use crate::{compiler::compile_str, test_compile};

    test_compile!(
        test_inc_ax,
        "INC AX",
        |compiled_instructions: &Vec<u8>| {
            assert_eq!(compiled_instructions, &[40]);
        }
    );

    test_compile!(
        test_inc_sp,
        "INC SP",
        |compiled_instructions: &Vec<u8>| {
            assert_eq!(compiled_instructions, &[44]);
        }
    );
}

#[cfg(test)]
mod test_inc_8bit{
    use crate::{compiler::compile_str, test_compile};

    test_compile!(
        test_inc_al,
        "INC AL",
        |compiled_instructions: &Vec<u8>| {
            assert_eq!(compiled_instructions, &[0xFE, 0xc0]);
        }
    );

    test_compile!(
        test_inc_bl,
        "INC BL",
        |compiled_instructions: &Vec<u8>| {
            assert_eq!(compiled_instructions, &[0xFE, 0xc3]);
        }
    );
}