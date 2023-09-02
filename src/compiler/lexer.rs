use super::tokens::{
    instructions::Instructions, registers16bit::Registers16bit, registers8bit::Registers8bit,
    Assembly8086Tokens, Token,
};

#[derive(Debug)]
pub(crate) struct Lexer {
    pub tokens: Vec<Token>,
}

impl Lexer {
    pub fn new() -> Lexer {
        Lexer { tokens: Vec::new() }
    }

    pub fn tokenize(&mut self, source: &str) {
        for (line_number, line) in source.lines().enumerate() {
            let line_number: u32 = line_number as u32;
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
                        self.tokens.push(Token::new(
                            Assembly8086Tokens::Space,
                            line_number,
                            iterating_col_num as u32,
                            num_spaces as u32,
                        ));
                        iterating_col_num += num_spaces;
                    }
                    ',' => {
                        self.tokens.push(Token::new(
                            Assembly8086Tokens::Comma,
                            line_number,
                            iterating_col_num as u32,
                            1,
                        ));
                        iterating_col_num += 1;
                    }
                    ';' => {
                        self.tokens.push(Token::new(
                            Assembly8086Tokens::Comment,
                            line_number,
                            iterating_col_num as u32,
                            1,
                        ));
                        iterating_col_num = line.len();
                    }
                    _ => {
                        let mut token_length = 0;
                        let mut token_string_buffer = String::new();
                        let mut i = iterating_col_num;
                        while i < line.len() {
                            let c = line_chars[i];
                            if c.is_whitespace() || c == ',' || c == ';' {
                                break;
                            }
                            i += 1;
                            token_length += 1;
                            token_string_buffer.push(c);
                        }
                        let token = self.str_to_token(&token_string_buffer.to_lowercase());
                        self.tokens.push(Token::new(
                            match token {
                                Some(token) => token,
                                None => Assembly8086Tokens::Error,
                            },
                            line_number,
                            iterating_col_num as u32,
                            token_length as u32,
                        ));
                        iterating_col_num += token_length;
                    }
                }
            }
        }
    }

    fn str_to_token(&self, token_string: &str) -> Option<Assembly8086Tokens> {
        // create a dict with all the tokens
        let tokens = vec![
            (Assembly8086Tokens::Register16bit(Registers16bit::AX), "ax"),
            (Assembly8086Tokens::Register16bit(Registers16bit::BX), "bx"),
            (Assembly8086Tokens::Register16bit(Registers16bit::CX), "cx"),
            (Assembly8086Tokens::Register16bit(Registers16bit::DX), "dx"),
            (Assembly8086Tokens::Register16bit(Registers16bit::SI), "si"),
            (Assembly8086Tokens::Register16bit(Registers16bit::DI), "di"),
            (Assembly8086Tokens::Register16bit(Registers16bit::BP), "bp"),
            (Assembly8086Tokens::Register16bit(Registers16bit::SP), "sp"),
            (Assembly8086Tokens::Register16bit(Registers16bit::CS), "cs"),
            (Assembly8086Tokens::Register16bit(Registers16bit::DS), "ds"),
            (Assembly8086Tokens::Register16bit(Registers16bit::ES), "es"),
            (Assembly8086Tokens::Register16bit(Registers16bit::SS), "ss"),
            (Assembly8086Tokens::Register16bit(Registers16bit::IP), "ip"),
            (Assembly8086Tokens::Register8bit(Registers8bit::AH), "ah"),
            (Assembly8086Tokens::Register8bit(Registers8bit::AL), "al"),
            (Assembly8086Tokens::Register8bit(Registers8bit::BH), "bh"),
            (Assembly8086Tokens::Register8bit(Registers8bit::BL), "bl"),
            (Assembly8086Tokens::Register8bit(Registers8bit::CH), "ch"),
            (Assembly8086Tokens::Register8bit(Registers8bit::CL), "cl"),
            (Assembly8086Tokens::Register8bit(Registers8bit::DH), "dh"),
            (Assembly8086Tokens::Register8bit(Registers8bit::DL), "dl"),
            (Assembly8086Tokens::Instruction(Instructions::MOV), "mov"),
            (Assembly8086Tokens::Instruction(Instructions::ADD), "add"),
        ];

        // iterate the dict and check if the token_string is in the dict
        for (token, token_string_in_dict) in tokens {
            if token_string == token_string_in_dict {
                return Some(token);
            }
        }
        if let Some(token) = self.parse_num_u8(token_string) {
            return Some(token);
        }
        if let Some(token) = self.parse_num_u16(token_string) {
            return Some(token);
        }
        None
    }

    fn parse_num_u16(&self, token_string: &str) -> Option<Assembly8086Tokens> {
        // try to parse numberr that is in 0x0011 format and
        // also in the format of 011h and also in the base 10 format
        // and return the number in the base 10 format
        if token_string.starts_with("0x") {
            if let Ok(number) = u16::from_str_radix(&token_string[2..], 16) {
                return Some(Assembly8086Tokens::Number16bit(number));
            }
        } else if token_string.ends_with("h") {
            if let Ok(number) = u16::from_str_radix(&token_string[..token_string.len() - 1], 16) {
                return Some(Assembly8086Tokens::Number16bit(number));
            }
        } else {
            if let Ok(number) = token_string.parse::<u16>() {
                return Some(Assembly8086Tokens::Number16bit(number));
            }
        }
        None
    }

    fn parse_num_u8(&self, token_string: &str) -> Option<Assembly8086Tokens> {
        // try to parse numberr that is in 0x0011 format and
        // also in the format of 011h and also in the base 10 format
        // and return the number in the base 10 format
        if token_string.starts_with("0x") {
            if let Ok(number) = u8::from_str_radix(&token_string[2..], 16) {
                return Some(Assembly8086Tokens::Number8bit(number));
            }
        } else if token_string.ends_with("h") {
            if let Ok(number) = u8::from_str_radix(&token_string[..token_string.len() - 1], 16) {
                return Some(Assembly8086Tokens::Number8bit(number));
            }
        } else {
            if let Ok(number) = token_string.parse::<u8>() {
                return Some(Assembly8086Tokens::Number8bit(number));
            }
        }
        None
    }
}
