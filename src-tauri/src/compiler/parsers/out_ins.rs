use crate::{
    compiler::{
        compilation_error::CompilationError,
        tokenized_line::TokenizedLine,
        tokens::{
            registers16bit::Registers16bit, registers8bit::Registers8bit, Assembly8086Tokens,
        },
        CompiledBytesReference,
    },
    convert_and_push_instructions,
};

use super::utils::{check_comma, push_instruction};

pub(in crate::compiler) fn parse_out(
    tokenized_line: &TokenizedLine,
    i: usize,
    compiled_bytes: &mut Vec<u8>,
    compiled_bytes_ref: &mut Vec<CompiledBytesReference>,
) -> Result<usize, CompilationError> {
    let token = tokenized_line.get(
        i,
        "This shouldn't happen, Please report this".to_string(),
        None,
    )?;
    let high_token = tokenized_line.get(
        i + 1,
        "Expected arguments after IN got nothing".to_string(),
        None, // TODO: Add suggestions later
    )?;
    check_comma(tokenized_line, high_token, i + 2)?;
    let low_token = tokenized_line.get(
        i + 3,
        "Expected arguments after IN got nothing".to_string(),
        None, // TODO: Add suggestions later
    )?;

    let is_al_ax_or_none = match &low_token.token_type {
        Assembly8086Tokens::Register8bit(Registers8bit::AL) => Some(true),
        Assembly8086Tokens::Register16bit(Registers16bit::AX) => Some(false),
        _ => None,
    };

    match &high_token.token_type {
        Assembly8086Tokens::Register16bit(Registers16bit::DX) => {
            match is_al_ax_or_none {
                Some(_) => {
                    let ins = match is_al_ax_or_none {
                        Some(true) => vec![0xEE],
                        Some(false) => vec![0xEF],
                        None => unreachable!(),
                    };
                    // i.e is al register
                    convert_and_push_instructions!(
                        compiled_bytes,
                        compiled_bytes_ref,
                        (
                            high_token => ins
                        )
                    );
                    Ok(i + 3)
                }

                None => Err(CompilationError::error_with_token(
                    low_token,
                    &format!(
                        "Expected 8bit number (or) DX got {:?} instead",
                        low_token.token_type
                    ),
                )),
            }
        }
        Assembly8086Tokens::Number8bit(num) => {
            {
                match is_al_ax_or_none {
                    Some(_) => {
                        let ins = match is_al_ax_or_none {
                            Some(true) => vec![0xE6],
                            Some(false) => vec![0xE7],
                            None => unreachable!(),
                        };
                        // i.e is al register
                        convert_and_push_instructions!(
                            compiled_bytes,
                            compiled_bytes_ref,
                            (
                                high_token => ins,
                                low_token => vec![*num]
                            )
                        );
                        Ok(i + 3)
                    }

                    None => Err(CompilationError::error_with_token(
                        low_token,
                        &format!(
                            "Expected 8bit number (or) DX got {:?} instead",
                            low_token.token_type
                        ),
                    )),
                }
            }
        }
        _ => Err(CompilationError::error_with_token(
            token,
            &format!(
                "Expected DX (or) 8bit number {:?} instead",
                high_token.token_type
            ),
        )),
    }
}

#[cfg(test)]
mod tests {
    use crate::{compiler::compile_str, test_compile};

    test_compile!(out_num_al, &format!(" OUT 0x80, AL",), |instructions: &Vec<
        u8,
    >| {
        assert_eq!(instructions, &vec![0xE6, 0x80])
    });

    
    test_compile!(out_num_ax, &format!(" OUT 0x80, AX",), |instructions: &Vec<
        u8,
    >| {
        assert_eq!(instructions, &vec![0xE7, 0x80])
    });

    
    test_compile!(out_dx_al, &format!(" OUT DX, AL",), |instructions: &Vec<
        u8,
    >| {
        assert_eq!(instructions, &vec![0xEE])
    });
    
    test_compile!(out_dx_ax, &format!(" OUT DX, AX",), |instructions: &Vec<
        u8,
    >| {
        assert_eq!(instructions, &vec![0xEF])
    });
}
