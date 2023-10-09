#![allow(unused)]

use super::CPU;
use crate::consts::{Byte, Word, U20};

macro_rules! sum {
    ($($x:expr),*) => {{
        let mut sum = 0u32;
        $(
            sum += $x as u32;
        )*
        sum
    }};
}

impl CPU {
    pub(in crate::cpu) fn get_16bit_register_by_index(&self, index: u8) -> Word {
        match index {
            0x00 => self.ax,
            0x01 => self.cx,
            0x02 => self.dx,
            0x03 => self.bx,
            0x04 => self.stack_pointer,
            0x05 => self.base_pointer,
            0x06 => self.source_index,
            0x07 => self.destination_index,
            _ => panic!("Invalid register index! This can't happen!"),
        }
    }

    pub(in crate::cpu) fn get_8bit_register_by_index(&self, index: u8) -> Byte {
        match index {
            0x00 => self.get_ax_low(),
            0x01 => self.get_cx_low(),
            0x02 => self.get_dx_low(),
            0x03 => self.get_bx_low(),
            0x04 => self.get_ax_high(),
            0x05 => self.get_cx_high(),
            0x06 => self.get_dx_high(),
            0x07 => self.get_bx_high(),
            _ => panic!("Invalid register index! This can't happen!"),
        }
    }

    pub(in crate::cpu) fn set_16bit_register_by_index(&mut self, index: u8, value: Word) {
        match index {
            0x00 => self.ax = value,
            0x01 => self.cx = value,
            0x02 => self.dx = value,
            0x03 => self.bx = value,
            0x04 => self.stack_pointer = value,
            0x05 => self.base_pointer = value,
            0x06 => self.source_index = value,
            0x07 => self.destination_index = value,
            _ => panic!("Invalid register index! This can't happen!"),
        }
    }

    pub(in crate::cpu) fn set_8bit_register_by_index(&mut self, index: u8, value: Byte) {
        match index {
            0x00 => self.set_ax_low(value),
            0x01 => self.set_cx_low(value),
            0x02 => self.set_dx_low(value),
            0x03 => self.set_bx_low(value),
            0x04 => self.set_ax_high(value),
            0x05 => self.set_cx_high(value),
            0x06 => self.set_dx_high(value),
            0x07 => self.set_bx_high(value),
            _ => panic!("Invalid register index! This can't happen!"),
        }
    }

    pub(in crate::cpu) fn get_offset_from_index_of_indexed_registers(&self, index: u8) -> U20 {
        let ds = (self.data_segment * 0x10) as u32;
        match index {
            0x00 => {
                // bx + si
                let sum = sum!(ds, self.bx, self.source_index);
                U20::from(sum)
            }
            0x01 => {
                // bx + di
                let sum = sum!(ds, self.bx, self.destination_index);
                U20::from(sum)
            }
            0x02 => {
                // bp + si
                let sum = sum!(ds, self.base_pointer, self.source_index);
                U20::from(sum)
            }
            0x03 => {
                // bp + di
                let sum = sum!(ds, self.base_pointer, self.destination_index);
                U20::from(sum)
            }
            0x04 => {
                // si
                let sum = sum!(ds, self.source_index);
                U20::from(sum)
            }
            0x05 => {
                // di
                let sum = sum!(ds, self.destination_index);
                U20::from(sum)
            }
            0x07 => {
                // bx
                let sum = sum!(ds, self.bx);
                U20::from(sum)
            }
            0x06 => {
                // bp
                let sum = sum!(ds, self.base_pointer);
                U20::from(sum)
            }
            _ => panic!("Invalid register index! This can't happen!"),
        }
    }

    pub(in crate::cpu) fn get_index_from_c0_ff_pattern(&self, instruction: Byte) -> (u8, u8) {
        // The pattern is as follows
        // C0 => (0, 0); C1 => (0, 1); C2 => (0, 2); C3 => (0, 3)
        // C4 => (0, 4); C5 => (0, 5); C6 => (0, 6); C7 => (0, 7)

        // C8 => (1, AX); C9 => (1, 1); CA => (1, 2); CB => (1, 3)
        // CC => (1, 4); CD => (1, 5); CE => (1, 6); CF => (1, 7)
        // Here these indices refer to a particular pattern namely
        // 0 => AX; 1 => CX; 2 => DX; 3 => BX; 4 => SP; 5 => BP; 6 => SI; 7 => DI
        //          (or)
        // 0 => AL; 1 => CL; 2 => DL; 3 => BL; 4 => AH; 5 => CH; 6 => DH; 7 => BH
        // The last four bits of the instruction are the source register index
        let source_idx = instruction & 0x0F; // source_idx = 0x08
                                             // This masking is done because the instruction are from C0 -> FF
                                             // This mask extracts the last 2 bits of the instruction which can be indexed to find the destination register
                                             // ex: (0xC8 & 0b00110000) = (0b00000000) i.e in the 0th index
                                             // (0x00) | 0x01 as source_idx > 7 => 0x01 i.e the `c` register
        let prefix = (instruction & 0b00110000) >> 3; // prefix = 0b0011 i.e the destination addr
        let write_idx = if source_idx > 7 {
            prefix | 0x01
        } else {
            prefix
        };
        (source_idx, write_idx)
    }

    pub(in crate::cpu) fn get_index_from_0x40_0x7f_pattern(&self, ins: Byte) -> (u8, u8) {
        self.get_low_high_from_0x3f_difference(0x40, ins)
    }

    pub(in crate::cpu) fn get_index_from_0x80_0xbf_pattern(&self, ins: Byte) -> (u8, u8) {
        self.get_low_high_from_0x3f_difference(0x80, ins)
    }

    pub(in crate::cpu) fn get_index_from_0x00_0x3f_pattern(&self, ins: Byte) -> (u8, u8) {
        self.get_low_high_from_0x3f_difference(0x00, ins)
    }

    pub(in crate::cpu) fn get_low_high_from_0x3f_difference(
        &self,
        start: Byte,
        ins: Byte,
    ) -> (u8, u8) {
        let low = ins & 0x0f;
        let high_idx = (ins & 0b_0111_1000) >> 3;
        let high = high_idx ^ start;
        (low % 8, high % 8)
    }

    pub(in crate::cpu) fn get_index_from_06_e6_pattern(&self, instruction: Byte) -> u8 {
        (instruction - 0x06) >> 3
    }
}

macro_rules! bools_to_u16 {
    ($($bools:expr),*) => {{
        let mut val: u16 = 0;
        let mut i: usize = 0;
        $(
                val |= ($bools as u16) << i;
            i += 1;
        )*
        val
    }}
}

impl CPU {
    pub(in crate::cpu) fn get_flags_as_binary(&self) -> u8 {
        let mut flags: u8 = 0;
        flags |= self.carry_flag as u8;
        flags |= (self.zero_flag as u8) << 1;
        flags |= (self.negative_flag as u8) << 2;
        flags |= (self.overflow_flag as u8) << 3;
        flags |= (self.pairity_flag as u8) << 4;
        flags |= (self.auxiliary_carry_flag as u8) << 5;
        flags |= (self.interrupt_disable_flag as u8) << 6;
        flags |= (self.direction_flag as u8) << 7;
        flags
    }

    pub(in crate::cpu) fn get_flags_as_16bit_number(&self) -> u16 {
        bools_to_u16!(
            (self.carry_flag),
            true,
            (self.pairity_flag),
            false,
            false,
            (self.auxiliary_carry_flag),
            false,
            (self.zero_flag),
            (self.negative_flag),
            false,
            false,
            (!self.interrupt_disable_flag),
            (self.direction_flag),
            (self.overflow_flag)
        )
    }

    pub(in crate::cpu) fn set_flags_from_u16(&mut self, value: u16) {
            self.carry_flag  = get_nth_bit(value, 0);
            // true
            self.pairity_flag = get_nth_bit(value, 2);
            // false,
            // false,
            self.auxiliary_carry_flag = get_nth_bit(value, 5);
            // false,
            self.zero_flag = get_nth_bit(value, 7);
            self.negative_flag = get_nth_bit(value, 8);
            // false,
            // false,
            self.interrupt_disable_flag = ! get_nth_bit(value, 11);
            self.direction_flag = get_nth_bit(value, 12);
            self.overflow_flag = get_nth_bit(value, 13);
    }
}

fn get_nth_bit(value: u16, n: u8) -> bool {
    (value & (1 << n)) != 0
}
