use serde::Serialize;

use crate::{
    consts::{Byte, Word, U20},
    Memory,
};

use self::{interrupt::Interrupt, ports_handler::Ports};
pub mod instructions;
pub mod interrupt;
pub mod ports_handler;
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

    // Ports
    pub ports: Ports,
}

impl CPU {
    generate_byte_access_methods!(ax);
    generate_byte_access_methods!(bx);
    generate_byte_access_methods!(cx);
    generate_byte_access_methods!(dx);

    pub fn get_instruciton_pointer(&self) -> Word {
        self.instruction_pointer
    }

    pub(self) fn set_instruction_pointer(&mut self, value: Word) {
        self.instruction_pointer = value;
    }

    pub(self) fn set_instruction_pointer_from_16bitoffset(&mut self, offset: Word) {
        if offset & 0x8000 != 0 {
            let offset = 0xFFFF - offset + 1;
            self.instruction_pointer = self.instruction_pointer.wrapping_sub(offset);
        } else {
            self.instruction_pointer = self.instruction_pointer.wrapping_add(offset);
        }
    }

    pub(self) fn set_ax(&mut self, value: Word) {
        self.ax = value;
    }

    pub(self) fn set_cx(&mut self, value: Word) {
        self.cx = value;
    }

    pub fn get_code_segment(&self) -> Word {
        self.code_segment
    }

    pub fn set_code_segment(&mut self, value: Word) {
        self.code_segment = value;
    }

    pub fn get_port(&self, port: Byte) -> Byte {
        self.ports.get(port)
    }

    pub fn get_port_word(&self, port: Byte) -> Word {
        let low_byte = self.get_port(port);
        let high_byte = self.get_port(port + 1);
        ((high_byte as Word) << 8) | (low_byte as Word)
    }

    pub fn set_port(&mut self, port: Byte, value: Byte) {
        self.ports.set(port, value);
    }

    pub fn set_port_word(&mut self, port: Byte, value: Word) {
        let low_byte = value as Byte;
        let high_byte = (value >> 8) as Byte;
        self.set_port(port, low_byte);
        self.set_port(port + 1, high_byte);
    }
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

            ports: Ports::new(),
        }
    }

    pub fn reset(&mut self, mem: &mut Memory) {
        self.instruction_pointer = 0x0000;
        self.stack_pointer = 0xFFFE;
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

        self.ports.reset();
        mem.reset();
        self.write_0x10_interrupt_procedure(mem);
        self.write_0x21_interrupt_procedure(mem);
    }

    pub fn set_org_defined(&mut self) {
        self.instruction_pointer = 0x100;
        self.code_segment = 0x700;
        self.data_segment = 0x700;
        self.stack_segment = 0x700;
        self.extra_segment = 0x700;
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

    fn execute_unknown_ins(&mut self, mem: &mut Memory, _opcode: Byte) {
        println!("-----------------------------------");
        println!("Unknown instruction: {:X}", _opcode);
        println!("-----------------------------------");
        self.execute_nop(mem);
    }

    pub fn execute(&mut self, mem: &mut Memory) -> Option<Interrupt> {
        let opcode = self.consume_instruction(mem);
        match opcode {
            // ADD [0x1234], AL
            0x00 => self.execute_add_address_and_8bit_register(mem),

            // ADD [0x1234], AX
            0x01 => self.execute_add_indexed_addr_16bit_register(mem),

            // ADD 8bit register, 8bit register
            0x02 => self.execute_add_register(mem),

            // ADD AX, _ 16bit register, direct addressing
            0x03 => self.execute_add(mem),

            // ADD AL, 0x12 i.e immediate addressing
            0x04 => self.add_al_in_immediate_addressing(mem),

            // ADD AX, 0x1234 i.e immediate addressing
            0x05 => self.add_ax_in_immediate_addressing(mem),

            // PUSH ES
            0x06 => self.execute_push_es(mem),

            // POP ES
            0x07 => self.execute_pop_es(mem),

            // OR b.mem, reg/mem
            0x08 => self.execute_or_byte_addr_as_first_operand(mem),

            // OR w.mem, reg/mem
            0x09 => self.execute_or_word_addr_as_first_operand(mem),

            // OR 8bit register, reg/mem
            0x0A => self.execute_or_8bit_reg(mem),

            // OR 16bit register, reg/mem
            0x0B => self.execute_or_16bit_reg(mem),

            // OR AL, 0x12 i.e immediate addressing
            0x0C => self.or_al_in_immediate_addressing(mem),

            // OR AX, 0x1234 i.e immediate addressing
            0x0D => self.or_ax_in_immediate_addressing(mem),

            // PUSH CS
            0x0E => self.execute_push_cs(mem),
            0x0F => self.execute_unknown_ins(mem, opcode),

            // ADC mem, reg8
            0x10 => self.execute_adc_byte_addr_as_first_operand(mem),

            // ADC mem, reg16
            0x11 => self.execute_adc_word_addr_as_first_operand(mem),

            // ADC reg8, reg8/mem
            0x12 => self.execute_adc_8bit_reg(mem),

            // ADC reg16, reg16/mem
            0x13 => self.execute_adc_16bit_reg(mem),

            // ADC AL, 0x12 i.e immediate addressing
            0x14 => self.adc_al_in_immediate_addressing(mem),

            // ADC AX, 0x1234 i.e immediate addressing
            0x15 => self.adc_ax_in_immediate_addressing(mem),

            // PUSH SS
            0x16 => self.execute_push_ss(mem),
            // POP SS
            0x17 => self.execute_pop_ss(mem),

            // SBB 8bit register, reg/mem
            0x18 => self.execute_sbb_byte_addr_as_first_operand(mem),

            // SBB 16bit register, reg/mem
            0x19 => self.execute_sbb_word_addr_as_first_operand(mem),

            // SBB b.mem, reg/mem
            0x1A => self.execute_sbb_8bit_reg(mem),

            // SBB w.mem, reg/mem
            0x1B => self.execute_sbb_16bit_reg(mem),

            // SBB AL, 0x12 i.e immediate addressing
            0x1C => self.sbb_al_in_immediate_addressing(mem),

            // SBB AX, 0x1234 i.e immediate addressing
            0x1D => self.sbb_ax_in_immediate_addressing(mem),

            // PUSH DS
            0x1E => self.execute_push_ds(mem),
            // POP DS
            0x1F => self.execute_pop_ds(mem),

            // AND Address, reg/mem
            0x20 => self.execute_and_byte_addr_as_first_operand(mem),

            // AND Address, reg/mem
            0x21 => self.execute_and_word_addr_as_first_operand(mem),

            // AND 8bit Register, 8bit Register/Memory
            0x22 => self.execute_and_8bit_reg(mem),

            // AND 16bit Register, 16bit Register/Memory
            0x23 => self.execute_and_16bit_reg(mem),

            // AND AL, 0x12 i.e immediate addressing
            0x24 => self.and_al_in_immediate_addressing(mem),

            // AND AX, 0x1234 i.e immediate addressing
            0x25 => self.and_ax_in_immediate_addressing(mem),

            // SUB [0x1234], AL
            0x28 => self.execute_sub_direct_addr_8bit_register(mem),

            // SUB [0x1234], AX
            0x29 => self.execute_sub_indexed_addr_16bit_register(mem),

            // SUB, AL, 8bit register
            0x2A => self.execute_sub_register_byte(mem),

            // SUB AX, 16bit register
            0x2B => self.execute_sub_register_word(mem),

            // SUB, AL, 0x12 i.e immediate addressing
            0x2C => self.sub_al_in_immediate_addressing(mem),

            // SUB, AX, 0x1234 i.e immediate addressing
            0x2D => self.sub_ax_in_immediate_addressing(mem),

            // XOR mem, reg8
            0x30 => self.execute_xor_byte_addr_as_first_operand(mem),

            // XOR mem, reg16
            0x31 => self.execute_xor_word_addr_as_first_operand(mem),

            // XOR reg8, reg8/mem
            0x32 => self.execute_xor_8bit_reg(mem),

            // XOR reg16, reg16/mem
            0x33 => self.execute_xor_16bit_reg(mem),

            // XOR AL, 0x12 i.e immediate addressing
            0x34 => self.xor_al_in_immediate_addressing(mem),

            // XOR AX, 0x1234 i.e immediate addressing
            0x35 => self.xor_ax_in_immediate_addressing(mem),

            // CMP [0x1234], AL
            0x38 => self.execute_cmp_byte_addr_as_first_operand(mem),

            // CMP [0x1234], AX
            0x39 => self.execute_cmp_word_addr_as_first_operand(mem),

            // CMP 8bit register, 8bit register/mem
            0x3A => self.execute_cmp_8bit_reg(mem),

            // CMP AX, 16bit register/mem
            0x3B => self.execute_cmp_16bit_reg(mem),

            // INC 16bit register
            0x40..=0x47 => self.execute_inc_word_register(opcode),
            // DEC 16bit register
            0x48..=0x4F => self.execute_dec_word_register(opcode),

            // PUSH 16bit register
            0x50..=0x57 => self.execute_push_word_register(mem, opcode),

            // POP 16bit register
            0x58..=0x5F => self.execute_pop_word_register(mem, opcode),

            // PUSH label/offset_u16
            0x68 => self.execute_push_16bit_number(mem),

            // PUSH label/offset_u8
            0x6A => self.execute_push_8bit_number(mem),

            // ADD, SUB 8bit register, immediate_addressing
            0x80 => {
                let opcode = self.peek_instruction(mem);
                match opcode {
                    0x06 => self.add_direct_address_8bit_val_immediate_value(mem),
                    0x0E => self.execute_or_byte_addr_and_number(mem),
                    0x16 => self.execute_adc_byte_addr_and_number(mem),
                    0x1E => self.execute_sbb_byte_addr_and_number(mem),
                    0x26 => self.execute_and_byte_addr_and_number(mem),
                    0x2E => self.sub_direct_address_8bit_val_immediate_value(mem),
                    0x36 => self.execute_xor_byte_addr_and_number(mem),
                    0x3E => self.execute_cmp_byte_addr_and_number(mem),
                    0xC0..=0xC7 => self.execute_add_immediate_byte(mem),
                    0xC8..=0xCF => self.execute_or_8bit_reg_and_number(mem),
                    0xD0..=0xD7 => self.execute_adc_8bit_reg_and_number(mem),
                    0xD8..=0xDF => self.execute_sbb_8bit_reg_and_number(mem),
                    0xE0..=0xE7 => self.execute_and_8bit_reg_and_number(mem),
                    0xE8..=0xEF => self.execute_sub_immediate_byte(mem),
                    0xF0..=0xF7 => self.execute_xor_8bit_reg_and_number(mem),
                    0xF8..=0xFF => self.execute_cmp_8bit_reg_and_number(mem),
                    _ => self.execute_unknown_ins(mem, opcode),
                }
            }

            // ADD, SUB 16bit register, immediate_addressing
            0x81 | 0x83 => {
                let _opcode = self.peek_instruction(mem);
                match _opcode {
                    0x06 => self.add_direct_address_16bit_val_immediate_value(mem, opcode),
                    0x0E => self.execute_or_word_addr_and_number(mem, opcode),
                    0x16 => self.execute_adc_word_addr_and_number(mem, opcode),
                    0x1E => self.execute_sbb_word_addr_and_number(mem, opcode),
                    0x26 => self.execute_and_word_addr_and_number(mem, opcode),
                    0x2E => self.sub_direct_address_16bit_val_immediate_value(mem, opcode),
                    0x36 => self.execute_xor_word_addr_and_number(mem, opcode),
                    0xC0..=0xC7 => self.execute_add_reg_immediate_word(mem, opcode),
                    0xC8..=0xCF => self.execute_or_16bit_reg_and_number(mem, opcode),
                    0xD0..=0xD7 => self.execute_adc_16bit_reg_and_number(mem, opcode),
                    0xD8..=0xDF => self.execute_sbb_16bit_reg_and_number(mem, opcode),
                    0xE0..=0xE7 => self.execute_and_16bit_reg_and_number(mem, opcode),
                    0xE8..=0xEF => self.execute_sub_immediate_word(mem, opcode),
                    0xF0..=0xF7 => self.execute_xor_16bit_reg_and_number(mem, opcode),
                    _ => self.execute_unknown_ins(mem, opcode),
                }
            }

            // TEST AL..DH, reg/mem
            0x84 => self.execute_test_8bit_reg(mem),

            // TEST AX..DI, reg/mem
            0x85 => self.execute_test_16bit_reg(mem),

            // XCHG 8bit register/mem, 8bit register/mem
            0x86 => self.execute_xchg_8bit_reg_including_mem(mem),

            // XCHG 16bit register/mem, 16bit register/mem
            0x87 => self.execute_xchg_16bit_regs_including_mem(mem),

            // MOV indexed addressing, 16bit register
            0x89 => self.execute_mov_indexed_addr_16bit_register(mem),

            // MOV 16bit register, 16bit register
            0x8A => self.execute_mov_register_byte(mem),
            0x8B => self.execute_mov_register_word(mem),

            // LEA 16bit reg, mem
            0x8D => self.exec_lea_reg_mem(mem),

            // POP ...
            0x8F => {
                let opcode = self.peek_instruction(mem);
                match opcode {
                    // POP [bx]
                    0x00..=0x07 => self.execute_pop_indexed_addressing_no_offset(mem),
                    // POP [bx + 8bit]
                    0x40..=0x47 => self.execute_pop_indexed_addressing_with_8bit_offset(mem),
                    // POP [bx + 16bit]
                    0x80..=0x87 => self.execute_pop_indexed_addressing_with_16bit_offset(mem),
                    // POP 16bit register
                    0xC0..=0xC7 => {
                        let ins = self.consume_instruction(mem);
                        self.execute_pop_word_register(mem, ins);
                    }
                    _ => self.execute_unknown_ins(mem, opcode),
                }
            }

            // No op
            0x90 => self.execute_nop(mem),
            0x91..=0x97 => self.execute_xchg_ax(opcode),

            // MOV AL, [0x102]
            0xA0 => self.execute_mov_al_direct_addressing(mem),
            // MOV AX, [0x102]
            0xA1 => self.execute_mov_ax_direct_addressing(mem),

            // MOV [0x102], AL
            0xA2 => self.execute_mov_direct_addressing_al(mem),

            // MOV [0x102], AX
            0xA3 => self.execute_mov_direct_addressing_ax(mem),

            // TEST AL, 0x20
            0xA8 => self.execute_test_al_and_number(mem),

            // TEST AX, 0x100
            0xA9 => self.execute_test_ax_and_number(mem),

            // MOV 16bit register, 0x1234
            0xB0..=0xB7 => self.execute_direct_mov_byte(mem, opcode),
            0xB8..=0xBF => self.execute_direct_mov_word(mem, opcode),

            // RET
            0xC3 => self.execute_ret(mem),

            // LES 16bit register, mem
            0xC4 => self.exec_les_16_bit_reg_mem(mem),

            // MOV [0x102], 0x12
            0xC6 => {
                let ins = self.consume_instruction(mem);
                match ins {
                    0x06 => self.execute_mov_direct_addressing_immediate_byte(mem),
                    _ => self.execute_unknown_ins(mem, 0xC6),
                }
            }

            // MOV [0x102], 0x1234
            0xC7 => {
                let ins = self.consume_instruction(mem);
                match ins {
                    0x06 => self.execute_mov_direct_addressing_immediate_word(mem),
                    _ => self.execute_unknown_ins(mem, 0xC7),
                }
            }

            // INT
            0xCD => self.execute_interrupt(mem),

            // IRET
            0xCF => self.execute_iret(mem),

            0xE2 => self.execute_loop_8bit(mem),
            0xE3 => self.execute_jmp_if_cx_zero_8bit(mem),

            // IN
            0xE4 => self.execute_in_al_8bit(mem),
            0xE5 => self.execute_in_ax_8bit(mem),

            // OUT
            0xE6 => self.execute_out_8bit_al(mem),
            0xE7 => self.execute_out_8bit_ax(mem),

            // CALL 16 bit address
            0xE8 => self.execute_call_and_16bitaddr(mem),

            // JMP 16bit register
            0xE9 => self.execute_jmp_16bit(mem),

            // JMP 8bit register
            0xEB => self.execute_jmp_8bit(mem),

            // IN AL, DX
            0xEC => self.execute_in_al_dx(),
            0xED => self.execute_in_ax_dx(),

            // OUT DX, AL
            0xEE => self.execute_out_dx_al(),
            0xEF => self.execute_out_dx_ax(),

            // HLT
            0xF4 => {
                // TODO: Implelemnt HLT as a breakpoint
                let _ = self.consume_instruction(mem);
            }

            // MUL 8bit register
            0xF6 => {
                let opcode = self.peek_instruction(mem);
                match opcode {
                    // TEST b.[0x100], 0x12
                    0x06 => self.execute_test_byte_indexed_addressing_and_number(mem),
                    0x26 => self.execute_mul_address_8bit(mem),
                    // TEST AL..DH, 0x12
                    0xC0..=0xC7 => self.execute_test_8bit_reg_and_number(mem),
                    0xE0..=0xE7 => self.execute_mul_8bit(mem),
                    _ => self.execute_unknown_ins(mem, opcode),
                }
            }

            // MUL 16bit register
            0xF7 => {
                let opcode = self.peek_instruction(mem);
                match opcode {
                    // TEST w.[0x100], 0x1234
                    0x06 => self.execute_test_word_indexed_addressing_and_number(mem),
                    0x26 => self.execute_mul_address_16bit(mem),
                    // TEST BX..DI, 0x1234
                    0xC0..=0xC7 => self.execute_test_16bit_reg_and_number(mem),
                    0xE0..=0xE7 => self.execute_mul_16bit(mem),
                    _ => self.execute_unknown_ins(mem, opcode),
                }
            }

            0xFE => {
                let opcode = self.peek_instruction(mem);
                match opcode {
                    // INC [0x1234]
                    0x06 => self.execute_inc_address_8bit(mem),
                    // DEC [0x1234]
                    0x0E => self.execute_dec_address_8bit(mem),
                    // INC AL | BH | CL ..
                    0xC0..=0xC7 => self.execute_inc_register_byte(mem),
                    // DEC AL | BH | CL ..
                    0xC8..=0xCF => self.execute_dec_register_byte(mem),
                    _ => self.execute_unknown_ins(mem, 0xFE),
                }
            }

            0xFF => {
                let opcode = self.peek_instruction(mem);
                match opcode {
                    // INC [0x1234]
                    0x06 => self.execute_inc_address_16bit(mem),
                    // DEC [0x1234]
                    0x0E => self.execute_dec_address_16bit(mem),
                    // JMP [0x1234]
                    0x26 => self.execute_jmp_abs_address(mem),
                    // PUSH indexed addr no offset
                    0x30..=0x37 => self.execute_push_indexed_addressing_no_offset(mem),
                    // PUSH indexed addr with 8bit-offset
                    0x70..=0x77 => self.execute_push_indexed_addressing_with_8bit_offset(mem),
                    // PUSH indexed addr with 16bit-offset
                    0xB0..=0xB7 => self.execute_push_indexed_addressing_with_16bit_offset(mem),
                    // BIOS DI
                    0xFF => {
                        let int = self.execute_bios_di(mem);
                        return Some(int);
                    }
                    _ => self.execute_unknown_ins(mem, 0xFF),
                }
            }
            _ => self.execute_unknown_ins(mem, opcode),
        }
        None
    }
}

// memory operations
impl CPU {
    pub fn write_instructions(&mut self, mem: &mut Memory, instructions: &[Byte]) {
        mem.write_instructions(self.code_segment, self.instruction_pointer, instructions);
    }

    fn read_word_from_u20(&self, mem: &Memory, offset: U20) -> Word {
        mem.read_word_with_u20(offset)
    }

    fn read_byte_from_u20(&self, mem: &Memory, offset: U20) -> Byte {
        mem.read_byte_with_u20(offset)
    }

    fn read_word_from_pointer(&self, mem: &Memory, pointer: Word) -> Word {
        mem.read_word(self.data_segment, pointer)
    }

    fn read_byte_from_pointer(&self, mem: &Memory, pointer: Word) -> Byte {
        mem.read_byte(self.data_segment, pointer)
    }

    fn write_byte_to_u20(&mut self, mem: &mut Memory, offset: U20, data: Byte) {
        mem.write_byte_with_u20(offset, data);
    }

    fn write_word_to_u20(&mut self, mem: &mut Memory, offset: U20, data: Word) {
        mem.write_word_with_u20(offset, data);
    }

    fn write_byte_from_pointer(&self, mem: &mut Memory, pointer: Word, data: Byte) {
        mem.write_byte(self.data_segment, pointer, data);
    }

    fn write_word_from_pointer(&self, mem: &mut Memory, pointer: Word, data: Word) {
        mem.write_word(self.data_segment, pointer, data);
    }
}

// stack operations
impl CPU {
    fn pop_stack(&mut self, mem: &mut Memory) -> Word {
        let sp = self.stack_pointer;
        let value = self.read_word_from_pointer(mem, sp);
        self.stack_pointer = sp.wrapping_add(2);
        value
    }

    fn push_stack(&mut self, mem: &mut Memory, value: Word) {
        let sp = self.stack_pointer.wrapping_sub(2);
        mem.write_word(self.stack_segment, sp, value);
        self.stack_pointer = sp;
    }

    #[allow(dead_code)]
    fn print_stack(&self, mem: &Memory) {
        // print all the valus starting from sp and ending with 0xFFFF
        // ----------
        // |   val  |
        // ----------
        // |   val  |
        // ----------
        println!("                |------------|");

        // go from 0xFFFE to sp
        for sp in (self.stack_pointer..0xFFFF).step_by(2).rev() {
            let value = self.read_word_from_pointer(mem, sp);
            println!(
                "0x{:04X}: 0x{:04X}: |   0x{:04X}   |",
                self.stack_segment, sp, value
            );
            println!("                |------------|");
        }
    }
}
