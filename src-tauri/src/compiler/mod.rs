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

pub(self) mod suggestions;
pub(self) mod suggestions_utils;

use compilation_error::CompilationError;
use lexer::Lexer;
use tokens::instructions::Instructions;

use self::{
    compilation_utils::{
        check_is_label,
        error_if_hasnt_consumed_all_ins, // find_data_line_num,
        get_full_line_error_starting_from_i,
        is_org_defined,
    },
    parsers::{
        add::parse_add, dec::parse_dec, inc::parse_inc, jmp::parse_jmp, mov::parse_mov,
        mul::parse_mul, sub::parse_sub, var::parse_var_declaration,
    },
    tokenized_line::TokenizedLine,
    tokens::{assembler_directives::AssemblerDirectives, Assembly8086Tokens, Token},
    types_structs::{
        CompiledBytesReference, CompiledLine, IsLabelBeforeRef, Label, LabelAddressMap, LineNumber,
        VariableAddressDefinitionMap, VariableAddressMap, VariableType,
    },
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

fn compile(
    lexed_strings: &Vec<Token>,
    is_org_defined: bool,
    compiled_line_label_ref: Option<&CompiledLineLabelRef>,
    variable_address_map: Option<&VariableAddressMap>,
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
    // let instruction = get_instruction(&lexed_str_without_spaces, i)?;
    let tokenized_line = TokenizedLine::new(&lexed_str_without_spaces, len_lexed_strings);

    let token = &lexed_str_without_spaces[i];
    if let Assembly8086Tokens::AssemblerDirectives(dir) = &token.token_type {
        match dir {
            AssemblerDirectives::Org => {}
            AssemblerDirectives::Data => {
                if is_org_defined {
                    let jmp_ins = get_jmp_code_compiled_line(token);
                    let jmp_ins: Vec<&Token> = jmp_ins.iter().collect();
                    let mut temp_line = CompiledLine::new();
                    let offset_bytes_from_line_and_is_label_before_ref =
                        unwrap_and_find_offset(&compiled_line_label_ref);

                    let _ = parse_jmp(
                        &TokenizedLine::new(&jmp_ins, 0),
                        0,
                        &mut temp_line.compiled_bytes,
                        &mut temp_line.compiled_bytes_ref,
                        Some(&VariableAddressMap::new()),
                        &mut temp_line.label_idx_map,
                        offset_bytes_from_line_and_is_label_before_ref,
                    )?;
                    let high_token = tokenized_line
                        .get(i, "Unexpected error, Please report this".to_string(), None)?;
                    temp_line
                        .label_idx_map
                        .insert("code".to_string(), (high_token.clone(), i));
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

    if i >= lexed_str_without_spaces.len() {
        return Ok(compiled_line);
    }

    let token = &lexed_str_without_spaces[i];
    let compiled_bytes = &mut compiled_line.compiled_bytes;
    let compiled_bytes_ref = &mut compiled_line.compiled_bytes_ref;
    let variable_ref_map = &mut compiled_line.variable_reference_map;

    let offset_bytes_from_line_and_is_label_before_ref =
        unwrap_and_find_offset(&compiled_line_label_ref);

    match &token.token_type {
        Assembly8086Tokens::Character(_) => {
            i = parse_var_declaration(
                &tokenized_line,
                i,
                compiled_bytes,
                compiled_bytes_ref,
                &mut compiled_line.variable_abs_address_map,
            )?;
            get_full_line_error_starting_from_i(&lexed_str_without_spaces, i, "VAR")?;
            Ok(compiled_line)
        }
        Assembly8086Tokens::Instruction(ins) => match ins {
            Instructions::Mov => {
                i = parse_mov(
                    &tokenized_line,
                    i,
                    compiled_bytes,
                    compiled_bytes_ref,
                    variable_ref_map,
                    variable_address_map,
                )?;
                error_if_hasnt_consumed_all_ins(&lexed_str_without_spaces, i, "MOV", 2)?;
                Ok(compiled_line)
            }

            Instructions::Add => {
                i = parse_add(
                    &tokenized_line,
                    i,
                    compiled_bytes,
                    compiled_bytes_ref,
                    variable_ref_map,
                    variable_address_map,
                )?;

                error_if_hasnt_consumed_all_ins(&lexed_str_without_spaces, i, "ADD", 2)?;
                Ok(compiled_line)
            }

            Instructions::Inc => {
                i = parse_inc(&tokenized_line, i, compiled_bytes, compiled_bytes_ref, variable_address_map)?;
                error_if_hasnt_consumed_all_ins(&lexed_str_without_spaces, i, "INC", 1)?;
                Ok(compiled_line)
            }

            Instructions::Dec => {
                i = parse_dec(
                    &tokenized_line,
                    i,
                    compiled_bytes,
                    compiled_bytes_ref,
                    variable_address_map,
                )?;

                error_if_hasnt_consumed_all_ins(&lexed_str_without_spaces, i, "DEC", 1)?;
                Ok(compiled_line)
            }

            Instructions::Sub => {
                i = parse_sub(
                    &tokenized_line,
                    i,
                    compiled_bytes,
                    compiled_bytes_ref,
                    variable_ref_map,
                    variable_address_map,
                )?;

                error_if_hasnt_consumed_all_ins(&lexed_str_without_spaces, i, "ADD", 2)?;
                Ok(compiled_line)
            }

            Instructions::Mul => {
                i = parse_mul(&tokenized_line, i, compiled_bytes, compiled_bytes_ref, variable_address_map)?;
                error_if_hasnt_consumed_all_ins(&lexed_str_without_spaces, i, "MUL", 1)?;
                Ok(compiled_line)
            }

            Instructions::Jmp => {
                let i = parse_jmp(
                    &tokenized_line,
                    i,
                    compiled_bytes,
                    compiled_bytes_ref,
                    variable_address_map,
                    &mut compiled_line.label_idx_map,
                    offset_bytes_from_line_and_is_label_before_ref,
                )?;
                // compiled_line.extend(_compliled_line);
                error_if_hasnt_consumed_all_ins(&lexed_str_without_spaces, i, "JMP", 1)?;
                Ok(compiled_line)
            }
        },
        Assembly8086Tokens::AssemblerDirectives(_) => {
            Ok(compiled_line)
        }

        _ => Err(CompilationError::new_without_suggestions(
            token.line_number,
            token.column_number,
            token.token_length,
            &format!(
                "Can't compile starting with {:?} as the first token must be an instruction",
                token.token_type
            ),
        )),
    }
}

struct CompiledLineLabelRef<'a> {
    compiled_bytes: &'a [Vec<u8>],
    line_num: LineNumber,
    label: &'a str,
    label_addr_map: &'a HashMap<UniCase<String>, LineNumber>,
    // is_org_defined: bool,
}

fn unwrap_and_find_offset(
    compiled_line_label_ref: &Option<&CompiledLineLabelRef>,
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

fn calc_offset(
    compiled_bytes: &[Vec<u8>],
    label_ref: LineNumber,
    label_addr: LineNumber,
) -> (u16, IsLabelBeforeRef) {
    let mut offset = 0;
    if label_addr < label_ref {
        for bytes in compiled_bytes.iter().take(label_ref).skip(label_addr) {
            offset += bytes.len();
        }
    } else {
        // i.e label is refernced before it is defined
        for bytes in compiled_bytes.iter().take(label_addr).skip(label_ref + 1) {
            offset += bytes.len();
        }
    }

    let is_label_before_ref = label_ref > label_addr;
    (offset as u16, is_label_before_ref)
}

fn mark_labels(
    label_ref: &LabelRefrenceList,

    tokenized_line: &Vec<Vec<Token>>,

    compiled_bytes: &mut [Vec<u8>],
    compiled_bytes_ref: &mut [Vec<CompiledBytesReference>],

    label_addr_map: &LabelAddressMap,
    is_org_defined: bool,
    idx: usize,
) -> Result<bool, CompilationError> {
    if idx >= label_ref.len() {
        return Ok(true);
    }
    let (label, _, line_number, tokenized_line_idx) = &label_ref[idx];
    let line_number = *line_number;
    for _ in 0..(label_ref.len() - idx) {
        let compiled_tokens = compile(
            &tokenized_line[*tokenized_line_idx],
            is_org_defined,
            Some(&CompiledLineLabelRef {
                compiled_bytes,
                line_num: line_number,
                label,
                label_addr_map,
                // is_org_defined,
            }),
            None,
        )?;

        compiled_bytes[line_number] = compiled_tokens.compiled_bytes;
        compiled_bytes_ref[line_number] = compiled_tokens.compiled_bytes_ref;

        let prev_compiled_bytes_len = compiled_bytes[line_number].len();
        let curr_compiled_bytes_len = compiled_bytes[line_number].len();
        if prev_compiled_bytes_len != curr_compiled_bytes_len {
            return Ok(false);
        }

        if mark_labels(
            label_ref,
            tokenized_line,
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

fn mark_variables(
    compiled_bytes: &mut [Vec<u8>],
    compiled_bytes_ref: &mut [Vec<CompiledBytesReference>],

    tokenized_lines: &Vec<Vec<Token>>,

    var_ref: &VariableReferenceList,
    var_addr_def_map: &VariableAddressDefinitionMap,

    is_org_defined: bool,
) -> Result<(), CompilationError> {
    // calc offset addr for each var
    let mut var_addr_map = VariableAddressMap::new();
    for (var_label, (var_type, label_definition_line_number)) in var_addr_def_map {
        let (offset, _) = calc_offset(compiled_bytes, 0, *label_definition_line_number);
        let org_offset = if is_org_defined { 0x100 } else { 0x00 };
        var_addr_map.insert(var_label.clone(), (var_type.clone(), offset + org_offset));
    }

    // mark the variables
    for (_, _, line_number, tokenized_line_index) in var_ref {
        let tokenized_line = &tokenized_lines[*tokenized_line_index];
        let line_number = *line_number;
        let compiled_tokens = compile(tokenized_line, is_org_defined, None, Some(&var_addr_map))?;

        compiled_bytes[line_number] = compiled_tokens.compiled_bytes;
        compiled_bytes_ref[line_number] = compiled_tokens.compiled_bytes_ref;
    }

    Ok(())
}

fn get_err_if_already_defined_label<T>(
    label_key: UniCase<String>,
    line: &[Token],
    label_addr_map: &mut HashMap<Label, T>,
    already_defined_line_number: LineNumber,
) -> Option<CompilationError> {
    let idx = line
        .iter()
        .position(|_token| _token.token_type == Assembly8086Tokens::Character(label_key.clone()))
        .unwrap();
    if label_addr_map.contains_key(&label_key) {
        return Some(CompilationError::new_without_suggestions(
            line[idx].line_number,
            line[idx].column_number,
            line[idx].token_length,
            &format!(
                "The label \"{}\" is already defined in line {}, Please use a different name.",
                label_key, already_defined_line_number
            ),
        ));
    }
    None
}

pub fn compile_lines(
    code: &str,
    debug_print: bool,
) -> Result<(Vec<u8>, Vec<CompiledBytesReference>, bool), Vec<CompilationError>> {
    let mut lexer = Lexer::new();
    lexer.tokenize(code);

    let mut compilation_errors = Vec::new();
    let mut compiled_bytes_lines_vec = Vec::new();
    let mut compiled_bytes_ref_lines_vec = Vec::new();

    let mut label_addr_map = LabelAddressMap::new();
    let mut label_ref = LabelRefrenceList::new();

    let mut var_addr_def_map = VariableAddressDefinitionMap::new();
    let mut var_ref = VariableReferenceList::new();

    match compile_lines_perform_var_label_substiution(
        &mut lexer,
        &mut compilation_errors,
        &mut compiled_bytes_lines_vec,
        &mut compiled_bytes_ref_lines_vec,
        &mut label_addr_map,
        &mut label_ref,
        &mut var_addr_def_map,
        &mut var_ref,
    ) {
        Some(is_org_defined) => {
            let compiled_bytes = compiled_bytes_lines_vec.into_iter().flatten().collect();
            let compiled_bytes_ref = compiled_bytes_ref_lines_vec
                .into_iter()
                .flatten()
                .collect::<Vec<CompiledBytesReference>>();

            if debug_print {
                lexer.print_with_compiled_tokens(&compiled_bytes_ref);
            }

            Ok((compiled_bytes, compiled_bytes_ref, is_org_defined))
        }
        None => {
            if !compilation_errors.is_empty() {
                Err(compilation_errors)
            } else {
                Ok((Vec::new(), Vec::new(), false))
            }
        }
    }
}

type LabelRefrenceList = Vec<(Label, Token, LineNumber, LineNumber)>;
type VariableReferenceList<'a> = Vec<(Label, VariableType, LineNumber, LineNumber )>;

fn compile_lines_perform_var_label_substiution(
    lexer: &mut Lexer,
    compilation_errors: &mut Vec<CompilationError>,
    compiled_bytes_lines_vec: &mut Vec<Vec<u8>>,
    compiled_bytes_ref_lines_vec: &mut Vec<Vec<CompiledBytesReference>>,
    label_addr_map: &mut LabelAddressMap,
    label_ref: &mut LabelRefrenceList,
    var_addr_def_map: &mut VariableAddressDefinitionMap,
    var_ref: &mut VariableReferenceList,
    // ) -> Result<(Vec<u8>, Vec<CompiledBytesReference>, bool), Vec<CompilationError>> {
) -> Option<bool> {
    let is_org_defined = match is_org_defined(&lexer.tokens) {
        Ok(is_org_defined) => is_org_defined,
        Err(err) => {
            compilation_errors.push(err);
            false
        }
    };

    for (i, line) in lexer.tokens.iter().enumerate() {
        match compile(line, is_org_defined, None, None) {
            Ok(compiled_line) => {
                let compiled_bytes_line = compiled_line.compiled_bytes;
                let compiled_bytes_ref_line = compiled_line.compiled_bytes_ref;

                // Pushing all the labels into a map after checking if they are already defined
                for label_str in compiled_line.labels {
                    let label_key = UniCase::new(label_str.to_string().clone());
                    let already_defined_line_number = *label_addr_map.get(&label_key).unwrap_or(&0);
                    // check if the label is already defined
                    if let Some(err) = get_err_if_already_defined_label(
                        label_key.clone(),
                        line,
                        label_addr_map,
                        already_defined_line_number,
                    ) {
                        compilation_errors.push(err);
                        continue;
                    }
                    // push the label into the label_addr_map if it's not already defined
                    label_addr_map.insert(label_key, compiled_bytes_lines_vec.len() as LineNumber);
                }

                // Pushing all the variables into a map after checking if they are already defined
                for (label_str, (label_type, _)) in compiled_line.variable_abs_address_map {
                    let label_key = UniCase::new(label_str.to_string().clone());
                    // get the line number of the already defined label if it exists
                    let already_defined_line_number = var_addr_def_map
                        .get(&label_key)
                        .unwrap_or(&(VariableType::Byte, 0))
                        .1;
                    // check if the label is already defined
                    if let Some(err) = get_err_if_already_defined_label(
                        label_key.clone(),
                        line,
                        var_addr_def_map,
                        already_defined_line_number,
                    ) {
                        compilation_errors.push(err);
                        continue;
                    }
                    // push the label into the label_addr_map if it's not already defined
                    var_addr_def_map.insert(
                        label_str,
                        (label_type, compiled_bytes_lines_vec.len() as LineNumber),
                    );
                }

                // Pushing all the labels that reference a particular label
                for (label_str, (token, _)) in compiled_line.label_idx_map {
                    let label = UniCase::new(label_str);
                    label_ref.push((
                        label.clone(),
                        token,
                        compiled_bytes_lines_vec.len() as LineNumber,
                        i
                    ));
                }

                // Pushing all the variable that reference a particular variable
                for (var_str, (label_type, _)) in compiled_line.variable_reference_map {
                    var_ref.push((
                        var_str.clone(),
                        label_type,
                        compiled_bytes_lines_vec.len() as LineNumber,
                        i,
                    ));
                }

                // pushing the compiled bytes and ref into a vec
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
    for (label, token, line_number, tokenized_line_number) in &mut *label_ref {
        let line = &lexer.tokens[*tokenized_line_number];
        let idx = line
            .iter()
            .position(|_token| {
                _token.line_number == token.line_number
                    && _token.column_number == token.column_number
            })
            .unwrap();
        if !label_addr_map.contains_key(label) {
            label_errors = true;
            compilation_errors.push(CompilationError::new_without_suggestions(
                *line_number as u32,
                line[idx].column_number,
                line[idx].token_length,
                &format!("The label \"{}\" is Undefined, Please define it.", label),
            ));
        }
    }

    // check if all the variables are defined
    let mut var_errors = false;
    for (_i, (var, used_as_type, line_number, tokenized_line_number)) in var_ref.iter().enumerate() {
        let line = &lexer.tokens[*tokenized_line_number];
        let idx = line
            .iter()
            .position(|_token| _token.token_type == Assembly8086Tokens::Character(var.clone()))
            .unwrap();
        if !var_addr_def_map.contains_key(var) {
            var_errors = true;
            compilation_errors.push(CompilationError::new_without_suggestions(
                *line_number as u32,
                line[idx].column_number,
                line[idx].token_length,
                &format!("The variable \"{}\" is Undefined, Please define it.", var),
            ));
        } else if *used_as_type == VariableType::Word
            && var_addr_def_map.get(var).unwrap().0 == VariableType::Byte
        {
            var_errors = true;
            compilation_errors.push(CompilationError::new_without_suggestions(
                *line_number as u32,
                line[idx].column_number,
                line[idx].token_length,
                &format!(
                    "The variable \"{}\" is defined as {:?}, but used as {:?}.",
                    var,
                    var_addr_def_map.get(var).unwrap().0,
                    used_as_type
                ),
            ));
        }
    }

    if label_errors || var_errors {
        return None;
    }

    match mark_labels(
        label_ref,
        &lexer.tokens,
        compiled_bytes_lines_vec,
        compiled_bytes_ref_lines_vec,
        label_addr_map,
        is_org_defined,
        0,
    ) {
        Ok(_) => (),
        Err(err) => {
            compilation_errors.push(err);
        }
    };

    match mark_variables(
        compiled_bytes_lines_vec,
        compiled_bytes_ref_lines_vec,
        &lexer.tokens,
        var_ref,
        var_addr_def_map,
        is_org_defined,
    ) {
        Ok(_) => (),
        Err(err) => {
            compilation_errors.push(err);
        }
    };

    if !compilation_errors.is_empty() {
        return None;
    }

    Some(is_org_defined)
}

pub fn compile_str(
    code: &str,
    debug_print: bool,
) -> Result<(Vec<u8>, Vec<CompiledBytesReference>), Vec<CompilationError>> {
    let (compiled_bytes, compiled_bytes_ref, _) = compile_lines(code, debug_print)?;
    Ok((compiled_bytes, compiled_bytes_ref))
}
