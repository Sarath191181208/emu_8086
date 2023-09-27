#![allow(unused)]

use super::CPU;
use crate::consts::{Byte, Word};

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

    pub(in crate::cpu) fn get_index_from_06_e6_pattern(&self, instruction: Byte) -> u8 {
        (instruction - 0x06) >> 3
    }
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
}
