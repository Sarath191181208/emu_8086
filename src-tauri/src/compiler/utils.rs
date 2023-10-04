use unicase::UniCase;

use super::{tokens::{instructions::Instructions, Assembly8086Tokens, Token}, compilation_error::CompilationError, types_structs::Label, lexer::Lexer};

pub(in crate::compiler) fn get_jmp_code_compiled_line(token: &Token) -> Vec<Token> {
    [
        Token::new(
            Assembly8086Tokens::Instruction(Instructions::Jmp),
            token.line_number,
            token.column_number,
            token.token_length,
        ),
        Token::new(
            Assembly8086Tokens::Character(UniCase::new("code".to_string())),
            token.line_number,
            token.column_number,
            token.token_length,
        ),
    ]
    .to_vec()
}


impl CompilationError{
    pub(super) fn error_with_token(token: &Token, msg: &str) -> Self {
        CompilationError::new_without_suggestions(
            token.line_number,
            token.column_number,
            token.token_length,
            msg,
        )
    }

    pub(super) fn error_between_tokens(token1: &Token, token2: &Token, msg: &str) -> Self {
        CompilationError::new_without_suggestions(
            token1.line_number,
            token1.column_number,
            token2.column_number + token2.token_length - token1.column_number,
            msg,
        )
    }
}

pub(super) fn get_label_token_from_line<'a>(
    lexer: &'a Lexer,
    line_number: usize,
    label: &Label
) -> Option<&'a Token>{
    let label_line = &lexer.tokens[line_number];

    label_line.iter().find(|token| {
        match &token.token_type {
            Assembly8086Tokens::Character(label_token) => {
                label_token.eq_ignore_ascii_case(label)
            },
            _ => false
        }
    })
}