use crate::{
    consts::{Word, U20},
    cpu::CPU,
    memory::Memory,
    utils::Either,
};

#[macro_export]
macro_rules! generate_16bit_reg_8bit_reg_indexed_and_byte_indexed_addressing_as_first_ins_methods {
    ($ins_name: ident, $exec_fn_16bit: expr, $exec_fn_8bit: expr) => {
        paste::item!(
            pub(in $crate::cpu) fn [<execute_ $ins_name _16bit_reg>](&mut self, mem: &mut Memory) {
                self.consume_bytes_and_parse_16bit_reg_as_first_arg_double_ins(
                    mem,
                    $exec_fn_16bit,
                )
            }

            pub(in $crate::cpu) fn [<execute_ $ins_name _8bit_reg>](&mut self, mem: &mut Memory) {
                self.consume_bytes_and_parse_8bit_reg_as_first_arg_double_ins(
                    mem,
                    $exec_fn_8bit,
                );
            }

            pub(in $crate::cpu) fn [<execute_ $ins_name _word_addr_as_first_operand>](
                &mut self,
                mem: &mut Memory,
            ) {
                self.consume_bytes_and_parse_mem_as_first_arg_double_ins(
                    mem,
                    $exec_fn_16bit,
                );
            }

            pub(in $crate::cpu) fn [<execute_ $ins_name _byte_addr_as_first_operand>](
                &mut self,
                mem: &mut Memory,
            ) {
                self.consume_bytes_and_parse_byte_mem_as_first_arg_double_ins(
                    mem,
                    $exec_fn_8bit,
                );
            }
        );
    };
}

#[macro_export]
macro_rules! generate_ins_al_and_num {
    ($ins_name: ident, $exec_fn: expr) => {
        paste::item!(
            pub(in $crate::cpu) fn [<$ins_name _al_in_immediate_addressing>](&mut self, mem: &mut Memory) {
                let val = self.consume_byte(mem);
                let exec_fn: &dyn Fn(&mut CPU, u8, u8) -> Option<u8> = $exec_fn;
                let res = exec_fn(self, self.get_ax_low(), val);
                if let Some(res) = res {
                    self.set_ax_low(res);
                }
            }
        );
    };
}

#[macro_export]
macro_rules! generate_ins_ax_and_num {
    ($ins_name: ident, $exec_fn: expr) => {
        paste::item! (
            pub(in $crate::cpu) fn [<$ins_name _ax_in_immediate_addressing>](&mut self, mem: &mut Memory) {
                let val = self.consume_word(mem);
                let exec_fn: &dyn Fn(&mut CPU, u16, u16) -> Option<u16> = $exec_fn;
                let res = exec_fn(self, self.ax, val);
                if let Some(res) = res {
                    self.set_ax(res);
                }
            }
        );
    };
}

#[macro_export]
macro_rules! generate_execute_ins_16bit_reg_and_number {
    ($ins_name: ident, $byte_val: expr, $exec_fn: expr) => {
        // $byte_val is the byte value of the instruction if it has 8-bit and 16-bit version, ex: 0x83
        // $reg_idx_offset is the offset of the register index in the instruction ex: 0xC0
        paste::item! (
            pub(in $crate::cpu) fn [< execute_ $ins_name _16bit_reg_and_number>](&mut self, mem: &mut Memory, ins: u8) {
                let is_num_u8 = ins == $byte_val;
                let ins = self.consume_instruction(mem);
                let reg_idx = ins%8;
                let num = if is_num_u8 {
                    self.consume_byte(mem) as u16
                } else {
                    self.consume_word(mem)
                };
                let reg_val = self.get_16bit_register_by_index(reg_idx);
                let exec_fn: &dyn Fn(&mut CPU, u16, u16) -> Option<u16> = $exec_fn;
                let res = exec_fn(self, reg_val, num);
                if let Some(res) = res {
                    self.set_16bit_register_by_index(reg_idx, res);
                }
            }
        );
    };
}

#[macro_export]
macro_rules! generate_execute_ins_8bit_reg_and_number {
    ($ins_name: ident, $exec_fn: expr) => {
        paste::item! (
            pub(in $crate::cpu) fn [<execute_ $ins_name _8bit_reg_and_number>](&mut self, mem: &mut Memory) {
                let ins = self.consume_instruction(mem);
                let reg_idx = ins % 8;
                let num = self.consume_byte(mem);
                let reg_val = self.get_8bit_register_by_index(reg_idx);
                let exec_fn: &dyn Fn(&mut CPU, u8, u8) -> Option<u8> = $exec_fn;
                let res = exec_fn(self, reg_val, num);
                if let Some(res) = res {
                    self.set_8bit_register_by_index(reg_idx, res);
                }
            }
        );
    };
}

#[macro_export]
macro_rules! generate_execute_ins_word_addr_and_number {
    ($ins_name: ident, $byte_val: expr, $exec_fn: expr) => {
        paste::item!(
            pub(in $crate::cpu) fn [<execute_ $ins_name _word_addr_and_number>](&mut self, mem: &mut Memory, ins: u8) {
                let is_num_u8 = ins == $byte_val;
                self.consume_instruction(mem); // 0x26
                let addr = self.consume_word(mem);
                let addr_val = self.read_word_from_pointer(mem, addr);
                let num = if is_num_u8 {
                    self.consume_byte(mem) as u16
                } else {
                    self.consume_word(mem)
                };
                let exec_fn: &dyn Fn(&mut CPU, u16, u16) -> Option<u16> = $exec_fn;
                let res = exec_fn(self, addr_val, num);
                if let Some(res) = res {
                    self.write_word_from_pointer(mem, addr, res);
                }
            }
        );
    };
}

#[macro_export]
macro_rules! generate_execute_ins_byte_addr_and_number {
    ($ins_name: ident, $exec_fn: expr) => {
        paste::item!(
            pub(in $crate::cpu) fn [< execute_ $ins_name _byte_addr_and_number>](&mut self, mem: &mut Memory) {
                self.consume_instruction(mem); // 0x26
                let addr = self.consume_word(mem);
                let addr_val = self.read_byte_from_pointer(mem, addr);
                let num = self.consume_byte(mem);
                let exec_fn: &dyn Fn(&mut CPU, u8, u8) -> Option<u8> = $exec_fn;
                let res = exec_fn(self, addr_val, num);
                if let Some(res) = res {
                    self.write_byte_from_pointer(mem, addr, res);
                }
            }
        );
    };
}

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

    // pub(super) fn set_auxillary_flag_from_nums(&mut self, val1: u8, val2: u8) {
    //     self.auxiliary_carry_flag = (val1 & 0x0F) + (val2 & 0x0F) > 0x0F;
    // }

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

    pub(super) fn consume_bytes_and_parse_double_ins(
        &mut self,
        mem: &mut Memory,
    ) -> AddressingMode {
        let ins = self.consume_instruction(mem);
        match ins {
            0x06 | 0x0E | 0x16 | 0x1E | 0x26 | 0x2E | 0x36 | 0x3E => {
                let reg_idx = ins >> 3;
                let addr = self.consume_word(mem);
                let offset = U20::from(addr);
                let offset = offset + U20::from(self.data_segment * 0x10);
                AddressingMode::Address(reg_idx, offset)
            }
            0x00..=0x3F => {
                let (indexed_addr_idx, reg_idx) = self.get_index_from_0x00_0x3f_pattern(ins);
                let mem_addr = self.get_offset_from_index_of_indexed_registers(indexed_addr_idx);
                AddressingMode::Address(reg_idx, mem_addr)
            }
            0x40..=0x7F => {
                let (indexed_addr_idx, reg_idx) = self.get_index_from_0x40_0x7f_pattern(ins);
                let mem_addr = self.consume_byte_and_get_cummulative_offset(mem, indexed_addr_idx);
                AddressingMode::Address(reg_idx, mem_addr)
            }
            0x80..=0xBF => {
                let (indexed_addr_idx, reg_idx) = self.get_index_from_0x80_0xbf_pattern(ins);
                let mem_addr = self.consume_word_and_get_cummulative_offset(mem, indexed_addr_idx);
                AddressingMode::Address(reg_idx, mem_addr)
            }
            0xC0..=0xFF => {
                let (low_reg, reg_idx) = self.get_index_from_c0_ff_pattern(ins);
                AddressingMode::Reg(reg_idx, low_reg)
            }
        }
    }

    pub(super) fn consume_bytes_and_parse_8bit_reg_as_first_arg_double_ins(
        &mut self,
        mem: &mut Memory,
        exec_fn: &dyn Fn(&mut CPU, u8, u8) -> Option<u8>,
    ) {
        let (reg_idx, res) = match self.consume_bytes_and_parse_double_ins(mem) {
            AddressingMode::Address(reg_idx, addr) => {
                let val = self.read_byte_from_u20(mem, addr);
                let reg_val = self.get_8bit_register_by_index(reg_idx);
                let res = exec_fn(self, reg_val, val);
                (reg_idx, res)
            }
            AddressingMode::Reg(reg_idx, low_reg) => {
                let reg_val = self.get_8bit_register_by_index(reg_idx % 8);
                let low_reg_val = self.get_8bit_register_by_index(low_reg % 8);
                let res = exec_fn(self, reg_val, low_reg_val);
                (reg_idx, res)
            }
        };

        if let Some(res) = res {
            self.set_8bit_register_by_index(reg_idx, res);
        }
    }

    pub(super) fn consume_bytes_and_parse_byte_mem_as_first_arg_double_ins(
        &mut self,
        mem: &mut Memory,
        exec_fn: &dyn Fn(&mut CPU, u8, u8) -> Option<u8>,
    ) {
        match self.consume_bytes_and_parse_double_ins(mem) {
            AddressingMode::Address(reg_idx, addr) => {
                let val = self.read_byte_from_u20(mem, addr.clone());
                let reg_val = self.get_8bit_register_by_index(reg_idx);
                let res = exec_fn(self, val, reg_val);
                if let Some(res) = res {
                    self.write_byte_to_u20(mem, addr, res);
                }
            }
            AddressingMode::Reg(reg_idx, low_reg) => {
                let reg_val = self.get_8bit_register_by_index(reg_idx % 8);
                let low_reg_val = self.get_8bit_register_by_index(low_reg % 8);
                let res = exec_fn(self, reg_val, low_reg_val);
                if let Some(res) = res {
                    self.set_8bit_register_by_index(reg_idx, res);
                }
            }
        };
    }

    pub(super) fn consume_bytes_and_parse_16bit_reg_as_first_arg_double_ins(
        &mut self,
        mem: &mut Memory,
        exec_fn: &dyn Fn(&mut CPU, u16, u16) -> Option<u16>,
    ) {
        let (reg_idx, res) = match self.consume_bytes_and_parse_double_ins(mem) {
            AddressingMode::Address(reg_idx, addr) => {
                let val = self.read_word_from_u20(mem, addr);
                let reg_val = self.get_16bit_register_by_index(reg_idx);
                let res = exec_fn(self, reg_val, val);
                (reg_idx, res)
            }
            AddressingMode::Reg(reg_idx, low_reg) => {
                let reg_val = self.get_16bit_register_by_index(reg_idx % 8);
                let low_reg_val = self.get_16bit_register_by_index(low_reg % 8);
                let res = exec_fn(self, reg_val, low_reg_val);
                (reg_idx, res)
            }
        };

        if let Some(res) = res {
            self.set_16bit_register_by_index(reg_idx, res);
        }
    }

    pub(super) fn consume_bytes_and_parse_mem_as_first_arg_double_ins(
        &mut self,
        mem: &mut Memory,
        exec_fn: &dyn Fn(&mut CPU, u16, u16) -> Option<u16>,
    ) {
        match self.consume_bytes_and_parse_double_ins(mem) {
            AddressingMode::Address(reg_idx, addr) => {
                let val = self.read_word_from_u20(mem, addr.clone());
                let reg_val = self.get_16bit_register_by_index(reg_idx);
                let res = exec_fn(self, val, reg_val);
                if let Some(res) = res {
                    self.write_word_to_u20(mem, addr, res);
                }
            }
            AddressingMode::Reg(reg_idx, low_reg) => {
                let reg_val = self.get_16bit_register_by_index(reg_idx % 8);
                let low_reg_val = self.get_16bit_register_by_index(low_reg % 8);
                let res = exec_fn(self, reg_val, low_reg_val);
                if let Some(res) = res {
                    self.set_16bit_register_by_index(reg_idx, res);
                }
            }
        };
    }
}

pub(crate) enum AddressingMode {
    Address(u8, U20),
    Reg(u8, u8),
}
