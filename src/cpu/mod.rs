use crate::{
    consts::{Byte, Word},
    Memory,
};
pub mod instructions;

pub struct CPU {
    // Memory
    program_counter: Word,
    stack_pointer: Word,
    base_pointer: Word,
    source_index: Word,
    destination_index: Word,

    // Registers
    ax: Word,
    bx: Word,
    cx: Word,
    dx: Word,

    // Status Flags
    carry_flag: bool,
    zero_flag: bool,
    interrupt_disable_flag: bool,
    decimal_mode_flag: bool,
    break_command_flag: bool,
    overflow_flag: bool,
    negative_flag: bool,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            program_counter: 0x0000,
            stack_pointer: 0x0000,
            base_pointer: 0x0000,
            source_index: 0x0000,
            destination_index: 0x0000,

            ax: 0x0000,
            bx: 0x0000,
            cx: 0x0000,
            dx: 0x0000,

            carry_flag: false,
            zero_flag: false,
            interrupt_disable_flag: false,
            decimal_mode_flag: false,
            break_command_flag: false,
            overflow_flag: false,
            negative_flag: false,
        }
    }

    pub fn reset(&mut self, mem: &mut Memory) {
        self.program_counter = 0xFFFC;
        self.stack_pointer = 0x0100;
        self.base_pointer = 0x0000;
        self.source_index = 0x0000;
        self.destination_index = 0x0000;

        self.ax = 0x0000;
        self.bx = 0x0000;
        self.cx = 0x0000;
        self.dx = 0x0000;

        self.carry_flag = false;
        self.zero_flag = false;
        self.interrupt_disable_flag = false;
        self.decimal_mode_flag = false;
        self.break_command_flag = false;
        self.overflow_flag = false;
        self.negative_flag = false;

        mem.reset();
    }

    fn get_flags_as_binary(&self) -> u8{
        let mut flags: u8 = 0;
        flags |= (self.carry_flag as u8) << 0;
        flags |= (self.zero_flag as u8) << 1;
        flags |= (self.interrupt_disable_flag as u8) << 2;
        flags |= (self.decimal_mode_flag as u8) << 3;
        flags |= (self.break_command_flag as u8) << 4;
        flags |= (self.overflow_flag as u8) << 6;
        flags |= (self.negative_flag as u8) << 7;
        flags
    }

    fn consume_instruction(&mut self, mem: &Memory) -> Byte {
        let opcode = mem.read(self.program_counter);
        self.program_counter += 1;
        opcode
    }

    pub fn execute(&mut self, mem: &mut Memory) {
        let opcode = self.consume_instruction(mem);
        match opcode {
            // 0x00 => self.brk(mem),
            // mov instruction opcode =
            0x8B => self.execute_mov(mem),
            _ => panic!("Invalid opcode: {:X}", opcode),
        }
        self.program_counter += 1;
    }
}

#[cfg(test)]
mod tests{
    use crate::memory::Memory;
    use super::CPU;

    #[test]
    fn test_mov_16bit_register_addressing_ax_cx(){
        let mut cpu = CPU::new(); let mut mem = Memory::new();
        cpu.reset(&mut mem); cpu.cx = 0xFF00;

        // check this operation 
        // MOV AX, CX 
        // 0x8B 0xC1
        mem.write_byte(0xFFFC, 0x8B); mem.write_byte(0xFFFD, 0xC1);
        cpu.execute(&mut mem);
        assert_eq!(cpu.ax, cpu.cx); assert_eq!(cpu.ax, 0xFF00); assert_eq!(cpu.cx, 0xFF00);
    }
}
