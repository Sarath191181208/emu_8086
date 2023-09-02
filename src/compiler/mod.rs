pub mod compilation_error;
pub mod lexer;
pub mod tokens;
pub mod tests;

use compilation_error::CompilationError;
use lexer::Lexer;
use tokens::instructions::Instructions;

use self::tokens::{Assembly8086Tokens, Token};

#[derive(Debug)]
pub struct CompiledBytes {
    bytes: Vec<u8>,

    line_number: u32,
    column_number: u32,
}

impl CompiledBytes {
    pub fn new(bytes: Vec<u8>, line_number: u32, column_number: u32) -> Self {
        Self {
            bytes,
            line_number,
            column_number,
        }
    }
}

fn compile(lexed_strings: &Vec<Token>) -> Result<(Vec<u8>, Vec<CompiledBytes>), CompilationError> {
    let mut compiled_bytes = Vec::new();
    let mut compiled_bytes_ref = Vec::<CompiledBytes>::new();
    let mut i = 0;
    let lexed_str_without_spaces = lexed_strings
        .iter()
        .filter(|token| token.token_type != Assembly8086Tokens::Space)
        .collect::<Vec<&Token>>();
    let last_token = match lexed_strings.last() {
        Some(token) => token,
        None => return Ok((compiled_bytes, compiled_bytes_ref)),
    };
    let len_lexed_strings = last_token.token_length + last_token.column_number;
    let token = lexed_str_without_spaces[i];
    match token.token_type {
        Assembly8086Tokens::Instruction(Instructions::MOV) => {
            if lexed_str_without_spaces.len() - 1 < i + 1 {
                return Err(CompilationError::new(
                    token.line_number,
                    token.column_number + token.token_length,
                    (len_lexed_strings + 1) as u32,
                    "Insufficient arguments to MOV",
                ));
            }
            let high_token = lexed_str_without_spaces[i + 1];
            match &high_token.token_type {
                Assembly8086Tokens::Register16bit(high_reg) => {
                    if i + 3 > lexed_str_without_spaces.len() - 1 {
                        return Err(CompilationError::new(
                            high_token.line_number,
                            high_token.column_number + high_token.token_length + 1,
                            (len_lexed_strings + 1) as u32,
                            "Insufficient arguments to MOV expected a 16bit value ",
                        ));
                    }
                    let low_token = lexed_str_without_spaces[i + 3];
                    let high_reg_idx = match high_reg.get_as_idx() {
                        Ok(idx) => idx,
                        Err(err) => {
                            return Err(CompilationError::new(
                                token.line_number,
                                high_token.column_number,
                                high_token.token_length,
                                err,
                            ));
                        }
                    };
                    match &low_token.token_type {
                        Assembly8086Tokens::Number16bit(number) => {
                            compiled_bytes.push(0xB8 | high_reg_idx);
                            compiled_bytes.push((number & 0xFF) as u8);
                            compiled_bytes.push((number >> 8) as u8);

                            compiled_bytes_ref.push(CompiledBytes::new(
                                vec![0xB8],
                                token.line_number,
                                token.column_number,
                            ));

                            compiled_bytes_ref.push(CompiledBytes::new(
                                vec![(number & 0xFF) as u8, (number >> 8) as u8],
                                low_token.line_number,
                                low_token.column_number,
                            ));
                            i += 3;
                        }
                        Assembly8086Tokens::Register16bit(low_reg) => {
                            let low_reg_idx = match low_reg.get_as_idx() {
                                Ok(idx) => idx,
                                Err(err) => {
                                    return Err(CompilationError::new(
                                        token.line_number,
                                        token.column_number,
                                        token.token_length,
                                        err,
                                    ));
                                }
                            };
                            compiled_bytes.push(0x8B);
                            let ins = (0xC0) | (high_reg_idx / 2) << 4;
                            let ins2 = low_reg_idx | (high_reg_idx % 2) << 3;
                            compiled_bytes.push(ins | ins2);

                            compiled_bytes_ref.push(CompiledBytes::new(
                                vec![0x8B],
                                token.line_number,
                                token.column_number,
                            ));
                            compiled_bytes_ref.push(CompiledBytes::new(
                                vec![ins2 | ins],
                                low_token.line_number,
                                low_token.column_number,
                            ));
                            i += 2;
                        }
                        _ => {
                            return Err(CompilationError::new(
                                token.line_number,
                                high_token.column_number + high_token.token_length + 1,
                                (len_lexed_strings
                                    - high_token.column_number
                                    - high_token.token_length)
                                    as u32,
                                &format!(
                                    "Expected a 16bit value after MOV got {:?} insted",
                                    &low_token.token_type
                                ),
                            ));
                        }
                    }
                }

                Assembly8086Tokens::Register8bit(high_reg) => {
                    if i + 3 > lexed_str_without_spaces.len() - 1 {
                        return Err(CompilationError::new(
                            high_token.line_number,
                            high_token.column_number,
                            (len_lexed_strings + 1) as u32,
                            "Insufficient arguments to MOV expected a 8bit value ",
                        ));
                    }
                    let low_token = lexed_str_without_spaces[i + 3];
                    match &low_token.token_type {
                        Assembly8086Tokens::Number8bit(number) => {
                            compiled_bytes.push(0xB0 | high_reg.get_as_idx());
                            compiled_bytes.push(*number);

                            compiled_bytes_ref.push(CompiledBytes::new(
                                vec![0xB0],
                                token.line_number,
                                token.column_number,
                            ));
                            compiled_bytes_ref.push(CompiledBytes::new(
                                vec![*number],
                                low_token.line_number,
                                low_token.column_number,
                            ));
                            i += 3;
                        }
                        Assembly8086Tokens::Register8bit(low_reg) => {
                            compiled_bytes.push(0x8A);
                            let ins = (0xC0) | (high_reg.get_as_idx() / 2) << 4;
                            let ins2 =
                                (low_reg.get_as_idx() / 2) | (high_reg.get_as_idx() % 2) << 3;
                            compiled_bytes.push(ins);
                            compiled_bytes.push(ins2);

                            compiled_bytes_ref.push(CompiledBytes::new(
                                vec![0x8A],
                                token.line_number,
                                token.column_number,
                            ));

                            compiled_bytes_ref.push(CompiledBytes::new(
                                vec![ins],
                                high_token.line_number,
                                high_token.column_number,
                            ));
                            i += 2;
                        }
                        _ => {
                            return Err(CompilationError::new(
                                high_token.line_number,
                                high_token.column_number + high_token.token_length + 1,
                                (len_lexed_strings
                                    - high_token.column_number
                                    - high_token.token_length)
                                    as u32,
                                &format!(
                                    "Expected a 8bit value after MOV got {:?} insted",
                                    &low_token.token_type
                                ),
                            ));
                        }
                    }
                }

                _ => {
                    return Err(CompilationError::new(
                        high_token.line_number,
                        high_token.column_number,
                        high_token.token_length,
                        &format!(
                            "Expected a 16bit or 8bit register after MOV got {:?} insted",
                            &high_token.token_type
                        ),
                    ));
                }
            }
        }
        _ => panic!("Not implemented"),
    }
    Ok((compiled_bytes, compiled_bytes_ref))
}

impl Lexer{
        pub fn print_tokens(&self) {
        // print a formatted headding
        println!(
            "| {0: <20} | {1: <10} | {2: <10} | {3: <10} |",
            "Token", "Line", "Column", "Length"
        );
        for token in &self.tokens {
            println!(
                "| {0: <20} | {1: <10} | {2: <10} | {3: <10} |",
                format!("{:?}", token.token_type),
                token.line_number,
                token.column_number,
                token.token_length
            );
        }
    }

    pub fn print_with_compiled_tokens(&self, compiled_tokens: &Vec<CompiledBytes>) {
        // print a formatted headding
        println!(
            "| {0: <20} | {1: <10} | {2: <10} | {3: <10} | {4: <10} |",
            "Token", "Line", "Column", "Length", "Bytes"
        );

        for token in &self.tokens {
            // find the compiled token that matches the line and column number
            let matched_compiled_token = compiled_tokens.iter().find(|compiled_token| {
                compiled_token.line_number == token.line_number
                    && compiled_token.column_number == token.column_number
            });
            // reduce the bytes to a string
            let bytes = match matched_compiled_token {
                Some(compiled_token) => {
                    let mut bytes_string = String::new();
                    for byte in &compiled_token.bytes {
                        bytes_string.push_str(&format!("{:02X} ", byte));
                    }
                    bytes_string
                }
                None => String::new(),
            };
            println!(
                "| {0: <20} | {1: <10} | {2: <10} | {3: <10} | {4: <10} |",
                format!("{:?}", token.token_type),
                token.line_number,
                token.column_number,
                token.token_length,
                bytes
            );
        }
    }

}

pub fn compile_str(
    code: &str,
    debug_print: bool,
) -> Result<(Vec<u8>, Vec<CompiledBytes>), CompilationError> {
    let mut lexer = Lexer::new();
    lexer.tokenize(&code);

    let (compiled_bytes, compiled_bytes_ref) = match compile(&lexer.tokens) {
        Ok((compiled_bytes, compiled_bytes_ref)) => (compiled_bytes, compiled_bytes_ref),
        Err(err) => {
            return Err(err);
        }
    };

    if debug_print {
        lexer.print_with_compiled_tokens(&compiled_bytes_ref);
    }

    Ok((compiled_bytes, compiled_bytes_ref))
}