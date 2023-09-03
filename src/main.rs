pub mod compiler;
pub mod consts;
pub mod cpu;
pub mod memory;

use cpu::CPU;
use memory::Memory;

fn main() {
    let mut cpu = CPU::new();
    let mut mem = Memory::new();
    // let code = "MOV \t AX, SP";
    // let inst = match compiler::compile_str(&code, true){
    //     Ok(instructions) => {
    //         instructions
    //     },
    //     Err(e) => {
    //         e.print_compilation_error(&code);
    //         return;
    //     }
    // };

    let u8_vec: Vec<u8> = vec![0, 1, 2, 3, 4, 5, 6, 7];
    for high_reg_idx in &u8_vec {
        for low_reg_idx in &u8_vec {
            let ins = (0xC0) | (high_reg_idx / 2) << 4;
            let ins2 = low_reg_idx | (high_reg_idx % 2) << 3;
            print!("{} ", ins | ins2);
        }
    }
}
