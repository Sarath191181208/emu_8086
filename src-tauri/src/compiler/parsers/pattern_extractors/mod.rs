use crate::compiler::{
    compilation_error::CompilationError,
    tokenized_line::TokenizedLine,
    tokens::{Assembly8086Tokens, Token},
};

use super::utils::{check_comma, if_num_8bit_to_16bit};

pub(crate) enum AddressingMode<'a> {
    Registers16bit {
        high_token: &'a Token,
        low_token: Token,
    },
    Registers8bit {
        high_token: &'a Token,
        low_token: &'a Token,
    },
    Registers16bitNumber {
        high_token: &'a Token,
        low_token: Token,
    },
    Register8bitNumber {
        high_token: &'a Token,
        low_token: &'a Token,
    },
}

pub(crate) fn parse_line<'a>(
    tokenized_line: &'a TokenizedLine<'a>,
    i: usize,
    ins: &'a str,
) -> Result<AddressingMode<'a>, CompilationError> {
    let len_lexed_strings = tokenized_line.get_len_lexed_strings();
    let token = tokenized_line.get(i, "This shouldn't happen, Please report this".to_string())?;
    let high_token = tokenized_line.get(
        i + 1,
        format!("Expected arguments after {} got nothing", ins).to_string(),
    )?;

    match &high_token.token_type {
        Assembly8086Tokens::Register16bit(_) => {
            let low_token = tokenized_line.get(
                i + 3,
                format!("Expected 16bit value after {:?} got nothing", high_token).to_string(),
            )?;
            check_comma(tokenized_line, high_token, i + 2)?;
            let changed_low_token = Token::new(
                if_num_8bit_to_16bit(low_token.token_type.clone()),
                low_token.line_number,
                low_token.column_number,
                low_token.token_length,
            );
            match &changed_low_token.token_type {
                Assembly8086Tokens::Number16bit(_) => Ok(AddressingMode::Registers16bitNumber {
                    high_token,
                    low_token: changed_low_token.clone(),
                }),
                Assembly8086Tokens::Register16bit(_) => Ok(AddressingMode::Registers16bit {
                    high_token,
                    low_token: changed_low_token.clone(),
                }),

                _ => Err(CompilationError::new(
                    token.line_number,
                    high_token.column_number + high_token.token_length + 1,
                    len_lexed_strings - high_token.column_number - high_token.token_length,
                    &format!(
                        "Expected a 16bit value after {} got {:?} insted",
                        ins, &low_token.token_type
                    ),
                )),
            }
        }

        Assembly8086Tokens::Register8bit(_) => {
            let low_token = tokenized_line.get(
                i + 3,
                format!("Expected 8bit value after {:?} got nothing", high_token).to_string(),
            )?;
            check_comma(tokenized_line, high_token, i + 2)?;
            match &low_token.token_type {
                Assembly8086Tokens::Number8bit(_) => Ok(AddressingMode::Register8bitNumber {
                    high_token,
                    low_token,
                }),
                Assembly8086Tokens::Register8bit(_) => Ok(AddressingMode::Registers8bit {
                    high_token,
                    low_token,
                }),

                _ => Err(CompilationError::new(
                    token.line_number,
                    high_token.column_number + high_token.token_length + 1,
                    len_lexed_strings - high_token.column_number - high_token.token_length,
                    &format!(
                        "Expected a 8bit value after {} got {:?} insted",
                        ins, &low_token.token_type
                    ),
                )),
            }
        }
        _ => Err(CompilationError::new(
            high_token.line_number,
            high_token.column_number,
            high_token.token_length,
            &format!(
                "Expected a 16bit or 8bit register after {} got {:?} insted",
                ins, &high_token.token_type
            ),
        )),
    }
}
