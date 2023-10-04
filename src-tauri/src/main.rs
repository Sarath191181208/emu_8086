// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod compiler;
pub mod consts;
pub mod cpu;
pub mod memory;
pub mod utils;

use compiler::{
    compilation_error::CompilationError, compile_lines, types_structs::CompiledBytesReference,
    utils::get_label_token_from_line,
};
use consts::Byte;
use cpu::CPU;
use memory::Memory;
use std::sync::{Arc, Mutex};
use tauri::State;
use utils::TokenPosition;

use crate::compiler::{
    compile_lines_perform_var_label_substiution,
    lexer::Lexer,
    types_structs::{
        LabelAddressMap, LabelRefrenceList, VariableAddressDefinitionMap, VariableReferenceList,
    },
};

type CompilationErrors = Vec<CompilationError>;
type CompiledBytesReferences = Vec<CompiledBytesReference>;
type MemoryChanges = Vec<(usize, Byte)>;

#[derive(Default)]
struct MutableCpu(Arc<Mutex<CPU>>);

#[derive(Default)]
struct MutableMem(Arc<Mutex<Memory>>);

#[tauri::command]
fn next(cpu: State<'_, MutableCpu>, mem: State<'_, MutableMem>) -> (CPU, MemoryChanges) {
    let mut cpu = cpu.0.lock().unwrap();
    let mut mem = mem.0.lock().unwrap();
    cpu.execute(&mut mem);

    (*cpu, mem.get_recent_new_bytes())
}

#[tauri::command]
fn compile_code(
    code: String,
    cpu: State<'_, MutableCpu>,
    mem: State<'_, MutableMem>,
) -> Result<(CPU, CompiledBytesReferences, MemoryChanges), CompilationErrors> {
    let (compile_bytes, compiled_bytes_ref, is_org_defined) = compile_lines(&code, true)?;
    let mut cpu = cpu.0.lock().unwrap();
    let mut mem = mem.0.lock().unwrap();
    cpu.reset(&mut mem);
    if is_org_defined {
        cpu.set_org_defined();
    }
    cpu.write_instructions(&mut mem, &compile_bytes);
    Ok((*cpu, compiled_bytes_ref, mem.get_recent_new_bytes()))
}

type DefintionTokenPosition = TokenPosition;
type ReferenceTokenPosition = TokenPosition;

#[tauri::command]
fn get_label_and_var_address_definitions(
    code: String,
) -> Vec<(DefintionTokenPosition, ReferenceTokenPosition)> {
    let mut lexer = Lexer::new();
    lexer.tokenize(&code);

    let mut label_addr_map = LabelAddressMap::new();
    let mut label_ref = LabelRefrenceList::new();

    let mut var_addr_def_map = VariableAddressDefinitionMap::new();
    let mut var_ref = VariableReferenceList::new();

    compile_lines_perform_var_label_substiution(
        &mut lexer,
        &mut Vec::new(),
        &mut Vec::new(),
        &mut Vec::new(),
        &mut label_addr_map,
        &mut label_ref,
        &mut var_addr_def_map,
        &mut var_ref,
    );

    let mut label_and_var_address_definitions = Vec::new();

    for (label, ref_token, _, _) in label_ref {
        match label_addr_map.get(&label) {
            Some(line_number) => {
                let def_token = get_label_token_from_line(&lexer, *line_number, &label).unwrap();
                label_and_var_address_definitions.push((
                    DefintionTokenPosition::from(def_token),
                    ReferenceTokenPosition::from(&ref_token),
                ));
            }
            None => {}
        }
    }

    for (label, _, _, tokenized_line_number) in var_ref {
        let ref_token = get_label_token_from_line(&lexer, tokenized_line_number, &label).unwrap();
        match var_addr_def_map.get(&label) {
            Some((_, line_number)) => {
                let def_token = get_label_token_from_line(&lexer, *line_number, &label).unwrap();
                label_and_var_address_definitions.push((
                    DefintionTokenPosition::from(def_token),
                    ReferenceTokenPosition::from(ref_token),
                ));
            }
            None => {}
        }
    }

    label_and_var_address_definitions
}

#[tauri::command]
fn try_compile_code(code: String) -> Result<(), Vec<CompilationError>> {
    let mut lexer = Lexer::new();
    lexer.tokenize(&code);

    let mut compilation_errors = Vec::new();

    match compile_lines_perform_var_label_substiution(
        &mut lexer,
        &mut compilation_errors,
        &mut Vec::new(),
        &mut Vec::new(),
        &mut LabelAddressMap::new(),
        &mut LabelRefrenceList::new(),
        &mut VariableAddressDefinitionMap::new(),
        &mut VariableReferenceList::new(),
    ) {
        Some(_) => Ok(()),
        None => Err(compilation_errors),
    }
}

fn main() {
    tauri::Builder::default()
        .manage(MutableCpu::default())
        .manage(MutableMem::default())
        .invoke_handler(tauri::generate_handler![
            try_compile_code,
            get_label_and_var_address_definitions,
            compile_code,
            next
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
