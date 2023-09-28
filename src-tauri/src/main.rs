// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod compiler;
pub mod consts;
pub mod cpu;
pub mod memory;
pub mod utils;

use compiler::{
    compilation_error::CompilationError, compile_lines, types_structs::CompiledBytesReference,
};
use consts::Byte;
use cpu::CPU;
use memory::Memory;
use std::sync::{Arc, Mutex};
use tauri::State;

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

#[tauri::command]
fn try_compile_code(code: String) -> Result<(), Vec<CompilationError>> {
    compile_lines(&code, false)?;
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .manage(MutableCpu::default())
        .manage(MutableMem::default())
        .invoke_handler(tauri::generate_handler![
            try_compile_code,
            compile_code,
            next
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
