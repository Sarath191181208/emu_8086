use serde::Serialize;

use crate::{
    consts::{Byte, Word},
    Memory,
};
pub mod instructions;
pub(in crate::cpu) mod utils;

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

#[derive(Serialize)]
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
    negative_flag: bool,
    overflow_flag: bool,
    pairity_flag: bool,
    auxiliary_carry_flag: bool,
    interrupt_disable_flag: bool,
    direction_flag: bool,
}

impl CPU {
    generate_byte_access_methods!(ax);
    generate_byte_access_methods!(bx);
    generate_byte_access_methods!(cx);
    generate_byte_access_methods!(dx);
}

impl Default for CPU {
    fn default() -> Self {
        Self::new()
    }
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
            pairity_flag: false,
            auxiliary_carry_flag: false,
            direction_flag: false,
            overflow_flag: false,
            negative_flag: false,
        }
    }

    pub fn get_instruciton_pointer(&self) -> Word {
        self.instruction_pointer
    }

    // fn set_instruciton_pointer(&mut self, value: Word) {
    //     self.instruction_pointer = value;
    // }

    pub fn write_instructions(&mut self, mem: &mut Memory, instructions: &[Byte]) {
        for (i, instruction) in instructions.iter().enumerate() {
            mem.write_byte(self.instruction_pointer + (i as u16), *instruction);
        }
    }

    pub fn reset(&mut self, mem: &mut Memory) {
        self.instruction_pointer = 0x0100;
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
        self.pairity_flag = false;
        self.auxiliary_carry_flag = false;
        self.direction_flag = false;
        self.overflow_flag = false;
        self.negative_flag = false;

        mem.reset();
    }

    fn consume_instruction(&mut self, mem: &Memory) -> Byte {
        let opcode = mem.read(self.get_instruciton_pointer());
        self.instruction_pointer += 1;
        opcode
    }

    fn peek_instruction(&self, mem: &Memory) -> Byte {
        mem.read(self.get_instruciton_pointer())
    }

    pub fn execute(&mut self, mem: &mut Memory) {
        let opcode = self.consume_instruction(mem);
        match opcode {
            // ADD 8bit register, 8bit register
            0x02 => self.execute_add_register_byte(mem),
            // ADD 16bit register addressing
            0x03 => self.execute_add_register_word(mem),

            // ADD AL, 0x12 i.e immediate addressing
            0x04 => self.add_al_in_immediate_addressing(mem),

            // ADD AX, 0x1234 i.e immediate addressing
            0x05 => self.add_ax_in_immediate_addressing(mem),

            // SUB, AL, 8bit register
            0x2A => self.execute_sub_register_byte(mem),

            // SUB AX, 16bit register
            0x2B => self.execute_sub_register_word(mem),

            // SUB, AL, 0x12 i.e immediate addressing
            0x2C => self.sub_al_in_immediate_addressing(mem),

            // SUB, AX, 0x1234 i.e immediate addressing
            0x2D => self.sub_ax_in_immediate_addressing(mem),

            // INC 16bit register
            0x40..=0x47 => self.execute_inc_word_register(opcode),
            // DEC 16bit register
            0x48..=0x4F => self.execute_dec_word_register(opcode),

            // ADD, SUB 8bit register, immediate_addressing
            0x80 => {
                let opcode = self.peek_instruction(mem);
                match opcode {
                    0xC0..=0xC7 => self.execute_add_immediate_byte(mem),
                    0xE8..=0xEF => self.execute_sub_immediate_byte(mem),
                    _ => panic!("Unimplemented opcode: {:X}", opcode),
                }
            }

            // ADD, SUB 16bit register, immediate_addressing
            0x81 | 0x83 => {
                let _opcode = self.peek_instruction(mem);
                match _opcode {
                    0xC0..=0xC7 => self.execute_add_immediate_word(mem, opcode),
                    0xE8..=0xEF => self.execute_sub_immediate_word(mem, opcode),
                    _ => panic!("Unimplemented opcode: {:X}", opcode),
                }
            }

            // MOV 16bit register, 16bit register
            0x8A => self.execute_mov_register_byte(mem),
            0x8B => self.execute_mov_register_word(mem),

            // MOV 16bit register, 0x1234
            0xB0..=0xB7 => self.execute_direct_mov_byte(mem, opcode),
            0xB8..=0xBF => self.execute_direct_mov_word(mem, opcode),

            // JMP 16bit register
            0xE9 => self.execute_jmp_16bit(mem),

            // JMP 8bit register 
            0xEB => self.execute_jmp_8bit(mem),

            // INC 8bit register
            0xFE => {
                let opcode = self.peek_instruction(mem);
                match opcode {
                    0xC0..=0xC7 => self.execute_inc_register_byte(mem),
                    0xC8..=0xCF => self.execute_dec_register_byte(mem),
                    _ => panic!("Unimplemented opcode: {:X}", opcode),
                }
            }
            _ => panic!("Unimplemented opcode: {:X}", opcode),
        }
    }
}
