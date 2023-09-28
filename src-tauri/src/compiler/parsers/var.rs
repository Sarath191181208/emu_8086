use crate::{
    compiler::{
        compilation_error::CompilationError,
        parsers::utils::push_instruction,
        suggestions_utils::{get_8bit_number_suggestion, get_all_define_data_suggestions, get_16bit_number_suggestion},
        tokenized_line::TokenizedLine,
        tokens::{data::DefineData, Assembly8086Tokens},
        types_structs::{CompiledBytesReference, VariableAddressDefinitionMap, VariableType},
    },
    convert_and_push_instructions,
};

use super::utils::{check_comma, get_token_as_label, if_num_8bit_to_16bit};

fn to_bytes(
    i: usize,
    tokenized_line: &TokenizedLine,
    compiled_bytes: &mut Vec<u8>,
    compiled_bytes_ref: &mut Vec<CompiledBytesReference>,
) -> Result<usize, CompilationError> {
    let mut j = i;
    let last_token = tokenized_line.get(
        tokenized_line.len() - 1,
        "This shouldn't happen, Please report this".to_string(),
        None,
    )?;
    if j > tokenized_line.len() {
        return Err(CompilationError::new_with_suggestions(
            last_token.line_number,
            last_token.column_number,
            last_token.token_length,
            "Expected 8 bit number, Got nothing!",
            vec![get_8bit_number_suggestion()]),
        );
    }
    while j < tokenized_line.len() {
        let data_token = tokenized_line.get(
            j,
            "This shouldn't happen, Please report this".to_string(),
            None,
        )?;
        match data_token.token_type {
            Assembly8086Tokens::Number8bit(number) => {
                convert_and_push_instructions!(
                    compiled_bytes,
                    compiled_bytes_ref,
                    (
                        data_token => vec![number]
                    )
                );
                j += 1;
                if j < tokenized_line.len() - 1 {
                    check_comma(tokenized_line, data_token, j)?;
                    j += 1;
                }
            }
            _ => {
                return Err(CompilationError::new_without_suggestions(
                    data_token.line_number,
                    data_token.column_number,
                    data_token.column_number + data_token.token_length,
                    &format!(
                        "Expected 8 bit number, Got {}, Insted",
                        data_token.token_type
                    ),
                ))
            }
        }
    }
    Ok(j)
}

fn to_words(
    i: usize,
    tokenized_line: &TokenizedLine,
    compiled_bytes: &mut Vec<u8>,
    compiled_bytes_ref: &mut Vec<CompiledBytesReference>,
) -> Result<usize, CompilationError> {
    let mut j = i;
        let last_token = tokenized_line.get(
        tokenized_line.len() - 1,
        "This shouldn't happen, Please report this".to_string(),
        None,
    )?;
    if j > tokenized_line.len() {
        return Err(CompilationError::new_with_suggestions(
            last_token.line_number,
            last_token.column_number,
            last_token.token_length,
            "Expected 8 bit number, Got nothing!",
            vec![get_16bit_number_suggestion()]),
        );
    }
    while j < tokenized_line.len() {
        let data_token = tokenized_line.get(
            j,
            "This shouldn't happen, Please report this".to_string(),
            None,
        )?;
        let changed_token = if_num_8bit_to_16bit(data_token.token_type.clone());
        match changed_token {
            Assembly8086Tokens::Number16bit(number) => {
                convert_and_push_instructions!(
                    compiled_bytes,
                    compiled_bytes_ref,
                    (
                        data_token => vec![number as u8, (number >> 8) as u8]
                    )
                );
                j += 1;
                if j < tokenized_line.len() - 1 {
                    check_comma(tokenized_line, data_token, j)?;
                    j += 1;
                }
            }
            _ => {
                return Err(CompilationError::new_without_suggestions(
                    data_token.line_number,
                    data_token.column_number,
                    data_token.column_number + data_token.token_length,
                    &format!("Expected 16 bit number, Got {}", data_token.token_type),
                ))
            }
        }
    }
    Ok(j)
}

pub(in crate::compiler) fn parse_var_declaration(
    tokenized_line: &TokenizedLine,
    i: usize,
    compiled_bytes: &mut Vec<u8>,
    compiled_bytes_ref: &mut Vec<CompiledBytesReference>,
    label_abs_address_map: &mut VariableAddressDefinitionMap,
) -> Result<usize, CompilationError> {
    let variable_token = tokenized_line.get(
        i,
        "This shouldn't happen, Please report this".to_string(),
        None,
    )?;
    let define_data = tokenized_line.get(
        i + 1,
        "Expected db (or) dw, Got nothing!".to_string(),
        Some(vec![get_all_define_data_suggestions()]),
    )?;
    let var_label = get_token_as_label(variable_token);
    match &define_data.token_type {
        Assembly8086Tokens::Data(data) => match data {
            DefineData::Db => {
                let i = to_bytes(i + 2, tokenized_line, compiled_bytes, compiled_bytes_ref)?;
                label_abs_address_map.insert(var_label.clone(), (VariableType::Byte, 0));

                Ok(i)
            }
            DefineData::Dw => {
                let i = to_words(i + 2, tokenized_line, compiled_bytes, compiled_bytes_ref)?;
                label_abs_address_map.insert(var_label.clone(), (VariableType::Word, 0));
                Ok(i)
            }
        },
        _ => Err(CompilationError::new_without_suggestions(
            define_data.line_number,
            define_data.column_number,
            define_data.column_number + define_data.token_length,
            &format!("Expected db (or) dw, Got {} insted", define_data.token_type),
        )),
    }
}

#[cfg(test)]
mod test_variable_declaration {
    use crate::{compiler::compile_str, test_compile};

    test_compile!(
        dec_var_1,
        "
        var1 db 0x12
        ",
        |instructions: &Vec<u8>| {
            assert_eq!(instructions, &vec![0x12]);
        }
    );

    test_compile!(
        dec_vars_5,
        "
        var db 0x12, 0x13, 0x14, 0x15, 0x16
        ",
        |instructions: &Vec<u8>| {
            assert_eq!(instructions, &vec![0x12, 0x13, 0x14, 0x15, 0x16]);
        }
    );

    test_compile!(
        dec_2_vars_1,
        "
        var db 0x12
        var2 db 0x13
        ",
        |instructions: &Vec<u8>| {
            assert_eq!(instructions, &vec![0x12, 0x13]);
        }
    );

    test_compile!(
        dec_2_vars_5,
        "
        var db 0x12, 0x13, 0x14, 0x15, 0x16
        var2 db 0x12, 0x13, 0x14, 0x15, 0x16
        ",
        |instructions: &Vec<u8>| {
            assert_eq!(
                instructions,
                &vec![0x12, 0x13, 0x14, 0x15, 0x16, 0x12, 0x13, 0x14, 0x15, 0x16]
            );
        }
    );

    test_compile!(
        dec_var_1_word,
        "
        var1 dw 0x1234
        ",
        |instructions: &Vec<u8>| {
            assert_eq!(instructions, &vec![0x34, 0x12]);
        }
    );

    test_compile!(
        dec_vars_5_word,
        "
        var dw 0x1234, 0x1234
        ",
        |instructions: &Vec<u8>| { assert_eq!(instructions, &vec![0x34, 0x12, 0x34, 0x12]) }
    );

    test_compile!(
        dec_2_vars_1_word,
        "
        var dw 0x1234
        var2 dw 0x1234
        ",
        |instructions: &Vec<u8>| { assert_eq!(instructions, &vec![0x34, 0x12, 0x34, 0x12]) }
    );

    test_compile!(
        dec_2_vars_2_words,
        "
        var dw 0x1234, 0x1234
        var2 dw 0x1234, 0x1234
        ",
        |instructions: &Vec<u8>| {
            assert_eq!(
                instructions,
                &vec![0x34, 0x12, 0x34, 0x12, 0x34, 0x12, 0x34, 0x12]
            )
        }
    );
}
