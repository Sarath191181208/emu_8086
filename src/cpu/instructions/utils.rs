use crate::{consts::Word, cpu::CPU};

impl CPU {
    fn set_16bit_flags(&mut self, a: u16, b: u16, result: u16, overflow: bool) {
        self.overflow_flag = a & 0x8000 == b & 0x8000 && result & 0x8000 != a & 0x8000;
        self.carry_flag = overflow;
        self.zero_flag = result == 0;
        self.negative_flag = result & 0x8000 != 0;
        self.auxiliary_carry_flag = (a & 0xFF) + (b & 0xFF) > 0xFF;
        self.pairity_flag = ((result & 0xFF) as u8).count_ones() % 2 == 0;
    }

    fn set_8bit_flags(&mut self, a: u8, b: u8, result: u8, overflow: bool) {
        self.carry_flag = overflow;
        self.zero_flag = result == 0;
        self.negative_flag = result & 0x80 != 0;
        self.auxiliary_carry_flag = (a as u16 + b as u16) > 0xFF;
        self.pairity_flag = result.count_ones() % 2 == 0;
    }

    pub fn add_16bit_with_overflow_and_set_flags(&mut self, a: Word, b: Word) -> (Word, bool) {
        let (result, overflow) = a.overflowing_add(b);
        self.set_16bit_flags(a, b, result, overflow);
        (result, overflow)
    }

    pub fn add_8bit_with_overflow_and_set_flags(&mut self, a: u8, b: u8) -> (u8, bool) {
        let (result, overflow) = a.overflowing_add(b);
        self.set_8bit_flags(a, b, result, overflow);
        (result, overflow)
    }

    pub fn sub_16bit_with_overflow_and_set_flags(&mut self, a: Word, b: Word) -> (Word, bool) {
        let (result, overflow) = a.overflowing_sub(b);
        self.set_16bit_flags(a, b, result, overflow);
        self.overflow_flag = a & 0x8000 != b & 0x8000 && result & 0x8000 != a & 0x8000;
        self.auxiliary_carry_flag = (a & 0x0F) < (b & 0x0F);
        (result, overflow)
    }

    pub fn sub_8bit_with_overflow_and_set_flags(&mut self, a: u8, b: u8) -> (u8, bool) {
        let (result, overflow) = a.overflowing_sub(b);
        self.set_8bit_flags(a, b, result, overflow);
        (result, overflow)
    }
}
