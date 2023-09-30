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

#[derive(Serialize, Clone, Copy)]
pub struct CPU {
    // Memory
    instruction_pointer: Word,
    stack_pointer: Word,
    base_pointer: Word,
    source_index: Word,
    destination_index: Word,

    // Segments
    code_segment: Word,
    stack_segment: Word,
    data_segment: Word,
    extra_segment: Word,

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

            stack_segment: 0x0100,
            code_segment: 0x0100,
            data_segment: 0x0100,
            extra_segment: 0x0100,

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

    pub fn get_code_segment(&self) -> Word {
        self.code_segment
    }

    pub fn set_org_defined(&mut self) {
        self.instruction_pointer = 0x100;
        self.code_segment = 0x700;
        self.data_segment = 0x700;
        self.stack_segment = 0x700;
        self.extra_segment = 0x700;
    }

    pub fn write_instructions(&mut self, mem: &mut Memory, instructions: &[Byte]) {
        mem.write_instructions(self.code_segment, self.instruction_pointer, instructions);
    }

    pub fn reset(&mut self, mem: &mut Memory) {
        self.instruction_pointer = 0x0000;
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

        self.stack_segment = 0x0100;
        self.code_segment = 0x0100;
        self.data_segment = 0x0100;
        self.extra_segment = 0x0100;

        mem.reset();
    }

    fn read_word_from_pointer(&self, mem: &Memory, pointer: Word) -> Word {
        mem.read_word(self.data_segment, pointer)
    }

    fn read_byte_from_pointer(&self, mem: &Memory, pointer: Word) -> Byte {
        mem.read_byte(self.data_segment, pointer)
    }

    fn write_byte_from_pointer(&self, mem: &mut Memory, pointer: Word, data: Byte) {
        mem.write_byte(self.data_segment, pointer, data);
    }

    fn write_word_from_pointer(&self, mem: &mut Memory, pointer: Word, data: Word) {
        mem.write_word(self.data_segment, pointer, data);
    }

    fn consume_instruction(&mut self, mem: &Memory) -> Byte {
        let opcode = mem.read_byte(self.code_segment, self.instruction_pointer);
        self.instruction_pointer += 1;
        opcode
    }

    fn consume_word(&mut self, mem: &Memory) -> Word {
        let low_byte = self.consume_instruction(mem);
        let high_byte = self.consume_instruction(mem);
        ((high_byte as Word) << 8) | (low_byte as Word)
    }

    fn consume_byte(&mut self, mem: &Memory) -> Byte {
        self.consume_instruction(mem)
    }

    fn peek_instruction(&self, mem: &Memory) -> Byte {
        mem.read_byte(self.code_segment, self.instruction_pointer)
    }

    fn execute_nop(&mut self, mem: &mut Memory) {
        let _ = self.consume_instruction(mem);
    }

    pub fn execute(&mut self, mem: &mut Memory) {
        let opcode = self.consume_instruction(mem);
        match opcode {
            0x00 => self.execute_add_address_and_8bit_register(mem),

            // ADD [0x1234], AX
            0x01 => self.execute_add_address_and_16bit_register(mem),

            // ADD 8bit register, 8bit register
            0x02 => self.execute_add_register(mem),

            // ADD AX, _ 16bit register, direct addressing
            0x03 => self.execute_add(mem),

            // ADD AL, 0x12 i.e immediate addressing
            0x04 => self.add_al_in_immediate_addressing(mem),

            // ADD AX, 0x1234 i.e immediate addressing
            0x05 => self.add_ax_in_immediate_addressing(mem),

            // SUB [0x1234], AL
            0x28 => self.execute_sub_direct_addr_8bit_register(mem),

            // SUB [0x1234], AX
            0x29 => self.execute_sub_direct_addr_16bit_register(mem),

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
                    0x06 => self.add_direct_address_8bit_val_immediate_value(mem),
                    0x2E => self.sub_direct_address_8bit_val_immediate_value(mem),
                    0xC0..=0xC7 => self.execute_add_immediate_byte(mem),
                    0xE8..=0xEF => self.execute_sub_immediate_byte(mem),
                    _ => unimplemented!("Unimplemented opcode: {:X} for operation 0x80", opcode),
                }
            }

            // ADD, SUB 16bit register, immediate_addressing
            0x81 | 0x83 => {
                let _opcode = self.peek_instruction(mem);
                match _opcode {
                    0x06 => self.add_direct_address_16bit_val_immediate_value(mem, opcode),
                    0x2E => self.sub_direct_address_16bit_val_immediate_value(mem, opcode),
                    0xC0..=0xC7 => self.execute_add_reg_immediate_word(mem, opcode),
                    0xE8..=0xEF => self.execute_sub_immediate_word(mem, opcode),
                    _ => unimplemented!(
                        "Unimplemented opcode: {:X} for operation 0x81 | 0x83",
                        opcode
                    ),
                }
            }

            // MOV 16bit register, 16bit register
            0x8A => self.execute_mov_register_byte(mem),
            0x8B => self.execute_mov_register_word(mem),

            // No op
            0x90 => self.execute_nop(mem),

            // MOV AL, [0x102]
            0xA0 => self.execute_mov_al_direct_addressing(mem),
            // MOV AX, [0x102]
            0xA1 => self.execute_mov_ax_direct_addressing(mem),

            // MOV [0x102], AL
            0xA2 => self.execute_mov_direct_addressing_al(mem),

            // MOV [0x102], AX
            0xA3 => self.execute_mov_direct_addressing_ax(mem),

            // MOV 16bit register, 0x1234
            0xB0..=0xB7 => self.execute_direct_mov_byte(mem, opcode),
            0xB8..=0xBF => self.execute_direct_mov_word(mem, opcode),

            // MOV [0x102], 0x12
            0xC6 => {
                let ins = self.consume_instruction(mem);
                match ins {
                    0x06 => self.execute_mov_direct_addressing_immediate_byte(mem),
                    _ => unimplemented!("Unimplemented opcode: {:X} for operation 0xC7", ins),
                }
            }

            // MOV [0x102], 0x1234
            0xC7 => {
                let ins = self.consume_instruction(mem);
                match ins {
                    0x06 => self.execute_mov_direct_addressing_immediate_word(mem),
                    _ => unimplemented!("Unimplemented opcode: {:X} for operation 0xC7", ins),
                }
            }

            // JMP 16bit register
            0xE9 => self.execute_jmp_16bit(mem),

            // JMP 8bit register
            0xEB => self.execute_jmp_8bit(mem),

            // MUL 8bit register
            0xF6 => {
                let opcode = self.peek_instruction(mem);
                match opcode {
                    0xE0..=0xE7 => self.execute_mul_8bit(mem),
                    _ => unimplemented!("Unimplemented opcode: {:X} for operation 0xF6", opcode),
                }
            }

            // MUL 16bit register
            0xF7 => {
                let opcode = self.peek_instruction(mem);
                match opcode {
                    0xE0..=0xE7 => self.execute_mul_16bit(mem),
                    _ => unimplemented!("Unimplemented opcode: {:X} for operation 0xF7", opcode),
                }
            }

            0xFE => {
                let opcode = self.peek_instruction(mem);
                match opcode {
                    // DEC [0x1234]
                    0x0E => self.execute_dec_address_8bit(mem),
                    // INC AL | BH | CL ..
                    0xC0..=0xC7 => self.execute_inc_register_byte(mem),
                    // DEC AL | BH | CL ..
                    0xC8..=0xCF => self.execute_dec_register_byte(mem),
                    _ => unimplemented!("Unimplemented opcode: {:X} for operation 0xFE", opcode),
                }
            }

            0xFF => {
                let opcode = self.peek_instruction(mem);
                match opcode {
                    // DEC [0x1234]
                    0x0E => self.execute_dec_address_16bit(mem),
                    _ => unimplemented!("Unimplemented opcode: {:X} for operation 0xFF", opcode),
                }
            }
            _ => unimplemented!("Unimplemented opcode: {:X}", opcode),
        }
    }
}
