use super::{compilation_error::CompilationError, tokens::Token};

pub struct TokenizedLine<'a> {
    tokens: &'a Vec<&'a Token>,
    len_lexed_strings: u32,
}

impl<'a> TokenizedLine<'a> {
    pub(crate) fn new(tokens: &'a Vec<&'a Token>, len_lexed_strings: u32) -> Self {
        Self {
            tokens,
            len_lexed_strings,
        }
    }

    pub fn get_len_lexed_strings(&self) -> u32 {
        self.len_lexed_strings
    }

    pub(crate) fn get(
        &self,
        i: usize,
        exception_str: String,
    ) -> Result<&'a Token, CompilationError> {
        if i > self.tokens.len() - 1 {
            let last_token = match self.tokens.last() {
                Some(token) => token,
                None => {
                    return Err(CompilationError::new(
                        0,
                        0,
                        0,
                        "Err: No tokens found this shouldn't happen, Please report this issue!",
                    ))
                }
            };
            return Err(CompilationError::new(
                last_token.line_number,
                last_token.column_number + last_token.token_length,
                self.len_lexed_strings + 1,
                &exception_str,
            ));
        }
        Ok(self.tokens[i])
    }
}
