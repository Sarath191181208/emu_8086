use unicase::UniCase;

use super::{
    compilation_error::CompilationError,
    lexer::Lexer,
    tokens::{instructions::Instructions, Assembly8086Tokens, Token},
    types_structs::{CompiledBytesReference, Label, LineNumber},
};

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

impl CompilationError {
    pub(super) fn error_with_token(token: &Token, msg: &str) -> Self {
        CompilationError::new_without_suggestions(
            token.line_number,
            token.column_number,
            token.token_length,
            msg,
        )
    }

    pub(super) fn error_line(line: u32, msg: &str) -> Self {
        CompilationError::new_without_suggestions(line, 0, u8::MAX as u32 , msg)
    }

    pub(super) fn error_between_tokens(token1: &Token, token2: &Token, msg: &str) -> Self {
        CompilationError::new_without_suggestions(
            token1.line_number,
            token1.column_number,
            (token2.column_number + token2.token_length) - token1.column_number,
            msg,
        )
    }

    pub(super) fn same_error_on_two_tokens(token1: &Token, token2: &Token, msg: &str) -> [Self; 2] {
        [
            CompilationError::new_without_suggestions(
                token1.line_number,
                token1.column_number,
                token1.token_length,
                msg,
            ),
            CompilationError::new_without_suggestions(
                token2.line_number,
                token2.column_number,
                token2.token_length,
                msg,
            ),
        ]
    }
}

pub(crate) fn get_label_token_from_line<'a>(
    lexer: &'a Lexer,
    line_number: usize,
    label: &Label,
) -> Option<&'a Token> {
    let label_line = &lexer.tokens[line_number];

    label_line.iter().find(|token| match &token.token_type {
        Assembly8086Tokens::Character(label_token) => label_token.eq_ignore_ascii_case(label),
        _ => false,
    })
}

impl Lexer {
    pub fn print_with_compiled_tokens(&self, compiled_tokens: &[CompiledBytesReference]) {
        // print a formatted headding
        println!(
            "| {0: <30} | {1: <10} | {2: <10} | {3: <10} | {4: <10} |",
            "Token", "Line", "Column", "Length", "Bytes"
        );

        for token_list in &self.tokens {
            // find the compiled token that matches the line and column number
            for token in token_list {
                let mut bytes = String::new();
                for compiled_token in compiled_tokens {
                    if compiled_token.line_number == token.line_number
                        && compiled_token.column_number == token.column_number
                    {
                        for byte in &compiled_token.bytes {
                            bytes.push_str(&format!("{:02X} ", byte));
                        }
                    }
                }
                println!(
                    "| {0: <30} | {1: <10} | {2: <10} | {3: <10} | {4: <10} |",
                    format!("{}", token.token_type),
                    token.line_number,
                    token.column_number,
                    token.token_length,
                    bytes
                );
            }
            println!();
        }
    }
}
