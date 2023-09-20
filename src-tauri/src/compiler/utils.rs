use super::tokens::{instructions::Instructions, Assembly8086Tokens, Token};

pub(in crate::compiler) fn get_jmp_code_compiled_line(token: &Token) -> Vec<Token> {
    [
        Token::new(
            Assembly8086Tokens::Instruction(Instructions::Jmp),
            token.line_number,
            token.column_number,
            token.token_length,
        ),
        Token::new(
            Assembly8086Tokens::Character("code".to_string()),
            token.line_number,
            token.column_number,
            token.token_length,
        ),
    ]
    .to_vec()
}
