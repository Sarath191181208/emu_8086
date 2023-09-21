use std::collections::HashMap;

use unicase::UniCase;

pub mod compilation_error;
pub mod lexer;
pub mod tests;
pub mod tokens;

pub(crate) mod compilation_utils;
mod parsers;
pub(crate) mod tokenized_line;
pub mod types_structs;
pub(crate) mod utils;

use compilation_error::CompilationError;
use lexer::Lexer;
use tokens::instructions::Instructions;

use self::{
    compilation_utils::{
        check_is_label,
        has_consumed_all_instructions, // find_data_line_num,
        is_org_defined,
    },
    parsers::{
        add::parse_add, dec::parse_dec, inc::parse_inc, jmp::parse_jmp, mov::parse_mov,
        mul::parse_mul, sub::parse_sub,
    },
    tokenized_line::TokenizedLine,
    tokens::{assembler_directives::AssemblerDirectives, Assembly8086Tokens, Token},
    types_structs::{CompiledBytes, CompiledLine, IsLabelBeforeRef, Label, LineNumber},
    utils::get_jmp_code_compiled_line,
};

fn strip_space_and_comments_and_iterate_labels(
    lexed_line: &[Token],
) -> (Vec<&Token>, Option<String>) {
    let lexed_str_without_spaces = lexed_line
        .iter()
        .filter(|token| token.token_type != Assembly8086Tokens::Space)
        .take_while(|token| token.token_type != Assembly8086Tokens::Comment)
        .collect::<Vec<&Token>>();

    if let Some(val) = check_is_label(&lexed_str_without_spaces) {
        return (lexed_str_without_spaces, Some(val));
    }

    (lexed_str_without_spaces, None)
}

fn get_instruction<'a>(
    lexed_str_without_spaces: &'a [&'a Token],
    i: usize,
) -> Result<&'a Instructions, CompilationError> {
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
    Ok(instruction)
}

fn compile(
    lexed_strings: &[Token],
    is_org_defined: bool,
) -> Result<CompiledLine, CompilationError> {
    let mut i = 0;
    let mut compiled_line = CompiledLine::new();

    let (lexed_str_without_spaces, label) =
        strip_space_and_comments_and_iterate_labels(lexed_strings);
    if let Some(label) = label {
        compiled_line.labels.push(label);
        i += 2;
    }
    let last_token = match lexed_str_without_spaces.last() {
        Some(token) => token,
        None => return Ok(compiled_line),
    };
    if last_token.token_type == Assembly8086Tokens::Space {
        return Ok(compiled_line);
    }
    let len_lexed_strings = last_token.token_length + last_token.column_number;
    if i >= lexed_str_without_spaces.len() {
        return Ok(compiled_line);
    }
    // error if the token type isn't an instruction
    let instruction = get_instruction(&lexed_str_without_spaces, i)?;
    let tokenized_line = TokenizedLine::new(&lexed_str_without_spaces, len_lexed_strings);

    if let Instructions::AssemblerDirectives(dir) = instruction {
        match dir {
            AssemblerDirectives::Org => {}
            AssemblerDirectives::Data => {
                if is_org_defined {
                    let token = lexed_str_without_spaces[i];
                    let jmp_ins = get_jmp_code_compiled_line(token);
                    let jmp_ins = jmp_ins.iter().collect::<Vec<&Token>>();
                    let mut temp_line = CompiledLine::new();
                    let _ = parse_jmp(
                        &TokenizedLine::new(&jmp_ins, 0),
                        0,
                        &mut temp_line.compiled_bytes,
                        &mut temp_line.compiled_bytes_ref,
                        &mut temp_line.label_idx_map,
                        None,
                    )?;
                    compiled_line.extend(temp_line);
                    i += 1;
                }
            }
            AssemblerDirectives::Code => {
                // push code into compiled_line.labels don't change the other values already in compiled_line
                compiled_line.labels.push("code".to_string());
            }
        }
    }

    let mut compiled_bytes = &mut compiled_line.compiled_bytes;
    let mut compiled_bytes_ref = &mut compiled_line.compiled_bytes_ref;

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

        Instructions::Mul => {
            i = parse_mul(
                &tokenized_line,
                i,
                &mut compiled_bytes,
                &mut compiled_bytes_ref,
            )?;
            has_consumed_all_instructions(&lexed_str_without_spaces, i, "MUL", 1)?;
        }

        Instructions::Jmp => {
            let _compliled_line = compile_labeled_instructions(lexed_strings, None)?;
            compiled_line.extend(_compliled_line);
        }
        Instructions::AssemblerDirectives(_) => (),
    }
    Ok(compiled_line)
}

struct CompiledLineLabelRef<'a> {
    compiled_bytes: &'a Vec<Vec<u8>>,
    line_num: LineNumber,
    label: &'a str,
    label_addr_map: &'a HashMap<UniCase<String>, LineNumber>,
    is_org_defined: bool,
}

fn unwrap_and_find_offset(
    compiled_line_label_ref: &Option<CompiledLineLabelRef>,
) -> Option<(u16, IsLabelBeforeRef)> {
    match compiled_line_label_ref {
        None => None,
        Some(line) => {
            let CompiledLineLabelRef {
                compiled_bytes,
                line_num,
                label,
                label_addr_map,
                ..
            } = line;
            let start_line = line_num;
            let end_line = *label_addr_map
                .get(&UniCase::new(label.to_string()))
                .unwrap();
            let val = calc_offset(compiled_bytes, *start_line, end_line);
            Some(val)
        }
    }
}

fn compile_labeled_instructions(
    lexed_strings: &[Token],
    compiled_line_label_ref: Option<CompiledLineLabelRef>,
) -> Result<CompiledLine, CompilationError> {
    let mut i = 0;
    // just pass the label if it's present
    let (lexed_str_without_spaces, label) =
        strip_space_and_comments_and_iterate_labels(lexed_strings);
    if label.is_some() {
        i += 2;
    }
    // helper vars
    let mut compiled_line = CompiledLine::new();
    let ins = get_instruction(&lexed_str_without_spaces, i)?;
    let token = lexed_str_without_spaces[i];

    let offset_bytes_from_line_and_is_label_before_ref =
        unwrap_and_find_offset(&compiled_line_label_ref);

    if let Instructions::AssemblerDirectives(AssemblerDirectives::Data) = ins {
        if let Some(line) = compiled_line_label_ref {
            if line.is_org_defined {
                let jmp_ins = get_jmp_code_compiled_line(token);
                let jmp_ins: Vec<&Token> = jmp_ins.iter().collect();
                let mut temp_line = CompiledLine::new();
                let offset_bytes_from_line_and_is_label_before_ref =
                    unwrap_and_find_offset(&Some(line));

                let _ = parse_jmp(
                    &TokenizedLine::new(&jmp_ins, 0),
                    0,
                    &mut temp_line.compiled_bytes,
                    &mut temp_line.compiled_bytes_ref,
                    &mut temp_line.label_idx_map,
                    offset_bytes_from_line_and_is_label_before_ref,
                )?;
                compiled_line.extend(temp_line);
                i += 1;
            }
        }
    }

    if i >= lexed_str_without_spaces.len() {
        return Ok(compiled_line);
    }

    let ins = get_instruction(&lexed_str_without_spaces, i)?;
    let token = lexed_str_without_spaces[i];
    let tokenized_line = TokenizedLine::new(&lexed_str_without_spaces, lexed_strings.len() as u32);

    match ins {
        Instructions::Jmp => {
            i = parse_jmp(
                &tokenized_line,
                i,
                &mut compiled_line.compiled_bytes,
                &mut compiled_line.compiled_bytes_ref,
                &mut compiled_line.label_idx_map,
                offset_bytes_from_line_and_is_label_before_ref,
            )?;

            has_consumed_all_instructions(&lexed_str_without_spaces, i, "JMP", 1)?;
        }
        _ => {
            return Err(CompilationError::new(
                token.line_number,
                token.column_number,
                token.token_length,
                &format!(
                    "Can't compile starting with {:?} this is an issue with the compiler, Please report this!",
                    token.token_type
                ),
            ));
        }
    }

    Ok(compiled_line)
}

impl Lexer {
    pub fn print_with_compiled_tokens(&self, compiled_tokens: &[CompiledBytes]) {
        // print a formatted headding
        println!(
            "| {0: <30} | {1: <10} | {2: <10} | {3: <10} | {4: <10} |",
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
                    "| {0: <30} | {1: <10} | {2: <10} | {3: <10} | {4: <10} |",
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

fn calc_offset(
    compiled_bytes: &[Vec<u8>],
    label_ref: LineNumber,
    label_addr: LineNumber,
) -> (u16, IsLabelBeforeRef) {
    let mut offset = 0;
    if label_addr < label_ref {
        // if label is refernced after it's defined
        for i in label_addr..label_ref {
            offset += compiled_bytes[i as usize].len();
        }
    } else {
        // i.e label is refernced before it is defined
        for i in (label_ref + 1)..label_addr {
            offset += compiled_bytes[i as usize].len();
        }
    }

    let is_label_before_ref = label_ref > label_addr;
    (offset as u16, is_label_before_ref)
}

fn mark_labels(
    label_ref: &Vec<(Label, LineNumber, &Vec<Token>)>,
    compiled_bytes: &mut Vec<Vec<u8>>,
    compiled_bytes_ref: &mut Vec<Vec<CompiledBytes>>,
    label_addr_map: &std::collections::HashMap<UniCase<String>, LineNumber>,
    is_org_defined: bool,
    idx: usize,
) -> Result<bool, CompilationError> {
    if idx >= label_ref.len() {
        return Ok(true);
    }
    let (label, line_number, tokenized_line) = &label_ref[idx];
    let line_number = *line_number;
    for _ in 0..(label_ref.len() - idx) {
        // let (offset_bytes, is_label_before_ref) = calc_offset(
        //     compiled_bytes,
        //     line_number,
        //     *label_addr_map.get(label).unwrap(),
        // );

        let compiled_tokens = compile_labeled_instructions(
            tokenized_line,
            Some(CompiledLineLabelRef {
                compiled_bytes,
                line_num: line_number,
                label,
                label_addr_map,
                is_org_defined,
            }),
        )?;

        compiled_bytes[line_number as usize] = compiled_tokens.compiled_bytes;
        compiled_bytes_ref[line_number as usize] = compiled_tokens.compiled_bytes_ref;

        let prev_compiled_bytes_len = compiled_bytes[line_number as usize].len();
        let curr_compiled_bytes_len = compiled_bytes[line_number as usize].len();
        if prev_compiled_bytes_len != curr_compiled_bytes_len {
            return Ok(false);
        }

        if mark_labels(
            label_ref,
            compiled_bytes,
            compiled_bytes_ref,
            label_addr_map,
            is_org_defined,
            idx + 1,
        )? {
            return Ok(true);
        }
    }
    Ok(false)
}

pub fn compile_lines(
    code: &str,
    debug_print: bool,
) -> Result<(Vec<u8>, Vec<CompiledBytes>), Vec<CompilationError>> {
    let mut lexer = Lexer::new();
    lexer.tokenize(code);

    let mut compilation_errors = Vec::<CompilationError>::new();

    let mut compiled_bytes_lines_vec = Vec::new();
    let mut compiled_bytes_ref_lines_vec = Vec::new();

    let mut label_addr_map = std::collections::HashMap::<Label, LineNumber>::new();
    let mut label_ref: Vec<(Label, LineNumber, &Vec<Token>)> = Vec::new();

    let is_org_defined = match is_org_defined(&lexer.tokens) {
        Ok(is_org_defined) => is_org_defined,
        Err(err) => {
            compilation_errors.push(err);
            false
        }
    };

    for line in lexer.tokens.iter() {
        match compile(line, is_org_defined) {
            Ok(compiled_line) => {
                let compiled_bytes_line = compiled_line.compiled_bytes;
                let compiled_bytes_ref_line = compiled_line.compiled_bytes_ref;

                for label_str in compiled_line.labels {
                    label_addr_map.insert(
                        UniCase::new(label_str),
                        compiled_bytes_lines_vec.len() as u16,
                    );
                }

                for (label_str, (_, _)) in compiled_line.label_idx_map {
                    let label = UniCase::new(label_str);
                    label_ref.push((
                        label.clone(),
                        compiled_bytes_lines_vec.len() as LineNumber,
                        line,
                    ));
                }

                compiled_bytes_lines_vec.push(compiled_bytes_line);
                compiled_bytes_ref_lines_vec.push(compiled_bytes_ref_line);
            }
            Err(err) => {
                compilation_errors.push(err);
            }
        }
    }

    // check if all flags are defined
    let mut label_errors = false;
    for (label, line_number, line) in &label_ref {
        // find the postion of the label
        let idx = line
            .iter()
            .position(|token| token.token_type == Assembly8086Tokens::Character(label.to_string()))
            .unwrap_or(0);
        if !label_addr_map.contains_key(label) {
            label_errors = true;
            compilation_errors.push(CompilationError::new(
                *line_number as u32,
                line[idx].column_number,
                line[idx].token_length,
                &format!("The label \"{}\" is Undefined, Please define it.", label),
            ));
        }
    }

    if label_errors {
        return Err(compilation_errors);
    }

    match mark_labels(
        &label_ref,
        &mut compiled_bytes_lines_vec,
        &mut compiled_bytes_ref_lines_vec,
        &label_addr_map,
        is_org_defined,
        0,
    ) {
        Ok(_) => (),
        Err(err) => {
            compilation_errors.push(err);
        }
    };

    // convert compiled bytes and ref to single vec
    let compiled_bytes = compiled_bytes_lines_vec
        .iter()
        .flatten()
        .cloned()
        .collect::<Vec<u8>>();

    let mut compiled_bytes_ref = Vec::new();
    for i in compiled_bytes_ref_lines_vec {
        for j in i {
            compiled_bytes_ref.push(j);
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
