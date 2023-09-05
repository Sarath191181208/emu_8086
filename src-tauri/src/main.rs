// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod compiler;
pub mod consts;
pub mod cpu;
pub mod memory;

use compiler::{compilation_error::CompilationError, compile_lines};
use cpu::CPU;
use memory::Memory;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

#[tauri::command]
fn compile_code_and_run(code: String) -> Result<(CPU, Memory), Vec<CompilationError>> {
    let mut mem = Memory::new();
    let mut cpu = CPU::new();

    // compile the code
    let (compile_bytes, _) = compile_lines(&code, false)?;
    cpu.reset(&mut mem);

    // write the compiled bytes to memory
    cpu.set_instruciton_pointer();
    for (i, byte) in compile_bytes.iter().enumerate() {
        mem.write_byte(0x100 + (i as u16), *byte);
    }

    // count the number of lines in code 
    let line_count = code.lines().count();
    for _ in 0..line_count {
        cpu.execute(&mut mem);
    }

    Ok((cpu, mem))
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![compile_code_and_run])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
