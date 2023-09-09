// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod compiler;
pub mod consts;
pub mod cpu;
pub mod memory;

use compiler::{compilation_error::CompilationError, compile_lines};
use cpu::CPU;
use memory::Memory;
use tauri::State;
use std::sync::{Mutex, Arc};

#[derive(Default)]
struct MutableCpu(Arc<Mutex<CPU>>);

#[derive(Default)]
struct MutableMem(Arc<Mutex<Memory>>);

#[tauri::command]
fn next( cpu: State<'_, MutableCpu>, mem: State<'_, MutableMem> ) -> (CPU, Memory){
    let mut cpu = cpu.0.lock().unwrap();
    let mut mem = mem.0.lock().unwrap();
    cpu.execute(&mut mem);

    (cpu.clone(), mem.clone())
}

#[tauri::command]
fn compile_code(
    code: String,
    cpu: State<'_, MutableCpu>,
    mem: State<'_, MutableMem>,
) -> Result<(CPU, Memory), Vec<CompilationError>> {
    let (compile_bytes, _) = compile_lines(&code, false)?;
    let mut cpu = cpu.0.lock().unwrap();
    let mut mem = mem.0.lock().unwrap();
    cpu.reset(&mut mem);
    cpu.write_instructions(&mut mem, &compile_bytes);
    Ok((cpu.clone(), mem.clone()))
}

#[tauri::command]
fn compile_code_and_run(code: String) -> Result<(CPU, Memory), Vec<CompilationError>> {
    let mut mem = Memory::new();
    let mut cpu = CPU::new();

    // compile the code
    let (compile_bytes, _) = compile_lines(&code, false)?;
    cpu.reset(&mut mem);

    // write the compiled bytes to memory
    for (i, byte) in compile_bytes.iter().enumerate() {
        mem.write_byte(0x100 + (i as u16), *byte);
    }

    loop {
        if mem.read(cpu.get_instruciton_pointer()) == 0 {
            break;
        }
        cpu.execute(&mut mem);
    }

    Ok((cpu, mem))
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
            compile_code_and_run,
            try_compile_code,
            compile_code,
            next
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
