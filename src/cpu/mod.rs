use crate::{
    consts::{Byte, Word},
    Memory,
};
pub mod instructions;

macro_rules! generate_byte_access_methods {
    ($register:ident) => {
        paste::item! {
            pub fn [<get_ $register _high>](&self) -> Byte {
                ((self.$register & 0xFF00) >> 8) as Byte
            }
        }

        paste::item! {
            pub fn [<get_ $register _low>](&self) -> Byte {
                (self.$register & 0xFF) as Byte
            }
        }

        paste::item! {
            pub fn [<set_ $register _high>](&mut self, value: Byte) {
                self.$register = (self.$register & 0xFF) | ((value as Word) << 8);
            }
        }

        paste::item! {
            pub fn [<set_ $register _low>](&mut self, value: Byte) {
                self.$register = (self.$register & 0xFF00) | (value as Word);
            }
        }
    };
}

pub struct CPU {
    // Memory
    instruction_pointer: Word,
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
    generate_byte_access_methods!(ax);
    generate_byte_access_methods!(bx);
    generate_byte_access_methods!(cx);
    generate_byte_access_methods!(dx);
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            instruction_pointer: 0x0000,
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
        self.instruction_pointer = 0xFFFC;
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

    fn get_flags_as_binary(&self) -> u8 {
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
        let opcode = mem.read(self.instruction_pointer);
        self.instruction_pointer += 1;
        opcode
    }

    pub fn execute(&mut self, mem: &mut Memory) {
        let opcode = self.consume_instruction(mem);
        match opcode {
            
            // MOV AX, BX i.e register addressing 
            0x8A => self.execute_mov_byte(mem),
            0x8B => self.execute_mov_word(mem),
            
            // MOV AX, 0x1234 i.e immediate addressing 
            0xB0..=0xB7 => self.execute_direct_mov_byte(mem, opcode),
            0xB8..=0xBF => self.execute_direct_mov_word(mem, opcode),
            _ => panic!("Invalid opcode: {:X}", opcode),
        }
    }
}
