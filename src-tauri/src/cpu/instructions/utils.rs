use crate::{
    consts::{Word, U20},
    cpu::CPU,
    memory::Memory,
};

impl CPU {
    pub(super) fn set_negative_flag_from_16bit_res(&mut self, res: u16) {
        self.negative_flag = res & 0x8000 != 0;
    }

    pub(super) fn set_negative_flag_from_8bit_res(&mut self, res: u8) {
        self.negative_flag = res & 0x80 != 0;
    }

    pub(super) fn set_pairity_flag_from_16bit_res(&mut self, res: u16) {
        self.pairity_flag = ((res & 0xFF) as u8).count_ones() % 2 == 0;
    }

    pub(super) fn set_pairity_flag_from_8bit_res(&mut self, res: u8) {
        self.pairity_flag = res.count_ones() % 2 == 0;
    }

    fn set_16bit_flags(&mut self, a: u16, b: u16, result: u16, overflow: bool) {
        self.overflow_flag = a & 0x8000 == b & 0x8000 && result & 0x8000 != a & 0x8000;
        self.carry_flag = overflow;
        self.auxiliary_carry_flag = (a & 0xFF) + (b & 0xFF) > 0xFF;
        self.zero_flag = result == 0;
        self.set_negative_flag_from_16bit_res(result);
        self.set_pairity_flag_from_16bit_res(result);
    }

    fn set_8bit_flags(&mut self, a: u8, b: u8, result: u8, overflow: bool) {
        self.carry_flag = overflow;
        self.zero_flag = result == 0;
        self.auxiliary_carry_flag = (a as u16 + b as u16) > 0xFF;
        self.set_negative_flag_from_8bit_res(result);
        self.set_pairity_flag_from_8bit_res(result);
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
        self.overflow_flag = a & 0x80 != b & 0x80 && result & 0x80 != a & 0x80;
        self.auxiliary_carry_flag = (a & 0x0F) < (b & 0x0F);
        (result, overflow)
    }

    pub fn dec_from_16bitvalue_and_set_flags(&mut self, value: u16) -> u16 {
        let prev_carry_flag = self.carry_flag;
        let (val, _) = self.sub_16bit_with_overflow_and_set_flags(value, 1);
        self.carry_flag = prev_carry_flag;
        val
    }

    pub fn dec_from_8bitvalue_and_set_flags(&mut self, value: u8) -> u8 {
        let prev_carr_flag = self.carry_flag;
        let (val, _) = self.sub_8bit_with_overflow_and_set_flags(value, 1);
        self.carry_flag = prev_carr_flag;
        val
    }

    pub(super) fn inc_from_16bitvalue_and_set_flags(&mut self, value: u16) -> u16 {
        let prev_carr_flag = self.carry_flag;
        let (val, _) = self.add_16bit_with_overflow_and_set_flags(value, 1);
        self.carry_flag = prev_carr_flag;
        val
    }

    pub(super) fn inc_from_8bitvalue_and_set_flags(&mut self, value: u8) -> u8 {
        let prev_carr_flag = self.carry_flag;
        let (val, _) = self.add_8bit_with_overflow_and_set_flags(value, 1);
        self.carry_flag = prev_carr_flag;
        val
    }

    pub(super) fn consume_byte_and_get_cummulative_offset(
        &mut self,
        mem: &mut Memory,
        reg_idx: u8,
    ) -> U20 {
        // getting the offset defined in ins i.e 0x20
        let offset = U20::from(self.consume_byte(mem));
        // getting the offset from the index of indexed registers i.e from [bx+si] | [bx]
        let memory_offset = self.get_offset_from_index_of_indexed_registers(reg_idx);
        offset + memory_offset
    }

    pub(super) fn consume_word_and_get_cummulative_offset(
        &mut self,
        mem: &mut Memory,
        reg_idx: u8,
    ) -> U20 {
        // getting the offset defined in ins i.e 0x20
        let offset = U20::from(self.consume_word(mem));
        // getting the offset from the index of indexed registers i.e from [bx+si] | [bx]
        let memory_offset = self.get_offset_from_index_of_indexed_registers(reg_idx);
        offset + memory_offset
    }

    pub(super) fn consume_bytes_and_parse_8bit_reg_as_first_arg_double_ins(
        &mut self,
        mem: &mut Memory,
        exec_fn: &dyn Fn(&mut CPU, u8, u8) -> Option<u8>,
    ) {
        // This is a function where the first argument is a 16bit register and the second argument is a 16bit reg or a memory address
        // The function consumes bytes from the memory and extracts the values of the fowllling addressing
        // For example: MOV AX, [0x1234] | MOV AX, [BX+SI] | MOV AX, [BX] | MOV AX, [0x1234] it calculates the address and gets the value from the memory
        // and executes the exec_fn with the values of the register and the memory value
        // If the function returns a value it sets the register to that value

        let ins = self.consume_instruction(mem);
        let (res, reg_idx) = match ins {
            0x06 | 0x0E | 0x16 | 0x1E | 0x26 | 0x2E | 0x36 | 0x3E => {
                // MVI
                let reg_idx = ins / 0x06 - 1;
                let addr = self.consume_word(mem);
                let val = self.read_byte_from_pointer(mem, addr);
                let reg_val = self.get_8bit_register_by_index(reg_idx);
                let res = exec_fn(self, reg_val, val);
                (res, reg_idx)
            }
            0x00..=0x3F => {
                let (indexed_addr_idx, reg_idx) = self.get_index_from_0x00_0x3f_pattern(ins);
                let mem_addr = self.get_offset_from_index_of_indexed_registers(indexed_addr_idx);
                let mem_val = self.read_byte_from_u20(mem, mem_addr);
                let reg_val = self.get_8bit_register_by_index(reg_idx);
                let res = exec_fn(self, reg_val, mem_val);
                (res, reg_idx)
            }
            0x40..=0x7F => {
                let (indexed_addr_idx, reg_idx) = self.get_index_from_0x40_0x7f_pattern(ins);
                let mem_addr = self.consume_byte_and_get_cummulative_offset(mem, indexed_addr_idx);
                let mem_val = self.read_byte_from_u20(mem, mem_addr);
                let reg_val = self.get_8bit_register_by_index(reg_idx);
                let res = exec_fn(self, reg_val, mem_val);
                (res, reg_idx)
            }
            0x80..=0xBF => {
                let (indexed_addr_idx, reg_idx) = self.get_index_from_0x80_0xbf_pattern(ins);
                let mem_addr = self.consume_word_and_get_cummulative_offset(mem, indexed_addr_idx);
                let mem_val = self.read_byte_from_u20(mem, mem_addr);
                let reg_val = self.get_8bit_register_by_index(reg_idx);
                let res = exec_fn(self, reg_val, mem_val);
                (res, reg_idx)
            }
            0xC0..=0xFF => {
                let (low_reg, reg_idx) = self.get_index_from_c0_ff_pattern(ins);
                let high_val = self.get_8bit_register_by_index(reg_idx % 8 );
                let low_val = self.get_8bit_register_by_index(low_reg % 8);
                let res = exec_fn(self, high_val, low_val);
                (res, reg_idx)
            }
        };
        if let Some(res) = res {
            self.set_8bit_register_by_index(reg_idx, res);
        }
    }

    pub(super) fn consume_bytes_and_parse_16bit_reg_as_first_arg_double_ins(
        &mut self,
        mem: &mut Memory,
        exec_fn: &dyn Fn(&mut CPU, u16, u16) -> Option<u16>,
    ) {
        // This is a function where the first argument is a 16bit register and the second argument is a 16bit reg or a memory address
        // The function consumes bytes from the memory and extracts the values of the fowllling addressing
        // For example: MOV AX, [0x1234] | MOV AX, [BX+SI] | MOV AX, [BX] | MOV AX, [0x1234] it calculates the address and gets the value from the memory
        // and executes the exec_fn with the values of the register and the memory value
        // If the function returns a value it sets the register to that value
        let ins = self.consume_instruction(mem);
        let (res, reg_idx): (Option<u16>, u8) = match ins {
            0x06 | 0x0E | 0x16 | 0x1E | 0x26 | 0x2E | 0x36 | 0x3E => {
                let reg_idx = ins / 0x06 - 1;
                let addr = self.consume_word(mem);
                let val = self.read_word_from_pointer(mem, addr);
                let reg_val = self.get_16bit_register_by_index(reg_idx);
                let res = exec_fn(self, reg_val, val);
                (res, reg_idx)
            }
            0x00..=0x3F => {
                let (indexed_addr_idx, reg_idx) = self.get_index_from_0x00_0x3f_pattern(ins);
                let mem_addr = self.get_offset_from_index_of_indexed_registers(indexed_addr_idx);
                let mem_val = self.read_word_from_u20(mem, mem_addr);
                let reg_val = self.get_16bit_register_by_index(reg_idx);
                let res = exec_fn(self, reg_val, mem_val);
                (res, reg_idx)
            }
            0x40..=0x7F => {
                let (indexed_addr_idx, reg_idx) = self.get_index_from_0x40_0x7f_pattern(ins);
                let mem_addr = self.consume_byte_and_get_cummulative_offset(mem, indexed_addr_idx);
                let mem_val = self.read_word_from_u20(mem, mem_addr);
                let reg_val = self.get_16bit_register_by_index(reg_idx);
                let res = exec_fn(self, reg_val, mem_val);
                (res, reg_idx)
            }
            0x80..=0xBF => {
                let (indexed_addr_idx, reg_idx) = self.get_index_from_0x80_0xbf_pattern(ins);
                let mem_addr = self.consume_word_and_get_cummulative_offset(mem, indexed_addr_idx);
                let mem_val = self.read_word_from_u20(mem, mem_addr);
                let reg_val = self.get_16bit_register_by_index(reg_idx);
                let res = exec_fn(self, reg_val, mem_val);
                (res, reg_idx)
            }
            0xC0..=0xFF => {
                let (low_reg, reg_idx) = self.get_index_from_c0_ff_pattern(ins);
                let reg_val = self.get_16bit_register_by_index(reg_idx % 8);
                let low_reg_val = self.get_16bit_register_by_index(low_reg % 8);
                let res = exec_fn(self, reg_val, low_reg_val);
                (res, reg_idx)
            }
        };
        if let Some(res) = res {
            self.set_16bit_register_by_index(reg_idx, res);
        }
    }
}
