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
        .invoke_handler(tauri::generate_handler![
            compile_code_and_run,
            try_compile_code
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod test {
    use crate::{compiler::compile_lines, cpu::CPU, memory::Memory};

    #[test]
    fn test_comp() {
        let code = "MOV AX, BX \n MOV CX, DX \n \n MOV CX, AX \n MOV AX, 0x1f11";
        let mut mem = Memory::new();
        let mut cpu = CPU::new();

        // compile the code
        let (compile_bytes, _) = compile_lines(&code, false).unwrap();
        cpu.reset(&mut mem);

        println!("{:?}", compile_bytes);

        // write the compiled bytes to memory
        for (i, byte) in compile_bytes.iter().enumerate() {
            mem.write_byte(0x100 + (i as u16), *byte);
        }

        // run untill you encounter 0
        loop {
            if mem.read(cpu.get_instruciton_pointer()) == 0 {
                break;
            }
            cpu.execute(&mut mem);
        }
    }
}
