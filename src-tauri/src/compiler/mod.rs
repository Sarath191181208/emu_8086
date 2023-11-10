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

pub mod suggestions;
pub mod suggestions_utils;

use compilation_error::CompilationError;
use lexer::Lexer;
use tokens::instructions::Instructions;

use crate::{
    compiler::{
        parsers::utils::push_instruction, types_structs::ProcDefitionType,
        utils::get_label_token_from_line,
    },
    convert_and_push_instructions,
};

use self::{
    compilation_utils::{
        check_is_label,
        error_if_hasnt_consumed_all_ins, // find_data_line_num,
        get_full_line_error_starting_from_i,
        is_org_defined,
    },
    parsers::{
        adc::parse_adc,
        add::parse_add,
        and::parse_and,
        call::parse_call,
        cmp::parse_cmp,
        dec::parse_dec,
        in_ins::parse_in,
        inc::parse_inc,
        jmp::parse_jmp,
        lea::parse_lea,
        les::parse_les,
        loop_ins::parse_loop,
        mov::parse_mov,
        mul::parse_mul,
        or::parse_or,
        out_ins::parse_out,
        pattern_extractors::{
            offset_label_pattern::parse_label_pattern_full, parse_two_arguments_line,
        },
        pop::parse_pop,
        push::parse_push,
        sbb::parse_sbb,
        sub::parse_sub,
        test_ins::parse_test,
        utils::iterate_with_seperator,
        var::parse_var_declaration,
        xchg::parse_xchg,
        xor::parse_xor,
    },
    tokenized_line::TokenizedLine,
    tokens::{
        assembler_directives::AssemblerDirectives, data::DefineData, Assembly8086Tokens, Token,
    },
    types_structs::{
        ArrayIndex, CompiledBytesIndexedLineNumber, CompiledBytesReference, CompiledLine,
        IsLabelBeforeRef, Label, LabelAddressMap, LabelRefrenceList, LineNumber,
        MacroBoundsDefintionMap, MacroReferenceList, ProcDefinitionLineNumberMap,
        ProcReferenceList, VariableAddressDefinitionMap, VariableAddressMap, VariableReferenceList,
        VariableType,
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
    line_number: usize,
    lexed_strings: &[Token],
    is_org_defined: bool,
    compiled_line_offset_maps: Option<&CompiledLineLabelRef>,
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

    let tokenized_line = TokenizedLine::new(&lexed_str_without_spaces, len_lexed_strings);

    let token = &lexed_str_without_spaces[i];
    if let Assembly8086Tokens::AssemblerDirectives(dir) = &token.token_type {
        match dir {
            AssemblerDirectives::Org | AssemblerDirectives::Macro | AssemblerDirectives::EndM => {}
            AssemblerDirectives::Data => {
                if is_org_defined {
                    let jmp_ins = get_jmp_code_compiled_line(token);
                    let jmp_ins: Vec<&Token> = jmp_ins.iter().collect();
                    let mut temp_line = CompiledLine::new();

                    let _ = parse_jmp(
                        &TokenizedLine::new(&jmp_ins, 0),
                        0,
                        line_number,
                        &mut temp_line.compiled_bytes,
                        &mut temp_line.compiled_bytes_ref,
                        Some(&VariableAddressMap::new()),
                        &mut temp_line.label_idx_map,
                        compiled_line_offset_maps,
                    )?;
                    let high_token = tokenized_line.get(
                        i,
                        "Unexpected error, Please report this".to_string(),
                        None,
                    )?;
                    temp_line
                        .label_idx_map
                        .insert("code".to_string(), (high_token.clone(), i, false));
                    compiled_line.extend(temp_line);
                    i += 1;
                }
            }
            AssemblerDirectives::Code => {
                // push code into compiled_line.labels don't change the other values already in compiled_line
                compiled_line.labels.push("code".to_string());
            }
            AssemblerDirectives::Offset
            | AssemblerDirectives::AsWord
            | AssemblerDirectives::AsByte => {}
        }
    }

    if i >= lexed_str_without_spaces.len() {
        return Ok(compiled_line);
    }

    let token = &lexed_str_without_spaces[i];
    let compiled_bytes = &mut compiled_line.compiled_bytes;
    let compiled_bytes_ref = &mut compiled_line.compiled_bytes_ref;
    let variable_ref_map = &mut compiled_line.variable_reference_map;

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
                let addressing_mode = parse_two_arguments_line(
                    &tokenized_line,
                    i,
                    is_org_defined,
                    "MOV",
                    &mut compiled_line.label_idx_map,
                    variable_ref_map,
                    variable_address_map.unwrap_or(&HashMap::default()),
                    compiled_line_offset_maps,
                )?;
                i = parse_mov(
                    &tokenized_line,
                    i,
                    compiled_bytes,
                    compiled_bytes_ref,
                    addressing_mode,
                )?;
                error_if_hasnt_consumed_all_ins(&lexed_str_without_spaces, i, "MOV", 2)?;
                Ok(compiled_line)
            }

            Instructions::Add => {
                let addressing_mode = parse_two_arguments_line(
                    &tokenized_line,
                    i,
                    is_org_defined,
                    "ADD",
                    &mut compiled_line.label_idx_map,
                    variable_ref_map,
                    variable_address_map.unwrap_or(&VariableAddressMap::default()),
                    compiled_line_offset_maps,
                )?;
                i = parse_add(
                    &tokenized_line,
                    i,
                    compiled_bytes,
                    compiled_bytes_ref,
                    addressing_mode,
                )?;

                error_if_hasnt_consumed_all_ins(&lexed_str_without_spaces, i, "ADD", 2)?;
                Ok(compiled_line)
            }

            Instructions::Adc => {
                let addressing_mode = parse_two_arguments_line(
                    &tokenized_line,
                    i,
                    is_org_defined,
                    "ADC",
                    &mut compiled_line.label_idx_map,
                    variable_ref_map,
                    variable_address_map.unwrap_or(&VariableAddressMap::default()),
                    compiled_line_offset_maps,
                )?;
                i = parse_adc(
                    &tokenized_line,
                    i,
                    compiled_bytes,
                    compiled_bytes_ref,
                    addressing_mode,
                )?;

                error_if_hasnt_consumed_all_ins(&lexed_str_without_spaces, i, "ADD", 2)?;
                Ok(compiled_line)
            }

            Instructions::Cmp => {
                let addressing_mode = parse_two_arguments_line(
                    &tokenized_line,
                    i,
                    is_org_defined,
                    "CMP",
                    &mut compiled_line.label_idx_map,
                    variable_ref_map,
                    variable_address_map.unwrap_or(&VariableAddressMap::default()),
                    compiled_line_offset_maps,
                )?;
                i = parse_cmp(
                    &tokenized_line,
                    i,
                    compiled_bytes,
                    compiled_bytes_ref,
                    addressing_mode,
                )?;

                error_if_hasnt_consumed_all_ins(&lexed_str_without_spaces, i, "ADD", 2)?;
                Ok(compiled_line)
            }

            Instructions::Sbb => {
                let addressing_mode = parse_two_arguments_line(
                    &tokenized_line,
                    i,
                    is_org_defined,
                    "SBB",
                    &mut compiled_line.label_idx_map,
                    variable_ref_map,
                    variable_address_map.unwrap_or(&VariableAddressMap::default()),
                    compiled_line_offset_maps,
                )?;
                i = parse_sbb(
                    &tokenized_line,
                    i,
                    compiled_bytes,
                    compiled_bytes_ref,
                    addressing_mode,
                )?;

                error_if_hasnt_consumed_all_ins(&lexed_str_without_spaces, i, "ADD", 2)?;
                Ok(compiled_line)
            }

            Instructions::Sub => {
                let addressing_mode = parse_two_arguments_line(
                    &tokenized_line,
                    i,
                    is_org_defined,
                    "SUB",
                    &mut compiled_line.label_idx_map,
                    variable_ref_map,
                    variable_address_map.unwrap_or(&VariableAddressMap::default()),
                    compiled_line_offset_maps,
                )?;
                i = parse_sub(
                    &tokenized_line,
                    i,
                    compiled_bytes,
                    compiled_bytes_ref,
                    addressing_mode,
                )?;

                error_if_hasnt_consumed_all_ins(&lexed_str_without_spaces, i, "ADD", 2)?;
                Ok(compiled_line)
            }

            Instructions::Test => {
                let addressing_mode = parse_two_arguments_line(
                    &tokenized_line,
                    i,
                    is_org_defined,
                    "TEST",
                    &mut compiled_line.label_idx_map,
                    variable_ref_map,
                    variable_address_map.unwrap_or(&VariableAddressMap::default()),
                    compiled_line_offset_maps,
                )?;
                i = parse_test(
                    &tokenized_line,
                    i,
                    compiled_bytes,
                    compiled_bytes_ref,
                    addressing_mode,
                )?;

                error_if_hasnt_consumed_all_ins(&lexed_str_without_spaces, i, "ADD", 2)?;
                Ok(compiled_line)
            }

            Instructions::And => {
                let addressing_mode = parse_two_arguments_line(
                    &tokenized_line,
                    i,
                    is_org_defined,
                    "AND",
                    &mut compiled_line.label_idx_map,
                    variable_ref_map,
                    variable_address_map.unwrap_or(&VariableAddressMap::default()),
                    compiled_line_offset_maps,
                )?;
                i = parse_and(
                    &tokenized_line,
                    i,
                    compiled_bytes,
                    compiled_bytes_ref,
                    addressing_mode,
                )?;

                error_if_hasnt_consumed_all_ins(&lexed_str_without_spaces, i, "ADD", 2)?;
                Ok(compiled_line)
            }

            Instructions::Or => {
                let addressing_mode = parse_two_arguments_line(
                    &tokenized_line,
                    i,
                    is_org_defined,
                    "OR",
                    &mut compiled_line.label_idx_map,
                    variable_ref_map,
                    variable_address_map.unwrap_or(&VariableAddressMap::default()),
                    compiled_line_offset_maps,
                )?;
                i = parse_or(
                    &tokenized_line,
                    i,
                    compiled_bytes,
                    compiled_bytes_ref,
                    addressing_mode,
                )?;

                error_if_hasnt_consumed_all_ins(&lexed_str_without_spaces, i, "ADD", 2)?;
                Ok(compiled_line)
            }

            Instructions::Xchg => {
                let addressing_mode = parse_two_arguments_line(
                    &tokenized_line,
                    i,
                    is_org_defined,
                    "XCHG",
                    &mut compiled_line.label_idx_map,
                    variable_ref_map,
                    variable_address_map.unwrap_or(&VariableAddressMap::default()),
                    compiled_line_offset_maps,
                )?;
                i = parse_xchg(
                    &tokenized_line,
                    i,
                    compiled_bytes,
                    compiled_bytes_ref,
                    addressing_mode,
                )?;

                error_if_hasnt_consumed_all_ins(&lexed_str_without_spaces, i, "ADD", 2)?;
                Ok(compiled_line)
            }

            Instructions::Xor => {
                let addressing_mode = parse_two_arguments_line(
                    &tokenized_line,
                    i,
                    is_org_defined,
                    "XOR",
                    &mut compiled_line.label_idx_map,
                    variable_ref_map,
                    variable_address_map.unwrap_or(&VariableAddressMap::default()),
                    compiled_line_offset_maps,
                )?;
                i = parse_xor(
                    &tokenized_line,
                    i,
                    compiled_bytes,
                    compiled_bytes_ref,
                    addressing_mode,
                )?;

                error_if_hasnt_consumed_all_ins(&lexed_str_without_spaces, i, "ADD", 2)?;
                Ok(compiled_line)
            }

            Instructions::Inc => {
                i = parse_inc(
                    &tokenized_line,
                    i,
                    compiled_bytes,
                    compiled_bytes_ref,
                    variable_ref_map,
                    variable_address_map,
                )?;
                error_if_hasnt_consumed_all_ins(&lexed_str_without_spaces, i, "INC", 1)?;
                Ok(compiled_line)
            }

            Instructions::Dec => {
                i = parse_dec(
                    &tokenized_line,
                    i,
                    compiled_bytes,
                    compiled_bytes_ref,
                    variable_ref_map,
                    variable_address_map,
                )?;

                error_if_hasnt_consumed_all_ins(&lexed_str_without_spaces, i, "DEC", 1)?;
                Ok(compiled_line)
            }

            Instructions::Mul => {
                i = parse_mul(
                    &tokenized_line,
                    i,
                    compiled_bytes,
                    compiled_bytes_ref,
                    variable_ref_map,
                    variable_address_map,
                )?;
                error_if_hasnt_consumed_all_ins(&lexed_str_without_spaces, i, "MUL", 1)?;
                Ok(compiled_line)
            }

            Instructions::Loop => {
                let i = parse_loop(
                    &tokenized_line,
                    i,
                    line_number,
                    compiled_bytes,
                    compiled_bytes_ref,
                    &mut compiled_line.label_idx_map,
                    compiled_line_offset_maps,
                )?;
                // compiled_line.extend(_compliled_line);
                error_if_hasnt_consumed_all_ins(&lexed_str_without_spaces, i, "LOOP", 1)?;
                Ok(compiled_line)
            }

            Instructions::Lea => {
                let i = parse_lea(
                    i,
                    &tokenized_line,
                    is_org_defined,
                    &mut compiled_line.label_idx_map,
                    variable_ref_map,
                    variable_address_map.unwrap_or(&VariableAddressMap::default()),
                    compiled_line_offset_maps,
                    compiled_bytes,
                    compiled_bytes_ref,
                )?;

                error_if_hasnt_consumed_all_ins(&lexed_str_without_spaces, i, "LEA", 2)?;
                Ok(compiled_line)
            }

            Instructions::Les => {
                let i = parse_les(
                    i,
                    &tokenized_line,
                    is_org_defined,
                    &mut compiled_line.label_idx_map,
                    variable_ref_map,
                    variable_address_map.unwrap_or(&VariableAddressMap::default()),
                    compiled_line_offset_maps,
                    compiled_bytes,
                    compiled_bytes_ref,
                )?;

                error_if_hasnt_consumed_all_ins(&lexed_str_without_spaces, i, "LEA", 2)?;
                Ok(compiled_line)
            }

            Instructions::Ja => {
                i = parse_label_pattern_full(
                    "JA",
                    vec![0x77],
                    vec![0x76, 0x03, 0xE9], // JNBE ins
                    &tokenized_line,
                    i,
                    line_number,
                    compiled_bytes,
                    compiled_bytes_ref,
                    &mut compiled_line.label_idx_map,
                    compiled_line_offset_maps,
                )?;
                // compiled_line.extend(_compliled_line);
                error_if_hasnt_consumed_all_ins(&lexed_str_without_spaces, i, "JA", 1)?;
                Ok(compiled_line)
            }

            Instructions::Jae => {
                i = parse_label_pattern_full(
                    "JAE",
                    vec![0x73],
                    vec![0x72, 0x03, 0xE9], // JB ins
                    &tokenized_line,
                    i,
                    line_number,
                    compiled_bytes,
                    compiled_bytes_ref,
                    &mut compiled_line.label_idx_map,
                    compiled_line_offset_maps,
                )?;
                // compiled_line.extend(_compliled_line);
                error_if_hasnt_consumed_all_ins(&lexed_str_without_spaces, i, "JAE", 1)?;
                Ok(compiled_line)
            }

            Instructions::Jb => {
                i = parse_label_pattern_full(
                    "JB",
                    vec![0x72],
                    vec![0x73, 0x03, 0xE9], // JB ins
                    &tokenized_line,
                    i,
                    line_number,
                    compiled_bytes,
                    compiled_bytes_ref,
                    &mut compiled_line.label_idx_map,
                    compiled_line_offset_maps,
                )?;
                // compiled_line.extend(_compliled_line);
                error_if_hasnt_consumed_all_ins(&lexed_str_without_spaces, i, "JB", 1)?;
                Ok(compiled_line)
            }

            Instructions::Jbe => {
                i = parse_label_pattern_full(
                    "JBE",
                    vec![0x76],
                    vec![0x77, 0x03, 0xE9], // JB ins
                    &tokenized_line,
                    i,
                    line_number,
                    compiled_bytes,
                    compiled_bytes_ref,
                    &mut compiled_line.label_idx_map,
                    compiled_line_offset_maps,
                )?;
                // compiled_line.extend(_compliled_line);
                error_if_hasnt_consumed_all_ins(&lexed_str_without_spaces, i, "JBE", 1)?;
                Ok(compiled_line)
            }

            Instructions::Jc => {
                i = parse_label_pattern_full(
                    "JC",
                    vec![0x72],
                    vec![0x73, 0x03, 0xE9], // JB ins
                    &tokenized_line,
                    i,
                    line_number,
                    compiled_bytes,
                    compiled_bytes_ref,
                    &mut compiled_line.label_idx_map,
                    compiled_line_offset_maps,
                )?;
                // compiled_line.extend(_compliled_line);
                error_if_hasnt_consumed_all_ins(&lexed_str_without_spaces, i, "JC", 1)?;
                Ok(compiled_line)
            }

            Instructions::Jcxz => {
                i = parse_label_pattern_full(
                    "JCXZ",
                    vec![0xE3],
                    vec![0x0B, 0xC9, 0x75, 0x03, 0xE9], // JB ins
                    &tokenized_line,
                    i,
                    line_number,
                    compiled_bytes,
                    compiled_bytes_ref,
                    &mut compiled_line.label_idx_map,
                    compiled_line_offset_maps,
                )?;
                // compiled_line.extend(_compliled_line);
                error_if_hasnt_consumed_all_ins(&lexed_str_without_spaces, i, "JCXZ", 1)?;
                Ok(compiled_line)
            }

            Instructions::Je => {
                i = parse_label_pattern_full(
                    "JE",
                    vec![0x74],
                    vec![0x75, 0x03, 0xE9], // JB ins
                    &tokenized_line,
                    i,
                    line_number,
                    compiled_bytes,
                    compiled_bytes_ref,
                    &mut compiled_line.label_idx_map,
                    compiled_line_offset_maps,
                )?;
                // compiled_line.extend(_compliled_line);
                error_if_hasnt_consumed_all_ins(&lexed_str_without_spaces, i, "JE", 1)?;
                Ok(compiled_line)
            }

            Instructions::Jg => {
                i = parse_label_pattern_full(
                    "JG",
                    vec![0x7F],
                    vec![0x7E, 0x03, 0xE9], // JB ins
                    &tokenized_line,
                    i,
                    line_number,
                    compiled_bytes,
                    compiled_bytes_ref,
                    &mut compiled_line.label_idx_map,
                    compiled_line_offset_maps,
                )?;
                // compiled_line.extend(_compliled_line);
                error_if_hasnt_consumed_all_ins(&lexed_str_without_spaces, i, "JG", 1)?;
                Ok(compiled_line)
            }

            Instructions::Jge => {
                i = parse_label_pattern_full(
                    "JGE",
                    vec![0x7D],
                    vec![0x7C, 0x03, 0xE9], // JB ins
                    &tokenized_line,
                    i,
                    line_number,
                    compiled_bytes,
                    compiled_bytes_ref,
                    &mut compiled_line.label_idx_map,
                    compiled_line_offset_maps,
                )?;
                // compiled_line.extend(_compliled_line);
                error_if_hasnt_consumed_all_ins(&lexed_str_without_spaces, i, "JGE", 1)?;
                Ok(compiled_line)
            }

            Instructions::Jl => {
                i = parse_label_pattern_full(
                    "JL",
                    vec![0x7C],
                    vec![0x7D, 0x03, 0xE9], // JB ins
                    &tokenized_line,
                    i,
                    line_number,
                    compiled_bytes,
                    compiled_bytes_ref,
                    &mut compiled_line.label_idx_map,
                    compiled_line_offset_maps,
                )?;
                // compiled_line.extend(_compliled_line);
                error_if_hasnt_consumed_all_ins(&lexed_str_without_spaces, i, "JL", 1)?;
                Ok(compiled_line)
            }

            Instructions::Jle => {
                i = parse_label_pattern_full(
                    "JLE",
                    vec![0x7E],
                    vec![0x7F, 0x03, 0xE9], // JB ins
                    &tokenized_line,
                    i,
                    line_number,
                    compiled_bytes,
                    compiled_bytes_ref,
                    &mut compiled_line.label_idx_map,
                    compiled_line_offset_maps,
                )?;
                // compiled_line.extend(_compliled_line);
                error_if_hasnt_consumed_all_ins(&lexed_str_without_spaces, i, "JLE", 1)?;
                Ok(compiled_line)
            }

            Instructions::Jna => {
                i = parse_label_pattern_full(
                    "JNA",
                    vec![0x76],
                    vec![0x77, 0x03, 0xE9],
                    &tokenized_line,
                    i,
                    line_number,
                    compiled_bytes,
                    compiled_bytes_ref,
                    &mut compiled_line.label_idx_map,
                    compiled_line_offset_maps,
                )?;
                // compiled_line.extend(_compliled_line);
                error_if_hasnt_consumed_all_ins(&lexed_str_without_spaces, i, "JNA", 1)?;
                Ok(compiled_line)
            }

            Instructions::Jnae => {
                i = parse_label_pattern_full(
                    "JNAE",
                    vec![0x72],
                    vec![0x73, 0x03, 0xE9],
                    &tokenized_line,
                    i,
                    line_number,
                    compiled_bytes,
                    compiled_bytes_ref,
                    &mut compiled_line.label_idx_map,
                    compiled_line_offset_maps,
                )?;
                // compiled_line.extend(_compliled_line);
                error_if_hasnt_consumed_all_ins(&lexed_str_without_spaces, i, "JNAE", 1)?;
                Ok(compiled_line)
            }

            Instructions::Jnb => {
                i = parse_label_pattern_full(
                    "JNB",
                    vec![0x73],
                    vec![0x72, 0x03, 0xE9],
                    &tokenized_line,
                    i,
                    line_number,
                    compiled_bytes,
                    compiled_bytes_ref,
                    &mut compiled_line.label_idx_map,
                    compiled_line_offset_maps,
                )?;
                // compiled_line.extend(_compliled_line);
                error_if_hasnt_consumed_all_ins(&lexed_str_without_spaces, i, "JNB", 1)?;
                Ok(compiled_line)
            }

            Instructions::Jnbe => {
                i = parse_label_pattern_full(
                    "JNBE",
                    vec![0x77],
                    vec![0x76, 0x03, 0xE9],
                    &tokenized_line,
                    i,
                    line_number,
                    compiled_bytes,
                    compiled_bytes_ref,
                    &mut compiled_line.label_idx_map,
                    compiled_line_offset_maps,
                )?;
                // compiled_line.extend(_compliled_line);
                error_if_hasnt_consumed_all_ins(&lexed_str_without_spaces, i, "JNBE", 1)?;
                Ok(compiled_line)
            }

            Instructions::Jnc => {
                i = parse_label_pattern_full(
                    "JNC",
                    vec![0x73],
                    vec![0x72, 0x03, 0xE9],
                    &tokenized_line,
                    i,
                    line_number,
                    compiled_bytes,
                    compiled_bytes_ref,
                    &mut compiled_line.label_idx_map,
                    compiled_line_offset_maps,
                )?;
                // compiled_line.extend(_compliled_line);
                error_if_hasnt_consumed_all_ins(&lexed_str_without_spaces, i, "JNC", 1)?;
                Ok(compiled_line)
            }

            Instructions::Jne => {
                i = parse_label_pattern_full(
                    "JNE",
                    vec![0x75],
                    vec![0x74, 0x03, 0xE9],
                    &tokenized_line,
                    i,
                    line_number,
                    compiled_bytes,
                    compiled_bytes_ref,
                    &mut compiled_line.label_idx_map,
                    compiled_line_offset_maps,
                )?;
                // compiled_line.extend(_compliled_line);
                error_if_hasnt_consumed_all_ins(&lexed_str_without_spaces, i, "JNE", 1)?;
                Ok(compiled_line)
            }

            Instructions::Jng => {
                i = parse_label_pattern_full(
                    "JNG",
                    vec![0x7E],
                    vec![0x7F, 0x03, 0xE9],
                    &tokenized_line,
                    i,
                    line_number,
                    compiled_bytes,
                    compiled_bytes_ref,
                    &mut compiled_line.label_idx_map,
                    compiled_line_offset_maps,
                )?;
                // compiled_line.extend(_compliled_line);
                error_if_hasnt_consumed_all_ins(&lexed_str_without_spaces, i, "JNG", 1)?;
                Ok(compiled_line)
            }

            Instructions::Jnge => {
                i = parse_label_pattern_full(
                    "JNGE",
                    vec![0x7C],
                    vec![0x7D, 0x03, 0xE9],
                    &tokenized_line,
                    i,
                    line_number,
                    compiled_bytes,
                    compiled_bytes_ref,
                    &mut compiled_line.label_idx_map,
                    compiled_line_offset_maps,
                )?;
                // compiled_line.extend(_compliled_line);
                error_if_hasnt_consumed_all_ins(&lexed_str_without_spaces, i, "JNGE", 1)?;
                Ok(compiled_line)
            }

            Instructions::Jnl => {
                i = parse_label_pattern_full(
                    "JNL",
                    vec![0x7D],
                    vec![0x7C, 0x03, 0xE9],
                    &tokenized_line,
                    i,
                    line_number,
                    compiled_bytes,
                    compiled_bytes_ref,
                    &mut compiled_line.label_idx_map,
                    compiled_line_offset_maps,
                )?;
                // compiled_line.extend(_compliled_line);
                error_if_hasnt_consumed_all_ins(&lexed_str_without_spaces, i, "JNL", 1)?;
                Ok(compiled_line)
            }

            Instructions::Jnle => {
                i = parse_label_pattern_full(
                    "JNLE",
                    vec![0x7F],
                    vec![0x7E, 0x03, 0xE9],
                    &tokenized_line,
                    i,
                    line_number,
                    compiled_bytes,
                    compiled_bytes_ref,
                    &mut compiled_line.label_idx_map,
                    compiled_line_offset_maps,
                )?;
                // compiled_line.extend(_compliled_line);
                error_if_hasnt_consumed_all_ins(&lexed_str_without_spaces, i, "JNLE", 1)?;
                Ok(compiled_line)
            }

            Instructions::Jno => {
                i = parse_label_pattern_full(
                    "JNO",
                    vec![0x71],
                    vec![0x70, 0x03, 0xE9],
                    &tokenized_line,
                    i,
                    line_number,
                    compiled_bytes,
                    compiled_bytes_ref,
                    &mut compiled_line.label_idx_map,
                    compiled_line_offset_maps,
                )?;
                // compiled_line.extend(_compliled_line);
                error_if_hasnt_consumed_all_ins(&lexed_str_without_spaces, i, "JNO", 1)?;
                Ok(compiled_line)
            }

            Instructions::Jnp => {
                i = parse_label_pattern_full(
                    "JNP",
                    vec![0x7B],
                    vec![0x7A, 0x03, 0xE9],
                    &tokenized_line,
                    i,
                    line_number,
                    compiled_bytes,
                    compiled_bytes_ref,
                    &mut compiled_line.label_idx_map,
                    compiled_line_offset_maps,
                )?;
                // compiled_line.extend(_compliled_line);
                error_if_hasnt_consumed_all_ins(&lexed_str_without_spaces, i, "JNP", 1)?;
                Ok(compiled_line)
            }

            Instructions::Jns => {
                i = parse_label_pattern_full(
                    "JNS",
                    vec![0x79],
                    vec![0x78, 0x03, 0xE9],
                    &tokenized_line,
                    i,
                    line_number,
                    compiled_bytes,
                    compiled_bytes_ref,
                    &mut compiled_line.label_idx_map,
                    compiled_line_offset_maps,
                )?;
                // compiled_line.extend(_compliled_line);
                error_if_hasnt_consumed_all_ins(&lexed_str_without_spaces, i, "JNS", 1)?;
                Ok(compiled_line)
            }

                        Instructions::Jnz => {
                i = parse_label_pattern_full(
                    "JNZ",
                    vec![0x75],
                    vec![0x74, 0x03, 0xE9],
                    &tokenized_line,
                    i,
                    line_number,
                    compiled_bytes,
                    compiled_bytes_ref,
                    &mut compiled_line.label_idx_map,
                    compiled_line_offset_maps,
                )?;
                // compiled_line.extend(_compliled_line);
                error_if_hasnt_consumed_all_ins(&lexed_str_without_spaces, i, "JNZ", 1)?;
                Ok(compiled_line)
            }

            Instructions::Jmp => {
                let i = parse_jmp(
                    &tokenized_line,
                    i,
                    line_number,
                    compiled_bytes,
                    compiled_bytes_ref,
                    variable_address_map,
                    &mut compiled_line.label_idx_map,
                    compiled_line_offset_maps,
                )?;
                // compiled_line.extend(_compliled_line);
                error_if_hasnt_consumed_all_ins(&lexed_str_without_spaces, i, "JMP", 1)?;
                Ok(compiled_line)
            }
            Instructions::Hlt => {
                convert_and_push_instructions!(
                    compiled_bytes,
                    compiled_bytes_ref,
                    (
                       token => vec![]
                    )
                );
                i += 1;
                error_if_hasnt_consumed_all_ins(&lexed_str_without_spaces, i, "HLT", 0)?;
                Ok(compiled_line)
            }
            Instructions::Ret => {
                convert_and_push_instructions!(
                    compiled_bytes,
                    compiled_bytes_ref,
                    (
                       token => vec![0xC3]
                    )
                );
                i += 1;
                error_if_hasnt_consumed_all_ins(&lexed_str_without_spaces, i, "RET", 0)?;
                Ok(compiled_line)
            }
            Instructions::Proc => {
                let proc_name_token = tokenized_line.get(
                    i + 1,
                    "Expected a label after PROC, Got nothing!".to_string(),
                    None,
                )?;
                match &proc_name_token.token_type {
                    Assembly8086Tokens::Character(c) => compiled_line
                        .proc_definition_map
                        .insert(c.clone(), types_structs::ProcDefitionType::Proc),
                    _ => {
                        return Err(CompilationError::error_with_token(
                            token,
                            &format!(
                                "Expected a label after PROC, Got {} nothing!",
                                proc_name_token.token_type
                            ),
                        ))
                    }
                };
                error_if_hasnt_consumed_all_ins(&lexed_str_without_spaces, i + 1, "PROC", 1)?;
                Ok(compiled_line)
            }
            Instructions::EndP => {
                let proc_name_token = tokenized_line.get(
                    i + 1,
                    "Expected a label after ENDP, Got nothing!".to_string(),
                    None,
                )?;
                match &proc_name_token.token_type {
                    Assembly8086Tokens::Character(c) => compiled_line
                        .proc_definition_map
                        .insert(c.clone(), types_structs::ProcDefitionType::EndP),
                    _ => {
                        return Err(CompilationError::error_with_token(
                            token,
                            &format!(
                                "Expected a label after ENDP, Got {} nothing!",
                                proc_name_token.token_type
                            ),
                        ))
                    }
                };
                error_if_hasnt_consumed_all_ins(&lexed_str_without_spaces, i + 1, "ENDP", 1)?;
                Ok(compiled_line)
            }
            Instructions::Call => {
                let i = parse_call(
                    &tokenized_line,
                    i,
                    line_number,
                    compiled_bytes,
                    compiled_bytes_ref,
                    &mut compiled_line.proc_reference_map,
                    compiled_line_offset_maps,
                )?;
                // compiled_line.extend(_compliled_line);
                error_if_hasnt_consumed_all_ins(&lexed_str_without_spaces, i, "CALL", 1)?;
                Ok(compiled_line)
            }
            Instructions::Int => {
                let val_token = tokenized_line.get(
                    i + 1,
                    "Expected a 8bit value after INT, Got nothing!".to_string(),
                    None,
                )?;
                match &val_token.token_type {
                    Assembly8086Tokens::Number8bit(val) => {
                        convert_and_push_instructions!(
                            compiled_bytes,
                            compiled_bytes_ref,
                            (
                               token => vec![0xCD, *val]
                            )
                        );
                        i += 1;
                    }
                    _ => {
                        return Err(CompilationError::error_with_token(
                            token,
                            &format!(
                                "Expected a 8bit value after INT, Got {} nothing!",
                                val_token.token_type
                            ),
                        ))
                    }
                };
                error_if_hasnt_consumed_all_ins(&lexed_str_without_spaces, i + 1, "INT", 1)?;
                Ok(compiled_line)
            }
            Instructions::Iret => {
                convert_and_push_instructions!(
                    compiled_bytes,
                    compiled_bytes_ref,
                    (
                       token => vec![0xCF]
                    )
                );
                i += 1;
                error_if_hasnt_consumed_all_ins(&lexed_str_without_spaces, i, "IRET", 0)?;
                Ok(compiled_line)
            }
            Instructions::In => {
                i = parse_in(&tokenized_line, i, compiled_bytes, compiled_bytes_ref)?;
                error_if_hasnt_consumed_all_ins(&lexed_str_without_spaces, i, "IN", 2)?;
                Ok(compiled_line)
            }
            Instructions::Out => {
                i = parse_out(&tokenized_line, i, compiled_bytes, compiled_bytes_ref)?;
                error_if_hasnt_consumed_all_ins(&lexed_str_without_spaces, i, "OUT", 2)?;
                Ok(compiled_line)
            }
            Instructions::Push => {
                i = parse_push(
                    &tokenized_line,
                    i,
                    is_org_defined,
                    compiled_bytes,
                    compiled_bytes_ref,
                    variable_ref_map,
                    variable_address_map,
                    &mut compiled_line.label_idx_map,
                    compiled_line_offset_maps,
                )?;
                error_if_hasnt_consumed_all_ins(&lexed_str_without_spaces, i, "PUSH", 1)?;
                Ok(compiled_line)
            }
            Instructions::Pop => {
                i = parse_pop(
                    &tokenized_line,
                    i,
                    is_org_defined,
                    compiled_bytes,
                    compiled_bytes_ref,
                    variable_ref_map,
                    variable_address_map,
                    &mut compiled_line.label_idx_map,
                    compiled_line_offset_maps,
                )?;
                error_if_hasnt_consumed_all_ins(&lexed_str_without_spaces, i, "POP", 1)?;
                Ok(compiled_line)
            }
        },
        Assembly8086Tokens::AssemblerDirectives(_) => Ok(compiled_line),

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

pub(crate) struct CompiledLineLabelRef<'a> {
    compiled_bytes: &'a [Vec<u8>],
    var_line_num_map: &'a HashMap<Label, (VariableType, CompiledBytesIndexedLineNumber)>,
    label_addr_map: &'a HashMap<UniCase<String>, LineNumber>,
    proc_compiled_bytes_line_start_map: &'a ProcDefinitionStartAndEndBytesMap,
}

impl<'a> CompiledLineLabelRef<'a> {
    pub fn find_label_offset_or_proc_offset(
        &self,
        label: &Label,
        line_number: LineNumber,
    ) -> Option<i16> {
        let label_off = self.find_label_offset(label, line_number);
        if let Some((label_off, is_label_before_ref)) = label_off {
            return Some(label_off as i16 * if is_label_before_ref { -1 } else { 1 });
        }
        let proc_off = self.find_proc_offset(label, line_number);
        if let Some(proc_off) = proc_off {
            return Some(proc_off);
        }
        None
    }

    pub fn find_label_offset(&self, label: &Label, line_number: LineNumber) -> Option<(u16, bool)> {
        let label_addr = self.label_addr_map.get(&UniCase::new(label.to_string()));
        let label_addr = match label_addr {
            None => return None,
            Some(label_addr) => label_addr,
        };
        let (offset, is_label_before_ref) =
            calc_offset(self.compiled_bytes, line_number, *label_addr);
        Some((offset, is_label_before_ref))
    }

    pub fn find_var_as_label_offset(
        &self,
        label: &Label,
        line_number: LineNumber,
    ) -> Option<(u16, bool)> {
        println!("var map: {:?}", self.var_line_num_map);
        let var_defn_line_num = self.var_line_num_map.get(label);
        let var_defn_line_num = match var_defn_line_num {
            None => return None,
            Some(var_defn_line_num) => var_defn_line_num.1,
        };
        let (offset, is_label_before_ref) =
            calc_offset(self.compiled_bytes, line_number, var_defn_line_num);
        Some((offset, is_label_before_ref))
    }

    pub fn find_proc_offset(&self, label: &Label, line_number: LineNumber) -> Option<i16> {
        let proc_reference_line_num = line_number;
        let proc_defined_line_num = self.proc_compiled_bytes_line_start_map.get(label);

        let proc_defined_line_num = match proc_defined_line_num {
            None => return None,
            Some(proc_defined_line_num) => proc_defined_line_num.0,
        };

        let mut offset = 0_i16;
        if proc_defined_line_num < proc_reference_line_num {
            // label defined on top of reference
            for bytes in self
                .compiled_bytes
                .iter()
                .take(proc_reference_line_num)
                .skip(proc_defined_line_num)
            {
                offset -= bytes.len() as i16;
            }
        } else {
            // defined on bottom
            for bytes in self
                .compiled_bytes
                .iter()
                .take(proc_defined_line_num)
                .skip(proc_reference_line_num + 1)
            {
                offset += bytes.len() as i16;
            }
        }

        Some(offset)
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

type IsVariable = bool;

#[allow(clippy::too_many_arguments)]
fn mark_labels(
    // label_ref: &LabelRefrenceList,
    variable_and_label_ref_list: &[(IsVariable, Label, LineNumber, LineNumber)],

    tokenized_line: &Vec<Vec<Token>>,

    compiled_bytes: &mut [Vec<u8>],
    compiled_bytes_ref: &mut [Vec<CompiledBytesReference>],

    label_addr_map: &LabelAddressMap,

    var_ref_compiled_bytes_line_num_map: &HashMap<
        Label,
        (VariableType, CompiledBytesIndexedLineNumber),
    >,
    var_addr_def_map: &VariableAddressDefinitionMap,
    var_abs_addr_map: &mut VariableAddressMap,

    proc_compiled_bytes_line_start_map: &ProcDefinitionStartAndEndBytesMap,

    is_org_defined: bool,
    idx: usize,
) -> Result<bool, CompilationError> {
    if idx >= variable_and_label_ref_list.len() {
        return Ok(true);
    }
    let (_, _, line_number, tokenized_line_idx) = &variable_and_label_ref_list[idx];
    let line_number = *line_number;
    for _ in 0..(variable_and_label_ref_list.len() - idx) {
        let compiled_tokens = compile(
            line_number,
            &tokenized_line[*tokenized_line_idx],
            is_org_defined,
            Some(&CompiledLineLabelRef {
                compiled_bytes,
                label_addr_map,
                proc_compiled_bytes_line_start_map,
                var_line_num_map: var_ref_compiled_bytes_line_num_map,
                // is_org_defined,
            }),
            Some(var_abs_addr_map),
        )?;
        let prev_compiled_bytes_len = compiled_bytes[line_number].len();

        compiled_bytes[line_number] = compiled_tokens.compiled_bytes;
        compiled_bytes_ref[line_number] = compiled_tokens.compiled_bytes_ref;

        let curr_compiled_bytes_len = compiled_bytes[line_number].len();

        if prev_compiled_bytes_len != curr_compiled_bytes_len {
            // update the variables
            calculate_variable_offset_map(
                var_addr_def_map,
                var_abs_addr_map,
                compiled_bytes,
                is_org_defined,
            );

            if idx != 0 {
                return Ok(false);
            } else {
                continue;
            }
        }

        if mark_labels(
            variable_and_label_ref_list,
            tokenized_line,
            compiled_bytes,
            compiled_bytes_ref,
            label_addr_map,
            var_ref_compiled_bytes_line_num_map,
            var_addr_def_map,
            var_abs_addr_map,
            proc_compiled_bytes_line_start_map,
            is_org_defined,
            idx + 1,
        )? {
            return Ok(true);
        } else if idx != 0 {
            return Ok(false);
        } else {
            continue;
        }
    }
    Ok(false)
}

fn calculate_variable_offset_map(
    var_addr_def_map: &VariableAddressDefinitionMap,
    var_abs_addr_map: &mut VariableAddressMap,
    compiled_bytes: &[Vec<u8>],
    is_org_defined: bool,
) {
    // calc offset addr for each var
    for (var_label, (var_type, label_definition_line_number)) in var_addr_def_map {
        let (offset, _) = calc_offset(compiled_bytes, 0, *label_definition_line_number);
        let org_offset = if is_org_defined { 0x100 } else { 0x00 };
        var_abs_addr_map.insert(var_label.clone(), (*var_type, offset + org_offset));
    }
}

fn get_err_if_already_defined_label<T>(
    label_key: UniCase<String>,
    line: &[Token],
    label_addr_map: &HashMap<Label, T>,
    already_defined_line_number: LineNumber,
) -> Option<CompilationError> {
    let idx = line
        .iter()
        .position(|_token| _token.token_type == Assembly8086Tokens::Character(label_key.clone()))
        .unwrap();
    let token = &line[idx];
    if label_addr_map.contains_key(&label_key) {
        return Some(CompilationError::error_with_token(
            token,
            &format!(
                "The label \"{}\" is already defined in line {}, Please use a different name.",
                label_key,
                (already_defined_line_number + 1)
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
        &mut ProcDefinitionLineNumberMap::new(),
        &mut ProcReferenceList::new(),
        &mut MacroBoundsDefintionMap::new(),
        &mut MacroReferenceList::new(),
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

type StartCompiledBytesIndexedLineNumber = LineNumber;
type EndCompiledBytesIndexedLineNumber = LineNumber;
type ProcDefinitionStartAndEndBytesMap = HashMap<
    Label,
    (
        StartCompiledBytesIndexedLineNumber,
        Option<EndCompiledBytesIndexedLineNumber>,
    ),
>;
type MacroBoundsDefintionMapWithOption = HashMap<Label, (ArrayIndex, Option<ArrayIndex>)>;
type TokenAndItsArrayIndex<'a> = Option<(&'a Assembly8086Tokens, usize)>;
fn get_first_two_non_space_characters_token_types<'a>(
    line: &'a [&'a Token],
    i: usize,
) -> (TokenAndItsArrayIndex<'a>, TokenAndItsArrayIndex<'a>) {
    let mut first: Option<(&Assembly8086Tokens, usize)> = None;
    let mut second = None;
    let mut j = 0_usize;
    for token in &line[i..] {
        if token.token_type == Assembly8086Tokens::Space {
            continue;
        }
        if first.is_none() {
            first = Some((&token.token_type, i + j));
        } else if second.is_none() {
            second = Some((&token.token_type, i + j));
            break;
        }
        j += 1;
    }
    (first, second)
}

fn find_macro_bounds(
    lexer: &Lexer,
) -> Result<MacroBoundsDefintionMapWithOption, Vec<CompilationError>> {
    // iterate lexer to find the macro bounds it should be in the form
    // character macro arguments
    // ....
    // ....
    // endm (or) character endm

    let mut macro_defn_map: MacroBoundsDefintionMapWithOption = HashMap::new();
    let mut current_macro_label = None;
    let mut compilaton_errors = Vec::<CompilationError>::new();

    for (i, tokens_vec) in lexer.tokens.iter().enumerate() {
        let toeken_ref_vec = tokens_vec.iter().collect::<Vec<&Token>>();
        let first_and_second_tokens =
            get_first_two_non_space_characters_token_types(&toeken_ref_vec, 0);
        match first_and_second_tokens {
            (
                Some((Assembly8086Tokens::Character(macro_label), token_idx)),
                Some((Assembly8086Tokens::AssemblerDirectives(AssemblerDirectives::Macro), _)),
            ) => {
                if let Some((_, line_num, tokn_idx)) = current_macro_label {
                    let line: &Vec<Token> = &lexer.tokens[line_num];
                    let token = &line[tokn_idx];
                    compilaton_errors.push(CompilationError::error_with_token(token, &format!("You have not closed this MACRO with name {}, Nested macros are not supported", macro_label)));
                }
                macro_defn_map.insert(macro_label.clone(), (i, None));
                current_macro_label = Some((macro_label.clone(), i, token_idx));
            }
            (
                Some((Assembly8086Tokens::Character(macro_label), macro_label_idx)),
                Some((Assembly8086Tokens::AssemblerDirectives(AssemblerDirectives::EndM), _)),
            ) => {
                match &current_macro_label {
                    None => {
                        compilaton_errors.push(CompilationError::error_with_token(
                            &tokens_vec[macro_label_idx+1],
                            &format!(
                                "The macro \"{}\" is not defined, Please define it before ending it. Maybe you are trying to define a recursive Macro ? It isn't supported",
                                macro_label
                            ),
                        ));
                    }
                    Some((current_macro_label, _, _)) => {
                        if current_macro_label != macro_label {
                            compilaton_errors.push(CompilationError::error_with_token(
                                &tokens_vec[macro_label_idx+1],
                                &format!(
                                    "The macro \"{}\" is not defined, Please define it before ending it. Maybe you are trying to define a recursive Macro ? It isn't supported",
                                    macro_label
                                ),
                            ));
                        }
                    }
                }
                if let Some((_, end_idx)) = macro_defn_map.get_mut(macro_label) {
                    *end_idx = Some(i);
                    current_macro_label = None;
                } else {
                    compilaton_errors.push(CompilationError::error_with_token(
                        &tokens_vec[macro_label_idx + 1],
                        &format!(
                            "The macro \"{}\" is not defined, Please define it before ending it.",
                            macro_label
                        ),
                    ));
                }
            }
            (
                Some((
                    Assembly8086Tokens::AssemblerDirectives(AssemblerDirectives::EndM),
                    end_macro_idx,
                )),
                None,
            ) => match &current_macro_label {
                None => {
                    compilaton_errors.push(CompilationError::error_with_token(
                            &tokens_vec[end_macro_idx+1],
                            "No macro defintion found, define a macro using `Name MACRO p1, p2, p3 syntax`, Please define it before ending it. Maybe you are trying to define a recursive Macro ? It isn't supported",
                        ));
                }
                Some((current_macro_label, _, _)) => {
                    if let Some((_, end_idx)) = macro_defn_map.get_mut(current_macro_label) {
                        *end_idx = Some(i);
                    } else {
                        compilaton_errors.push(CompilationError::error_with_token(
                                &tokens_vec[end_macro_idx+1],
                                &format!(
                                    "The macro \"{}\" is not defined, Please define it before ending it.",
                                    current_macro_label
                                ),
                            ));
                    }
                }
            },
            (
                Some((_, idx)),
                Some((Assembly8086Tokens::AssemblerDirectives(AssemblerDirectives::Macro), _)),
            ) => {
                compilaton_errors.push(CompilationError::error_with_token(
                    &tokens_vec[idx + 1],
                    "Macro name must be a character",
                ));
            }
            (
                Some((_, idx)),
                Some((Assembly8086Tokens::AssemblerDirectives(AssemblerDirectives::EndM), _)),
            ) => {
                compilaton_errors.push(CompilationError::error_with_token(
                    &tokens_vec[idx + 1],
                    "Macro name must be a character",
                ));
            }

            (
                Some((Assembly8086Tokens::AssemblerDirectives(AssemblerDirectives::Macro), idx)),
                _,
            ) => {
                compilaton_errors.push(CompilationError::error_with_token(
                    &tokens_vec[idx+1],
                    "Need to define a name for the macro with the folling syntax `Name MACRO p1, p2, p3`",
                ));
            }
            _ => {}
        }
    }

    if !compilaton_errors.is_empty() {
        return Err(compilaton_errors);
    }

    Ok(macro_defn_map)
}

fn find_macro_name(tokens: &[Token]) -> Result<Option<usize>, CompilationError> {
    let mut i = 0;
    let (lexed_str_without_spaces, label) = strip_space_and_comments_and_iterate_labels(tokens);
    if label.is_some() {
        i += 2;
    }
    // check if the next token is db (or) dw
    let first_and_second_tokens =
        get_first_two_non_space_characters_token_types(&lexed_str_without_spaces, i);
    match first_and_second_tokens {
        (
            Some((Assembly8086Tokens::Character(_), label_idx)),
            Some((Assembly8086Tokens::Data(DefineData::Db), _))
            | Some((Assembly8086Tokens::Data(DefineData::Dw), _)),
        ) => {
            return Err(CompilationError::error_with_token(
                lexed_str_without_spaces[label_idx],
                "The macro name cannot be a variable",
            ));
        }
        (
            Some((Assembly8086Tokens::Character(_), _)),
            Some((Assembly8086Tokens::AssemblerDirectives(AssemblerDirectives::Macro), _)),
        ) => {}
        (Some((Assembly8086Tokens::Character(_), label_idx)), _) => {
            let token_ref = lexed_str_without_spaces[label_idx];
            // iterate tokens to find this tokenidx in the tokens
            let token_idx = tokens.iter().position(|token| token.is_abs_eq(token_ref));
            return Ok(token_idx);
        }
        _ => {}
    }

    Ok(None)
}

fn get_args_seperated_by_comma_from_line(
    line: &[Token],
    start_idx: usize,
) -> Result<Vec<usize>, CompilationError> {
    // get only certain toekns from the line
    // line = token1 arg1, arg2, arg3
    // return = arg1, arg2, arg3
    let mut args: Vec<usize> = Vec::<usize>::new();
    let mut new_line = Vec::new();
    for tokn in line[start_idx..].iter() {
        new_line.push(tokn.clone());
    }

    let (stripped_line, _) = strip_space_and_comments_and_iterate_labels(new_line.as_slice());
    iterate_with_seperator(
        0,
        stripped_line.len(),
        &TokenizedLine::new(&stripped_line, stripped_line.len() as u32),
        &Assembly8086Tokens::Comma,
        |token| {
            let token_idx = line.iter().position(|tokn| tokn == token).unwrap();
            args.push(token_idx);
            Ok(())
        },
    )?;
    Ok(args)
}

fn get_parameter_to_argument<'a>(
    line: &'a [Token],
    macro_ref_start_index: usize,
    macro_definitin_line: &'a [Token],
    // macro_def_start_index: usize,
) -> Result<HashMap<&'a Token, &'a Token>, CompilationError> {
    // iterate to find the idex
    let macro_def_start_index = macro_definitin_line
        .iter()
        .position(|token| {
            token.token_type == Assembly8086Tokens::AssemblerDirectives(AssemblerDirectives::Macro)
        })
        .unwrap();
    let mut parameter_to_argument = HashMap::<&Token, &Token>::new();

    let parameter_args =
        get_args_seperated_by_comma_from_line(macro_definitin_line, macro_def_start_index + 1)?;
    let argument_args = get_args_seperated_by_comma_from_line(line, macro_ref_start_index)?;

    if parameter_args.len() != argument_args.len() {
        return Err(CompilationError::error_with_token(
            &line[macro_ref_start_index - 1],
            &format!(
                "The macro \"{}\" is defined with {} arguments, but you have passed {} arguments",
                line[macro_ref_start_index - 1].token_type,
                parameter_args.len(),
                argument_args.len()
            ),
        ));
    }

    for (parameter, argument) in parameter_args.iter().zip(argument_args.iter()) {
        let macro_def_parameter = &macro_definitin_line[*parameter];
        let macro_ref_argument = &line[*argument];
        parameter_to_argument.insert(macro_def_parameter, macro_ref_argument);
    }

    Ok(parameter_to_argument)
}

fn expand_macros(
    lexer: &mut Lexer,
    macro_bounds: HashMap<Label, (usize, usize)>,
    macro_ref_list: &mut MacroReferenceList,
) -> Result<(), Vec<CompilationError>> {
    let mut compilation_errors = Vec::<CompilationError>::new();
    let mut changes: Vec<(usize, usize, Vec<Vec<Token>>)> = Vec::new();
    let mut line_number = 0;

    for tokens_vec in &lexer.tokens {
        line_number += 1;

        let macro_name_index = match find_macro_name(tokens_vec) {
            Ok(macro_name_index) => macro_name_index,
            Err(err) => {
                compilation_errors.push(err);
                continue;
            }
        };
        if let Some(macro_reference_label_index) = macro_name_index {
            let macro_label = &tokens_vec[macro_reference_label_index];
            let macro_label_str = match &macro_label.token_type {
                Assembly8086Tokens::Character(c) => c,
                _ => {
                    compilation_errors.push(CompilationError::error_with_token(
                        macro_label,
                        "This shouldn't happen, Please report this! Error: 906 in expand macros",
                    ));
                    continue;
                }
            };

            let macro_bounds = match macro_bounds.get(macro_label_str) {
                Some(macro_bounds) => macro_bounds,
                None => {
                    compilation_errors.push(CompilationError::error_with_token(
                        macro_label,
                        "This shouldn't happen, Unable to get macro bounds. Please report this! Error: 916 in expand macros",
                    ));
                    continue;
                }
            };

            macro_ref_list.push((macro_label_str.clone(), macro_label.clone(), line_number));

            let (start_idx, end_idx) = *macro_bounds;
            // convert vec<Toekn> to vec<&Token>
            let macro_ref_line_start = tokens_vec;
            let macro_def_line_start = &lexer.tokens[start_idx];

            let parameters_to_arguments_map = match get_parameter_to_argument(
                macro_ref_line_start,
                macro_reference_label_index + 1,
                macro_def_line_start.as_slice(),
            ) {
                Ok(parameters_to_arguments_map) => parameters_to_arguments_map,
                Err(err) => {
                    compilation_errors.push(err);
                    continue;
                }
            };

            // start from start_idx and end at end_idx push all the tokens into the lexer
            let mut temp_vec = Vec::new();
            for i in (start_idx + 1)..=(end_idx - 1) {
                let mut tokens = lexer.tokens[i].clone();
                for token in &mut tokens {
                    if let Some(argument) = parameters_to_arguments_map.get(token) {
                        let new_token = Token::new(
                            argument.token_type.clone(),
                            argument.line_number,
                            argument.column_number,
                            argument.token_length,
                        );
                        *token = new_token;
                    }
                }
                temp_vec.push(tokens);
            }
            changes.push((line_number - 1, macro_reference_label_index, temp_vec));
        }
    }

    // change the macro defition lines to ';' comment token use iterators
    lexer.tokens = lexer
        .tokens
        .clone()
        .into_iter()
        .enumerate()
        .map(|(i, tokens)| {
            // check if the line is a by iterating the macro_bounds using iteratos
            let is_in_macro = macro_bounds.iter().any(|(_, (start, end))| {
                if i >= *start && i <= *end {
                    return true;
                }
                false
            });
            if is_in_macro {
                let mut tokens = tokens.clone();
                let first = tokens.first();
                if let Some(first) = first {
                    let mut first_clone = first.clone();
                    first_clone.token_type = Assembly8086Tokens::Comment;
                    tokens.insert(0, first_clone);
                }
                tokens
            } else {
                tokens
            }
        })
        .collect();

    // sort chages by line number
    changes.sort_by(|(line_number1, _, _), (line_number2, _, _)| line_number1.cmp(line_number2));

    // make the changes into the lexer
    // delete the lexer the chanes line number
    // replace the single line with maybe mutlpile lines defined in chages
    let mut index_offset = 0;
    for (change_line_number, ref_label_idx, change) in changes {
        // preserve label: from the macro expansion
        let line = &lexer.tokens[change_line_number];
        let ref_label_idx = if ref_label_idx > 0 {
            ref_label_idx - 1
        } else {
            ref_label_idx
        };
        let front = &line[..ref_label_idx]; // don't include the macro name
        lexer.tokens[change_line_number + index_offset] = front.to_vec();
        // expand the macro
        for tokens in change {
            index_offset += 1;
            lexer
                .tokens
                .insert(change_line_number + index_offset, tokens);
        }
    }

    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn compile_lines_perform_var_label_substiution(
    lexer: &mut Lexer,
    compilation_errors: &mut Vec<CompilationError>,
    compiled_bytes_lines_vec: &mut Vec<Vec<u8>>,
    compiled_bytes_ref_lines_vec: &mut Vec<Vec<CompiledBytesReference>>,
    label_addr_map: &mut LabelAddressMap,
    label_ref: &mut LabelRefrenceList,
    var_addr_def_map: &mut VariableAddressDefinitionMap,
    var_ref: &mut VariableReferenceList,
    proc_line_num_map: &mut ProcDefinitionLineNumberMap,
    proc_ref_vec: &mut ProcReferenceList,

    macro_line_num_map: &mut MacroBoundsDefintionMap,
    macro_ref_list: &mut MacroReferenceList,
) -> Option<bool> {
    let is_org_defined = match is_org_defined(&lexer.tokens) {
        Ok(is_org_defined) => is_org_defined,
        Err(err) => {
            compilation_errors.push(err);
            false
        }
    };

    let macro_bounds = match find_macro_bounds(lexer) {
        Ok(macro_bounds) => macro_bounds,
        Err(err) => {
            compilation_errors.extend(err);
            return None;
        }
    };

    // check if the end is defined in all the keys if not push to compilation errror
    // and convert macro_bonds to HashMap<Label, (usize, usize)>
    let macro_bounds = match macro_bounds
        .into_iter()
        .map(|(label, (start, end))| match end {
            None => {
                let line = &lexer.tokens[start];
                // find the first non space char
                let token = line
                    .iter()
                    .find(|token| token.token_type != Assembly8086Tokens::Space)
                    .unwrap();
                compilation_errors.push(CompilationError::error_with_token(
                    token,
                    &format!(
                        "The macro \"{}\" is not ended, Please end it with `ENDM`",
                        label
                    ),
                ));
                None
            }
            Some(end) => Some((label, (start, end))),
        })
        .collect::<Option<HashMap<Label, (usize, usize)>>>()
    {
        Some(macro_bounds) => macro_bounds,
        None => return None,
    };
    macro_line_num_map.extend(macro_bounds.clone());

    if !compilation_errors.is_empty() {
        return None;
    }

    match expand_macros(lexer, macro_bounds, macro_ref_list) {
        Ok(()) => {}
        Err(err) => {
            compilation_errors.extend(err);
            return None;
        }
    };

    type CompiledBytesIndexedLineNumber = LineNumber;
    let mut label_compiled_bytes_line_number_map =
        HashMap::<Label, CompiledBytesIndexedLineNumber>::new();
    let mut var_compiled_bytes_line_number_map =
        HashMap::<Label, (VariableType, CompiledBytesIndexedLineNumber)>::new();

    let mut proc_compiled_bytes_line_number_map = ProcDefinitionStartAndEndBytesMap::new();

    let mut labels_used_as_offsets = Vec::new();

    for (i, line) in lexer.tokens.iter().enumerate() {
        match compile(i, line, is_org_defined, None, None) {
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
                    label_compiled_bytes_line_number_map.insert(
                        label_key.clone(),
                        compiled_bytes_lines_vec.len() as CompiledBytesIndexedLineNumber,
                    );
                    label_addr_map.insert(label_key, i as LineNumber);
                }

                // Pushing all the variables into a map after checking if they are already defined
                for (var_str, (variable_type, _)) in compiled_line.variable_abs_address_map {
                    let label_key = UniCase::new(var_str.to_string().clone());
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
                    var_addr_def_map.insert(var_str, (variable_type, i as LineNumber));
                    var_compiled_bytes_line_number_map.insert(
                        label_key.clone(),
                        (
                            variable_type,
                            compiled_bytes_lines_vec.len() as CompiledBytesIndexedLineNumber,
                        ),
                    );
                }

                // pushing all the proc defintions into a map after chacking if they are already defined
                for (proc_label, proc_type) in compiled_line.proc_definition_map {
                    let label_key = UniCase::new(proc_label.to_string().clone());
                    let already_defined_line_number = proc_compiled_bytes_line_number_map
                        .get(&label_key)
                        .unwrap_or(&(0, None))
                        .0;
                    let line_number =
                        compiled_bytes_lines_vec.len() as CompiledBytesIndexedLineNumber;
                    let label_token = get_label_token_from_line(lexer, i, &proc_label);
                    match proc_type {
                        ProcDefitionType::Proc => {
                            if let Some(err) = get_err_if_already_defined_label(
                                label_key.clone(),
                                line,
                                &proc_compiled_bytes_line_number_map,
                                already_defined_line_number,
                            ) {
                                compilation_errors.push(err);
                                continue;
                            }

                            proc_compiled_bytes_line_number_map.insert(
                                label_key.clone(),
                                (line_number as CompiledBytesIndexedLineNumber, None),
                            );
                            proc_line_num_map.insert(label_key.clone(), (i as LineNumber, None));
                        }
                        ProcDefitionType::EndP => {
                            // error if didn't find a proc defintion
                            if !proc_compiled_bytes_line_number_map.contains_key(&label_key) {
                                compilation_errors.push(CompilationError::error_with_token(
                                    label_token.unwrap(),
                                    &format!(
                                        "The proc \"{}\" is not defined, Please define it.",
                                        label_key
                                    ),
                                ));
                                continue;
                            }
                            // error if already defined as endp
                            if let Some((_, Some(_))) =
                                proc_compiled_bytes_line_number_map.get(&label_key)
                            {
                                compilation_errors.push(CompilationError::error_with_token(
                                    label_token.unwrap(),
                                    &format!(
                                        "The proc \"{}\" is already defined as ENDP, Please use a different name.",
                                        label_key
                                    ),
                                ));
                                continue;
                            }

                            // push the endp into the map
                            proc_compiled_bytes_line_number_map.insert(
                                label_key.clone(),
                                (already_defined_line_number, Some(line_number)),
                            );
                            let already_defined_line_number =
                                proc_line_num_map.get(&label_key).unwrap_or(&(0, None)).0;
                            proc_line_num_map.insert(
                                label_key.clone(),
                                (already_defined_line_number, Some(i as LineNumber)),
                            );
                        }
                    }
                }

                // Pushing all the labels that reference a particular label
                for (label_str, (token, _, is_offset)) in compiled_line.label_idx_map {
                    let label = UniCase::new(label_str);
                    if is_offset {
                        labels_used_as_offsets.push(label.clone());
                    }
                    label_ref.push((
                        label.clone(),
                        token,
                        compiled_bytes_lines_vec.len() as CompiledBytesIndexedLineNumber,
                        i,
                    ));
                }

                // Pushing all the variable that reference a particular variable
                for (var_str, (label_type, _)) in compiled_line.variable_reference_map {
                    var_ref.push((
                        var_str.clone(),
                        label_type,
                        compiled_bytes_lines_vec.len() as CompiledBytesIndexedLineNumber,
                        i,
                    ));
                }

                // pushing all the proc references into a vec
                for (proc_label, _) in compiled_line.proc_reference_map {
                    let placeholder_token = get_label_token_from_line(lexer, i, &proc_label)
                        .unwrap()
                        .clone();
                    proc_ref_vec.push((
                        proc_label,
                        placeholder_token,
                        compiled_bytes_lines_vec.len() as CompiledBytesIndexedLineNumber,
                        i as LineNumber,
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

    // check between labels and variables
    for (label, (_, line_number)) in var_addr_def_map.iter() {
        let line_number = *line_number;
        if label_addr_map.contains_key(label) {
            let var_token = get_label_token_from_line(lexer, line_number, label).unwrap();
            let label_line_number = *label_addr_map.get(label).unwrap();
            let label_token = get_label_token_from_line(lexer, label_line_number, label).unwrap();
            compilation_errors.push(CompilationError::error_with_token(
                var_token,
                &format!(
                    "The VARIABLE \"{}\" is defined as a label on Line: {}, Please use a different name.",
                    label,
                    label_line_number+1
                ),
            ));
            compilation_errors.push(CompilationError::error_with_token(
                label_token,
                &format!(
                    "The VARIABLE \"{}\" is defined as a variable on Line: {}, Please use a different name.",
                    label,
                    line_number+1
                ),
            ));
        }
    }

    // labels and procs duplicate defintions
    let matching_keys = label_addr_map
        .keys()
        .filter(|key| proc_compiled_bytes_line_number_map.contains_key(key))
        .collect::<Vec<_>>();

    // for all keys in matching_keys push to compilation_errors
    matching_keys.iter().for_each(|key| {
        let line_number = *label_addr_map.get(key).unwrap();
        let token = get_label_token_from_line(lexer, line_number, key).unwrap();
        let (line_number, _) = *proc_line_num_map.get(key).unwrap();
        let token2 = get_label_token_from_line(lexer, line_number, key).unwrap();
        let temp_errors = CompilationError::same_error_on_two_tokens(
            token,
            token2,
            &format!(
                "The label \"{}\" is defined as a LABEL on Line: {} and as PROC on Line: {}, Please use a different name.",
                key, token.line_number+1, token2.line_number + 1
            ),
        );
        compilation_errors.extend(temp_errors)
    });

    // merge label_ref and proc_ref
    label_ref.append(&mut proc_ref_vec.clone());

    // merge label and proc
    label_compiled_bytes_line_number_map.extend(
        proc_compiled_bytes_line_number_map
            .clone()
            .into_iter()
            .map(|(label, (start, _))| (label, start)),
    );

    // merge label_ref and proc_ref
    label_addr_map.extend(
        proc_line_num_map
            .clone()
            .into_iter()
            .map(|(label, (start, _))| (label, start)),
    );

    // variable and proc duplicate defintions
    let matching_keys = var_addr_def_map
        .keys()
        .filter(|key| proc_compiled_bytes_line_number_map.contains_key(key))
        .collect::<Vec<_>>();

    // for all keys in matching_keys push to compilation_errors
    matching_keys.iter().for_each(|key| {
        let (_, line_number) = *var_addr_def_map.get(key).unwrap();
        let token = get_label_token_from_line(lexer, line_number, key).unwrap();
        let (line_number, _) = *proc_line_num_map.get(key).unwrap();
        let token2 = get_label_token_from_line(lexer, line_number, key).unwrap();
        let temp_errors = CompilationError::same_error_on_two_tokens(
            token,
            token2,
            &format!(
                "The label \"{}\" is defined as a VARIABLE on Line: {} and as PROC on Line: {}, Please use a different name.",
                key, token.line_number+1, token2.line_number + 1
            ),
        );
        compilation_errors.extend(temp_errors)
    });

    // check if all the variables are defined
    let mut var_errors = false;
    for (_i, (var, used_as_type, _, tokenized_line_number)) in var_ref.iter().enumerate() {
        let line = &lexer.tokens[*tokenized_line_number];
        let idx = line
            .iter()
            .position(|_token| _token.token_type == Assembly8086Tokens::Character(var.clone()))
            .unwrap();
        let token = &line[idx];
        match &var_addr_def_map.get(var) {
            None => {
                // check if the variable is defined as a label
                if !label_addr_map.contains_key(var) {
                    var_errors = true;
                    compilation_errors.push(CompilationError::error_with_token(
                        token,
                        &format!("The variable \"{}\" is Undefined, Please define it.", var),
                    ));
                }
            }
            &Some((var_type, _)) => {
                if used_as_type == &VariableType::Word && var_type == &VariableType::Byte {
                    var_errors = true;
                    compilation_errors.push(CompilationError::error_with_token(
                        token,
                        &format!(
                            "The VARIABLE \"{}\" is defined as {:?}, but used as {:?}.",
                            var, var_type, used_as_type
                        ),
                    ));
                }
            }
        }
    }

    // check if all the procs are defined
    let mut proc_errors = false;
    for (proc_label, _, _, tokenized_line_number) in proc_ref_vec.iter() {
        let token = get_label_token_from_line(lexer, *tokenized_line_number, proc_label).unwrap();

        if proc_line_num_map.contains_key(proc_label) {
            continue;
        }

        if label_addr_map.contains_key(proc_label) {
            continue;
        }

        proc_errors = true;
        compilation_errors.push(CompilationError::error_with_token(
            token,
            &format!(
                "The PROC \"{}\" is Undefined, Please define it.",
                proc_label
            ),
        ));
    }

    // check if all labels are defined
    let mut label_errors = false;
    for (label, token, _, _) in &mut *label_ref {
        if !label_addr_map.contains_key(label) && !var_addr_def_map.contains_key(label) {
            label_errors = true;
            compilation_errors.push(CompilationError::error_with_token(
                token,
                &format!("The label \"{}\" is Undefined, Please define it.", label),
            ));
        }
        if let Some((VariableType::Byte, line_number)) = var_addr_def_map.get(label) {
            if labels_used_as_offsets.contains(label) {
                continue;
            }
            label_errors = true;
            let var_token = get_label_token_from_line(lexer, *line_number, label).unwrap();
            compilation_errors.push(CompilationError::error_with_token(
                var_token,
                &format!("The label \"{}\" is defined as a 8-bit variable, Please use a 16-bit variable to use it in JMP instruction.", label),
            ));
            compilation_errors.push(CompilationError::error_with_token(token,
                &format!("The label \"{}\" is defined as a 8-bit variable, Please use a 16-bit variable to use it in JMP instruction.", label),
            ));
        }
    }

    if label_errors || var_errors || proc_errors {
        return None;
    }

    let mut var_abs_addr_map = VariableAddressMap::new();
    calculate_variable_offset_map(
        var_addr_def_map,
        &mut var_abs_addr_map,
        compiled_bytes_lines_vec,
        is_org_defined,
    );

    let mut var_label_ref = Vec::new();
    for (var, _, var_line_number, tokenized_line_number) in var_ref.iter() {
        var_label_ref.push((true, var.clone(), *var_line_number, *tokenized_line_number));
    }

    for (label, _, label_line_number, tokenized_line_number) in label_ref.iter() {
        var_label_ref.push((
            false,
            label.clone(),
            *label_line_number,
            *tokenized_line_number,
        ));
    }

    match mark_labels(
        &var_label_ref,
        &lexer.tokens,
        compiled_bytes_lines_vec,
        compiled_bytes_ref_lines_vec,
        &label_compiled_bytes_line_number_map,
        &var_compiled_bytes_line_number_map,
        var_addr_def_map,
        &mut var_abs_addr_map,
        &proc_compiled_bytes_line_number_map,
        is_org_defined,
        0,
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

#[cfg(test)]
mod test_hlt_and_ret {
    use crate::{compiler::compile_str, test_compile};

    test_compile!(add_ax_sp_hlt, "ADD AX, SP \n HLT", |instructions: &Vec<
        u8,
    >| {
        assert_eq!(instructions, &[0x03, 0xC4, 0xF4]);
    });

    test_compile!(add_ax_sp_ret, "ADD AX, SP \n RET", |instructions: &Vec<
        u8,
    >| {
        assert_eq!(instructions, &[0x03, 0xC4, 0xC3]);
    });

    test_compile!(
        add_ax_sp_ret_with_proc,
        "
        PROC main 
        ADD AX, SP \n RET
        ENDP main 
        ",
        |instructions: &Vec<u8>| {
            assert_eq!(instructions, &[0x03, 0xC4, 0xC3]);
        }
    );

    test_compile!(
        macro_def_inc,
        "
mymacro MACRO p1, p1, p3
    inc p1 
    inc p1
    inc p3
endm 

p1: mymacro ax, dx, cx

mymacro bx, cx, dx

inc dx 
jmp p1
        ",
        |instructions: &Vec<u8>| {
            assert_eq!(
                instructions,
                &[0x42, 0x42, 0x41, 0x41, 0x41, 0x42, 0x42, 0xEb, 0xF7]
            );
        }
    );

    test_compile!(
        test_int_10h,
        "
            mov ax, bx
            int 10h 
        ",
        |instructions: &Vec<u8>| {
            assert_eq!(instructions, &[0x8B, 0xC3, 0xCD, 0x10]);
        }
    );
}
