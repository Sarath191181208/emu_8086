pub mod compilation_error;
pub mod lexer;
pub mod tests;
pub mod tokens;

mod parsers;
pub(crate) mod tokenized_line;

use compilation_error::CompilationError;
use lexer::Lexer;
use tokens::instructions::Instructions;

use self::{
    parsers::{
        add::parse_add, dec::parse_dec, inc::parse_inc, jmp::parse_jmp, mov::parse_mov,
        sub::parse_sub,
    },
    tokenized_line::TokenizedLine,
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

fn has_consumed_all_instructions(
    lexed_str_without_spaces: &Vec<&Token>,
    i: usize,
    instruction: &str,
    num_args: usize,
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
            &format!(
                "Can't compile starting with {:?} as the {} instuction only supports {} arguments",
                unparsed_tokens_start.token_type, instruction, num_args
            ),
        ));
    }
    Ok(())
}

fn check_is_label(lexed_str_without_spaces: &Vec<&Token>) -> Option<String> {
    // return false if len < 2
    // check if the first token is a Character
    // check if the second token is a colon
    if lexed_str_without_spaces.len() < 2 {
        return None;
    }
    let first_token = lexed_str_without_spaces[0];
    let second_token = lexed_str_without_spaces[1];

    match &first_token.token_type {
        Assembly8086Tokens::Character(str) => {
            if let Assembly8086Tokens::Colon = second_token.token_type {
                return Some(str.to_string());
            }
        }
        _ => (),
    }

    None
}

struct CompiledLine {
    compiled_bytes: Vec<u8>,
    compiled_bytes_ref: Vec<CompiledBytes>,
    is_label: Option<String>,
    label_idx_map: std::collections::HashMap<String, (Token, u16)>,
}

impl CompiledLine {
    fn new() -> Self {
        Self {
            compiled_bytes: Vec::new(),
            compiled_bytes_ref: Vec::new(),
            is_label: None,
            label_idx_map: std::collections::HashMap::new(),
        }
    }
}

fn compile(lexed_strings: &[Token]) -> Result<CompiledLine, CompilationError> {
    let mut i = 0;
    let mut compiled_line = CompiledLine::new();
    let mut compiled_bytes = &mut compiled_line.compiled_bytes;
    let mut compiled_bytes_ref = &mut compiled_line.compiled_bytes_ref;

    let lexed_str_without_spaces = lexed_strings
        .iter()
        .filter(|token| token.token_type != Assembly8086Tokens::Space)
        .take_while(|token| token.token_type != Assembly8086Tokens::Comment)
        .collect::<Vec<&Token>>();
    let last_token = match lexed_str_without_spaces.last() {
        Some(token) => token,
        None => return Ok(compiled_line),
    };
    if last_token.token_type == Assembly8086Tokens::Space {
        return Ok(compiled_line);
    }
    let len_lexed_strings = last_token.token_length + last_token.column_number;
    if let Some(val) = check_is_label(&lexed_str_without_spaces) {
        compiled_line.is_label = Some(val);
        i += 2;
    }
    if i >= lexed_str_without_spaces.len() {
        return Ok(compiled_line);
    }
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

    let tokenized_line = TokenizedLine::new(&lexed_str_without_spaces, len_lexed_strings);
    match instruction {
        Instructions::Mov => {
            i = parse_mov(
                &tokenized_line,
                i,
                &mut compiled_bytes,
                &mut compiled_bytes_ref,
            )?;
            has_consumed_all_instructions(&lexed_str_without_spaces, i, "MOV", 2)?
        }

        Instructions::Add => {
            i = parse_add(
                &tokenized_line,
                i,
                &mut compiled_bytes,
                &mut compiled_bytes_ref,
            )?;

            has_consumed_all_instructions(&lexed_str_without_spaces, i, "ADD", 2)?;
        }

        Instructions::Inc => {
            i = parse_inc(
                &tokenized_line,
                i,
                &mut compiled_bytes,
                &mut compiled_bytes_ref,
            )?;
            has_consumed_all_instructions(&lexed_str_without_spaces, i, "INC", 1)?;
        }

        Instructions::Dec => {
            i = parse_dec(
                &tokenized_line,
                i,
                &mut compiled_bytes,
                &mut compiled_bytes_ref,
            )?;

            has_consumed_all_instructions(&lexed_str_without_spaces, i, "DEC", 1)?;
        }

        Instructions::Sub => {
            i = parse_sub(
                &tokenized_line,
                i,
                &mut compiled_bytes,
                &mut compiled_bytes_ref,
            )?;

            has_consumed_all_instructions(&lexed_str_without_spaces, i, "SUB", 2)?;
        }

        Instructions::JMP => {
            i = parse_jmp(
                &tokenized_line,
                i,
                &mut compiled_bytes,
                &mut compiled_bytes_ref,
                &mut compiled_line.label_idx_map,
            )?;

            has_consumed_all_instructions(&lexed_str_without_spaces, i, "JMP", 1)?;
        }
    }
    Ok(compiled_line)
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

    pub fn print_with_compiled_tokens(&self, compiled_tokens: &[CompiledBytes]) {
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

pub fn compile_lines(
    code: &str,
    debug_print: bool,
) -> Result<(Vec<u8>, Vec<CompiledBytes>), Vec<CompilationError>> {
    let mut lexer = Lexer::new();
    lexer.tokenize(code);

    let mut compilation_errors = Vec::<CompilationError>::new();
    let mut compiled_bytes = Vec::new();
    let mut compiled_bytes_ref = Vec::new();
    let mut label_addr_map = std::collections::HashMap::<String, usize>::new();
    let mut label_ref_map = std::collections::HashMap::<String, (Token, usize)>::new();

    for line in &lexer.tokens {
        match compile(line) {
            Ok(compiled_line) => {
                let mut compiled_bytes_line = compiled_line.compiled_bytes;
                let mut compiled_bytes_ref_line = compiled_line.compiled_bytes_ref;

                if let Some(label_str) = compiled_line.is_label {
                    label_addr_map.insert(label_str, compiled_bytes.len());
                }

                for (label_str, (token, idx)) in compiled_line.label_idx_map {
                    label_ref_map.insert(label_str, (token, compiled_bytes.len() + idx as usize));
                }

                compiled_bytes.append(&mut compiled_bytes_line);
                compiled_bytes_ref.append(&mut compiled_bytes_ref_line);
            }
            Err(err) => {
                compilation_errors.push(err);
            }
        }
    }

    for(lable_str, (token, mem_idx)) in label_ref_map{
        if let Some(label_addr) = label_addr_map.get(&lable_str){
            let label_addr = *label_addr as u16;
            let mem_idx: u16 = mem_idx as u16;
            if mem_idx > label_addr {
                let ins = (0xFF - (mem_idx-label_addr)) as u8;
                if ins < 0x80 {
                    compilation_errors.push(CompilationError::new(
                        token.line_number,
                        token.column_number,
                        token.token_length,
                        &format!(
                            "Can't compile starting with {:?} as the label {} is too far away this feature isn't supported yet",
                            token.token_type, lable_str
                        ),
                    ));
                }
                compiled_bytes[mem_idx as usize] = ins;
                compiled_bytes_ref[mem_idx as usize] = CompiledBytes::new(vec![ins], token.line_number, token.column_number);
            }

        }else{
            compilation_errors.push(CompilationError::new(
                token.line_number,
                token.column_number,
                token.token_length,
                &format!(
                    "Can't compile starting with {:?} as the label {} is not defined",
                    token.token_type, lable_str
                ),
            ));
        }
    }

    if debug_print {
        lexer.print_with_compiled_tokens(&compiled_bytes_ref);
    }

    if !compilation_errors.is_empty() {
        return Err(compilation_errors);
    }
    Ok((compiled_bytes, compiled_bytes_ref))
}

pub fn compile_str(
    code: &str,
    debug_print: bool,
) -> Result<(Vec<u8>, Vec<CompiledBytes>), Vec<CompilationError>> {
    let (compiled_bytes, compiled_bytes_ref) = compile_lines(code, debug_print)?;
    Ok((compiled_bytes, compiled_bytes_ref))
}
