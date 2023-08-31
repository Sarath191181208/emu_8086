use crate::consts::{Word, Byte};

use super::CPU;

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
}
