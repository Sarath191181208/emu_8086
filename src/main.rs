pub mod cpu;
pub mod consts;
pub mod memory;
pub mod compiler;

use cpu::CPU;
use memory::Memory;



fn main() {
    let mut cpu = CPU::new();
    let mut mem = Memory::new();
    let code = "MOV \t AX, SP";
    let inst = match compiler::compile_str(&code, true){
        Ok(instructions) => {
            instructions
        },
        Err(e) => {
            e.print_compilation_error(&code);
            return;
        }
    };
}
