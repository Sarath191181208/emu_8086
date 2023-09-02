pub mod compilation_error;
pub mod lexer;
pub mod tests;
pub mod tokens;


mod parsers;

use compilation_error::CompilationError;
use lexer::Lexer;
use tokens::instructions::Instructions;

use self::{
    parsers::{add::parse_add, mov::parse_mov, inc::parse_inc},
    tokens::{Assembly8086Tokens, Token},
};

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
    // error if the token type isn't an instruction
    let instruction = match &token.token_type {
        Assembly8086Tokens::Instruction(instruction) => instruction,
        _ => {
            return Err(CompilationError::new(
                token.line_number,
                token.column_number,
                token.token_length,
                &format!(
                    "Can't compile starting with {:?} as the first token must be an instruction",
                    token.token_type
                ),
            ));
        }
    };

    match instruction {
        Instructions::MOV => {
            i = parse_mov(
                &lexed_str_without_spaces,
                token,
                i,
                len_lexed_strings,
                &mut compiled_bytes,
                &mut compiled_bytes_ref,
            )?;

            // check if i hasn't consumed all the instructions
            if i < lexed_str_without_spaces.len() - 1 {
                let unparsed_tokens_start = lexed_str_without_spaces[i + 1];
                return Err(CompilationError::new(
                    unparsed_tokens_start.line_number,
                    unparsed_tokens_start.column_number,
                    (len_lexed_strings - unparsed_tokens_start.column_number) as u32,
                    &format!("Can't compile starting with {:?} as the MOV instuction only supports 2 arguments", unparsed_tokens_start.token_type),
                ));
            }
        }

        Instructions::ADD => {
            i = parse_add(
                &lexed_str_without_spaces,
                token,
                i,
                len_lexed_strings,
                &mut compiled_bytes,
                &mut compiled_bytes_ref,
            )?;

            // check if i hasn't consumed all the instructions
            if i < lexed_str_without_spaces.len() - 1 {
                let unparsed_tokens_start = lexed_str_without_spaces[i + 1];
                return Err(CompilationError::new(
                    unparsed_tokens_start.line_number,
                    unparsed_tokens_start.column_number,
                    (len_lexed_strings - unparsed_tokens_start.column_number) as u32,
                    &format!("Can't compile starting with {:?} as the ADD instuction only supports 2 arguments", unparsed_tokens_start.token_type),
                ));
            }
        }

        Instructions::INC => {
            i = parse_inc(
                &lexed_str_without_spaces,
                token,
                i,
                len_lexed_strings,
                &mut compiled_bytes,
                &mut compiled_bytes_ref,
            )?;
        }
    }
    Ok((compiled_bytes, compiled_bytes_ref))
}

impl Lexer {
    // pub fn print_tokens(&self) {
    //     // print a formatted headding
    //     println!(
    //         "| {0: <20} | {1: <10} | {2: <10} | {3: <10} |",
    //         "Token", "Line", "Column", "Length"
    //     );
    //     for token in &self.tokens {
    //         println!(
    //             "| {0: <20} | {1: <10} | {2: <10} | {3: <10} |",
    //             format!("{:?}", token.token_type),
    //             token.line_number,
    //             token.column_number,
    //             token.token_length
    //         );
    //     }
    // }

    pub fn print_with_compiled_tokens(&self, compiled_tokens: &Vec<CompiledBytes>) {
        // print a formatted headding
        println!(
            "| {0: <20} | {1: <10} | {2: <10} | {3: <10} | {4: <10} |",
            "Token", "Line", "Column", "Length", "Bytes"
        );

        for token_list in &self.tokens {
            // find the compiled token that matches the line and column number
            for token in token_list {
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
}

pub fn compile_lines(code: &str, debug_print: bool) -> Result<(Vec<u8>, Vec<CompiledBytes>), Vec<CompilationError>> {
    let mut lexer = Lexer::new();
    lexer.tokenize(&code);

    let mut compilation_errors = Vec::<CompilationError>::new();
    let mut compiled_bytes = Vec::new();
    let mut compiled_bytes_ref = Vec::<CompiledBytes>::new();

    for line in &lexer.tokens{
        match compile(&line){
            Ok((mut compiled_bytes_line, mut compiled_bytes_ref_line)) => {
                compiled_bytes.append(&mut compiled_bytes_line);
                compiled_bytes_ref.append(&mut compiled_bytes_ref_line);
            }
            Err(err) => {
                compilation_errors.push(err);
            }
        }
    }

    if debug_print {
        lexer.print_with_compiled_tokens(&compiled_bytes_ref);
    }

    if compilation_errors.len() > 0 {
        return Err(compilation_errors);
    }
    return Ok((compiled_bytes, compiled_bytes_ref));
}

pub fn compile_str(
    code: &str,
    debug_print: bool,
) -> Result<(Vec<u8>, Vec<CompiledBytes>), CompilationError> {
    let mut lexer = Lexer::new();
    lexer.tokenize(&code);

    let (compiled_bytes, compiled_bytes_ref) = match compile(&lexer.tokens[0]) {
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
