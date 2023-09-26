use super::{
    compilation_error::CompilationError,
    strip_space_and_comments_and_iterate_labels,
    tokens::{assembler_directives, instructions::Instructions, Assembly8086Tokens, Token},
};

pub(crate) fn find_data_line_num(lexed_strings: &[Vec<Token>]) -> u32 {
    // find the data directive
    let data_line_num = lexed_strings.iter().position(|line| {
        line.iter().any(|token| {
            token.token_type
                == Assembly8086Tokens::Instruction(Instructions::AssemblerDirectives(
                    assembler_directives::AssemblerDirectives::Data,
                ))
        })
    });
    match data_line_num {
        Some(data_line_num) => data_line_num as u32,
        None => 0xFFFF_FFFF,
    }
}

pub(crate) fn is_org_defined(lexed_strings: &Vec<Vec<Token>>) -> Result<bool, CompilationError> {
    let data_line_num = find_data_line_num(lexed_strings);
    for line in lexed_strings {
        let (stripped_line, _) = strip_space_and_comments_and_iterate_labels(line);
        // check if index 0 is of type ORG
        if stripped_line.is_empty() {
            continue;
        }

        let is_org = matches!(
            stripped_line[0].token_type,
            Assembly8086Tokens::Instruction(Instructions::AssemblerDirectives(
                assembler_directives::AssemblerDirectives::Org,
            ))
        );

        if !is_org {
            continue;
        }

        // check if data line is smaller than org
        if data_line_num < line[0].line_number {
            return Err(CompilationError::new(
                line[0].line_number,
                line[0].column_number,
                line[0].token_length,
                &format!(
                    "Can't compile starting with {:?} as the ORG directive must be defined before the DATA directive",
                    line[0].token_type
                ),
            ));
        }

        // chekc the len of arr
        if stripped_line.len() != 2 {
            return Err(CompilationError::new(
                stripped_line[0].line_number,
                stripped_line[0].column_number,
                stripped_line[0].token_length,
                &format!(
                    "Can't compile starting with {:?} as the ORG directive only supports 1 argument",
                    stripped_line[0].token_type
                ),
            ));
        }

        // check if the next instruction is 0x100
        match stripped_line[1].token_type {
            Assembly8086Tokens::Number16bit(0x100) => return Ok(true),
            Assembly8086Tokens::Register16bit(_) => return Ok(false),
            _ => {
                return Err(CompilationError::new(
                        stripped_line[1].line_number,
                        stripped_line[1].column_number,
                        stripped_line[1].token_length,
                        &format!(
                            "Can't compile starting with {:?} as the ORG directive only supports 0x100 as an argument",
                            stripped_line[1].token_type
                        ),
                    ));
            }
        }
    }
    Ok(false)
}

pub(crate) fn get_full_line_error_starting_from_i(
    lexed_str_without_spaces: &Vec<&Token>,
    i: usize,
    err_msg: &str,
) -> Result<(), CompilationError> {
    if i < lexed_str_without_spaces.len() - 1 {
        let unparsed_tokens_start = lexed_str_without_spaces[i + 1];
        let unparsed_tokens_end = lexed_str_without_spaces.last().unwrap();
        return Err(CompilationError::new(
            unparsed_tokens_start.line_number,
            unparsed_tokens_start.column_number,
            unparsed_tokens_start.column_number
                + unparsed_tokens_end.column_number
                + unparsed_tokens_end.token_length,
            err_msg,
        ));
    }
    Ok(())
}

pub(crate) fn error_if_hasnt_consumed_all_ins(
    lexed_str_without_spaces: &Vec<&Token>,
    i: usize,
    instruction: &str,
    num_args: usize,
) -> Result<(), CompilationError> {
    get_full_line_error_starting_from_i(
        lexed_str_without_spaces,
        i,
        &format!(
            "Can't compile as the {} instuction only supports {} arguments",
            instruction, num_args
        ),
    )?;
    Ok(())
}

pub(crate) fn check_is_label(lexed_str_without_spaces: &Vec<&Token>) -> Option<String> {
    // return false if len < 2
    // check if the first token is a Character
    // check if the second token is a colon
    if lexed_str_without_spaces.len() < 2 {
        return None;
    }
    let first_token = lexed_str_without_spaces[0];
    let second_token = lexed_str_without_spaces[1];

    if let Assembly8086Tokens::Character(str) = &first_token.token_type {
        if let Assembly8086Tokens::Colon = second_token.token_type {
            return Some(str.to_string());
        }
    }

    None
}
