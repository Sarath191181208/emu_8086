use super::{
    compilation_error::CompilationError,
    suggestions::SuggestionType,
    tokens::{Assembly8086Tokens, Token},
};

pub struct TokenizedLine<'a> {
    pub(crate) tokens: &'a Vec<&'a Token>,
    len_lexed_strings: u32,
}

impl<'a> TokenizedLine<'a> {
    pub(crate) fn new(tokens: &'a Vec<&'a Token>, len_lexed_strings: u32) -> Self {
        Self {
            tokens,
            len_lexed_strings,
        }
    }

    pub fn len(&self) -> usize {
        self.tokens.len()
    }

    pub fn get_len_lexed_strings(&self) -> u32 {
        self.len_lexed_strings
    }

    pub(in crate::compiler) fn slice(&self, start: usize, end: usize) -> &[&Token] {
        &self.tokens[start..end]
    }

    pub(crate) fn find_token(&self, token: Assembly8086Tokens) -> Option<usize> {
        self.tokens
            .iter()
            .position(|token_| token_.token_type == token)
    }

    pub(crate) fn find_comma(&self) -> Option<usize> {
        self.tokens
            .iter()
            .position(|token| token.token_type == Assembly8086Tokens::Comma)
    }

    pub(crate) fn get(
        &self,
        i: usize,
        exception_str: String,
        possible_suggestions: Option<Vec<Vec<SuggestionType>>>,
    ) -> Result<&'a Token, CompilationError> {
        if i > self.tokens.len() - 1 {
            let last_token = match self.tokens.last() {
                Some(token) => token,
                None => {
                    return Err(CompilationError::new_without_suggestions(
                        0,
                        0,
                        0,
                        "Err: No tokens found this shouldn't happen, Please report this issue!",
                    ))
                }
            };
            return Err(CompilationError::new_with_suggestions(
                last_token.line_number,
                last_token.column_number + last_token.token_length,
                self.len_lexed_strings + 1,
                &exception_str,
                possible_suggestions.unwrap_or_default(),
            ));
        }
        Ok(self.tokens[i])
    }
}
