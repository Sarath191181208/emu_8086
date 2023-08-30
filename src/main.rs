pub mod cpu;
pub mod consts;
pub mod memory;

use cpu::CPU;
use memory::Memory;



fn main() {
    let mut cpu = CPU::new();
    let mut mem = Memory::new();
    cpu.reset(&mut mem); // Reset the CPU
    mem.write_byte(0xFFFC, 0xA9); 
}
