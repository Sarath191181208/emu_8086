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

pub(in crate::compiler) fn parse_in(
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

    match &high_token.token_type {
        Assembly8086Tokens::Register16bit(Registers16bit::AX)
        | Assembly8086Tokens::Register8bit(Registers8bit::AL) => {
            let is_al = !matches!(
                &high_token.token_type,
                Assembly8086Tokens::Register16bit(Registers16bit::AX)
            );
            match &low_token.token_type {
                Assembly8086Tokens::Register16bit(Registers16bit::DX) => {
                    let ins = vec![if is_al { 0xEC } else { 0xED }];
                    convert_and_push_instructions!(
                        compiled_bytes,
                        compiled_bytes_ref,
                        (
                            high_token => ins
                        )
                    );
                    Ok(i + 3)
                }

                Assembly8086Tokens::Number8bit(num) => {
                    let ins = vec![if is_al { 0xE4 } else { 0xE5 }];
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

                _ => Err(CompilationError::error_with_token(
                    low_token,
                    &format!(
                        "Expected 8bit number (or) DX got {:?} instead",
                        low_token.token_type
                    ),
                )),
            }
        }
        _ => Err(CompilationError::error_with_token(
            token,
            &format!(
                "Expected AX (or) AL (or) DX got {:?} instead",
                high_token.token_type
            ),
        )),
    }
}

#[cfg(test)]
mod tests {
    use crate::{compiler::compile_str, test_compile};

    test_compile!(
        out_in_al_and_num,
        "IN AL, 0x80",
        |instructions: &Vec<u8>| { assert_eq!(instructions, &vec![0xE4, 0x80]) }
    );

    test_compile!(
        out_in_ax_and_num,
        "IN AX, 0x10",
        |instructions: &Vec<u8>| { assert_eq!(instructions, &vec![0xE5, 0x10]) }
    );

    test_compile!(
        out_in_al_and_dx,
        "IN AL, DX",
        |instructions: &Vec<u8>| { assert_eq!(instructions, &vec![0xEC]) }
    );

    test_compile!(
        out_in_ax_and_dx,
        "IN AX, DX",
        |instructions: &Vec<u8>| { assert_eq!(instructions, &vec![0xED]) }
    );
}
