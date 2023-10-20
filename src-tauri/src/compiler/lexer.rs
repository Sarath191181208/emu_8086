use std::str::FromStr;

use unicase::UniCase;

use super::tokens::{
    assembler_directives::AssemblerDirectives, data::DefineData, instructions::Instructions,
    registers16bit::Registers16bit, registers8bit::Registers8bit, Assembly8086Tokens, Token,
};

#[derive(Debug, Clone)]
pub(crate) struct Lexer {
    pub tokens: Vec<Vec<Token>>,
}

impl Lexer {
    pub fn new() -> Lexer {
        Lexer { tokens: Vec::new() }
    }

    pub fn tokenize(&mut self, source: &str) {
        for (line_number, line) in source.lines().enumerate() {
            let line_number: u32 = line_number as u32;
            let mut temp_vec = Vec::<Token>::new();
            let mut iterating_col_num = 0;
            // convert the string into array of chars don't use line.chars()
            let line_chars: Vec<char> = line.chars().collect();
            while iterating_col_num < line.len() {
                match line_chars[iterating_col_num] {
                    ' ' => {
                        // count the number of spaces
                        let num_spaces = line_chars
                            .iter()
                            .skip(iterating_col_num)
                            .take_while(|c| c.is_whitespace())
                            .count();
                        temp_vec.push(Token::new(
                            Assembly8086Tokens::Space,
                            line_number,
                            iterating_col_num as u32,
                            num_spaces as u32,
                        ));
                        iterating_col_num += num_spaces;
                    }
                    ',' => {
                        temp_vec.push(Token::new(
                            Assembly8086Tokens::Comma,
                            line_number,
                            iterating_col_num as u32,
                            1,
                        ));
                        iterating_col_num += 1;
                    }
                    ';' => {
                        temp_vec.push(Token::new(
                            Assembly8086Tokens::Comment,
                            line_number,
                            iterating_col_num as u32,
                            1,
                        ));
                        iterating_col_num = line.len();
                    }
                    ':' => {
                        temp_vec.push(Token::new(
                            Assembly8086Tokens::Colon,
                            line_number,
                            iterating_col_num as u32,
                            1,
                        ));
                        iterating_col_num += 1;
                    }
                    '[' => {
                        temp_vec.push(Token::new(
                            Assembly8086Tokens::OpenSquareBracket,
                            line_number,
                            iterating_col_num as u32,
                            1,
                        ));
                        iterating_col_num += 1;
                    }
                    ']' => {
                        temp_vec.push(Token::new(
                            Assembly8086Tokens::CloseSquareBracket,
                            line_number,
                            iterating_col_num as u32,
                            1,
                        ));
                        iterating_col_num += 1;
                    }
                    '+' => {
                        temp_vec.push(Token::new(
                            Assembly8086Tokens::Plus,
                            line_number,
                            iterating_col_num as u32,
                            1,
                        ));
                        iterating_col_num += 1;
                    }
                    '-' => {
                        temp_vec.push(Token::new(
                            Assembly8086Tokens::Minus,
                            line_number,
                            iterating_col_num as u32,
                            1,
                        ));
                        iterating_col_num += 1;
                    }
                    _ => {
                        let mut token_length = 0;
                        let mut token_string_buffer = String::new();
                        let mut i = iterating_col_num;
                        while i < line.len() {
                            let c = line_chars[i];
                            if c.is_whitespace()
                                || c == ','
                                || c == ';'
                                || c == ':'
                                || c == '['
                                || c == ']'
                                || c == '+'
                                || c == '-'
                            {
                                break;
                            }
                            i += 1;
                            token_length += 1;
                            token_string_buffer.push(c);
                        }
                        let token = self.str_to_token(&token_string_buffer);
                        temp_vec.push(Token::new(
                            match token {
                                Some(token) => token,
                                None => Assembly8086Tokens::Character(UniCase::new(
                                    token_string_buffer.clone(),
                                )),
                            },
                            line_number,
                            iterating_col_num as u32,
                            token_length as u32,
                        ));
                        iterating_col_num += token_length;
                    }
                }
            }
            self.tokens.push(temp_vec);
        }
    }

    fn str_to_token(&self, token_string: &str) -> Option<Assembly8086Tokens> {
        if let Ok(directive) = AssemblerDirectives::from_str(token_string) {
            return Some(Assembly8086Tokens::AssemblerDirectives(directive));
        }
        if let Ok(instruction) = Instructions::from_str(token_string) {
            return Some(Assembly8086Tokens::Instruction(instruction));
        }

        if let Ok(register) = Registers16bit::from_str(token_string) {
            return Some(Assembly8086Tokens::Register16bit(register));
        }

        if let Ok(register) = Registers8bit::from_str(token_string) {
            return Some(Assembly8086Tokens::Register8bit(register));
        }

        if let Ok(define_data) = DefineData::from_str(token_string) {
            return Some(Assembly8086Tokens::Data(define_data));
        }

        if let Some(token) = self.parse_num_u8(&token_string.replace('X', "x")) {
            return Some(token);
        }
        if let Some(token) = self.parse_num_u16(&token_string.to_lowercase()) {
            return Some(token);
        }

        None
    }

    fn parse_num_u16(&self, token_string: &str) -> Option<Assembly8086Tokens> {
        // try to parse numberr that is in 0x0011 format and
        // also in the format of 011h and also in the base 10 format
        // and return the number in the base 10 format
        if let Some(stripped_token) = token_string.strip_prefix("0x") {
            if let Ok(number) = u16::from_str_radix(stripped_token, 16) {
                return Some(Assembly8086Tokens::Number16bit(number));
            }
        } else if let Some(stripped_token) = token_string.strip_suffix('h') {
            if let Ok(number) = u16::from_str_radix(stripped_token, 16) {
                return Some(Assembly8086Tokens::Number16bit(number));
            }
        } else if let Some(stripped_token) = token_string.strip_suffix('b') {
            if let Ok(number) = u16::from_str_radix(stripped_token, 2) {
                return Some(Assembly8086Tokens::Number16bit(number));
            }
        } else if let Ok(number) = token_string.parse::<u16>() {
            return Some(Assembly8086Tokens::Number16bit(number));
        }

        None
    }

    fn parse_num_u8(&self, token_string: &str) -> Option<Assembly8086Tokens> {
        // try to parse numberr that is in 0x0011 format and
        // also in the format of 011h and also in the base 10 format
        // and return the number in the base 10 format
        if let Some(stripped_token) = token_string.strip_prefix("0x") {
            if let Ok(number) = u8::from_str_radix(stripped_token, 16) {
                return Some(Assembly8086Tokens::Number8bit(number));
            }
        } else if let Some(stripped_token) = token_string.strip_suffix('h') {
            if let Ok(number) = u8::from_str_radix(stripped_token, 16) {
                return Some(Assembly8086Tokens::Number8bit(number));
            }
        } else if let Some(stripped_token) = token_string.strip_suffix('b') {
            if let Ok(number) = u8::from_str_radix(stripped_token, 2) {
                return Some(Assembly8086Tokens::Number8bit(number));
            }
        } else if let Ok(number) = token_string.parse::<u8>() {
            return Some(Assembly8086Tokens::Number8bit(number));
        }

        None
    }
}

impl Default for Lexer {
    fn default() -> Self {
        Self::new()
    }
}
