use crate::{consts::U20, cpu::CPU, memory::Memory};

impl CPU {
    pub(super) fn mov_16bit_register_indexed_registers_without_offset(
        &mut self,
        mem: &mut Memory,
        ins: u8,
    ) {
        let (low_reg_idx, high_reg_idx) = self.get_index_from_0x00_0x3f_pattern(ins);
        match low_reg_idx {
            0x06 => {
                // mov reg, [0x1234]
                let addr = self.consume_word(mem);
                let data = self.read_word_from_pointer(mem, addr);
                self.set_16bit_register_by_index(high_reg_idx, data);
            }
            _ => {
                let memory_offset = self.get_offset_from_index_of_indexed_registers(low_reg_idx);
                let data = self.read_word_from_u20(mem, memory_offset);
                self.set_16bit_register_by_index(high_reg_idx, data);
            }
        }
    }

    pub(super) fn mov_8bit_register_indexed_registers_without_offset(
        &mut self,
        mem: &mut Memory,
        ins: u8,
    ) {
        let (low_reg_idx, high_reg_idx) = self.get_index_from_0x00_0x3f_pattern(ins);
        match low_reg_idx {
            0x06 => {
                // mov reg, [0x1234]
                let addr = self.consume_word(mem);
                let data = self.read_byte_from_pointer(mem, addr);
                self.set_8bit_register_by_index(high_reg_idx, data);
            }
            _ => {
                let memory_offset = self.get_offset_from_index_of_indexed_registers(low_reg_idx);
                let data = self.read_byte_from_u20(mem, memory_offset);
                self.set_8bit_register_by_index(high_reg_idx, data);
            }
        }
    }

    pub(super) fn mov_16bit_register_indexed_registers_with_8bit_offset(
        &mut self,
        mem: &mut Memory,
        ins: u8,
    ) {
        let offset = U20::from(self.consume_byte(mem));
        let (low_reg_idx, high_reg_idx) = self.get_index_from_0x40_0x7f_pattern(ins);
        let memory_offset = self.get_offset_from_index_of_indexed_registers(low_reg_idx);
        let data = self.read_word_from_u20(mem, memory_offset + offset);
        self.set_16bit_register_by_index(high_reg_idx, data);
    }

    pub(super) fn mov_8bit_register_indexed_registers_with_8bit_offset(
        &mut self,
        mem: &mut Memory,
        ins: u8,
    ) {
        let offset = U20::from(self.consume_byte(mem));
        let (low_reg_idx, high_reg_idx) = self.get_index_from_0x40_0x7f_pattern(ins);
        let memory_offset = self.get_offset_from_index_of_indexed_registers(low_reg_idx);
        let data = self.read_byte_from_u20(mem, memory_offset + offset);
        self.set_8bit_register_by_index(high_reg_idx, data);
    }

    pub(super) fn mov_16bit_register_indexed_registers_with_16bit_offset(
        &mut self,
        mem: &mut Memory,
        ins: u8,
    ) {
        let offset = U20::from(self.consume_word(mem));
        let (low_reg_idx, high_reg_idx) = self.get_index_from_0x80_0xbf_pattern(ins);
        let memory_offset = self.get_offset_from_index_of_indexed_registers(low_reg_idx);
        let data = self.read_word_from_u20(mem, memory_offset + offset);
        self.set_16bit_register_by_index(high_reg_idx, data);
    }

    pub(super) fn mov_8bit_register_indexed_registers_with_16bit_offset(
        &mut self,
        mem: &mut Memory,
        ins: u8,
    ) {
        let offset = U20::from(self.consume_word(mem));
        let (low_reg_idx, high_reg_idx) = self.get_index_from_0x80_0xbf_pattern(ins);
        let memory_offset = self.get_offset_from_index_of_indexed_registers(low_reg_idx);
        let data = self.read_byte_from_u20(mem, memory_offset + offset);
        self.set_8bit_register_by_index(high_reg_idx, data);
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        cpu::{instructions::test_macro::compile_and_test_str, CPU},
        memory::Memory,
    };

    #[test]
    fn no_offset_indexed_mov() {
        compile_and_test_str(
            "
            org 100h
            .data 
            var dw 0x1234
            code: 
            mov bx, 0x100 
            mov si, 0x02
            mov ax, [bx+si]
            ",
            4,
            |cpu: &CPU, _: &Memory| {
                assert_eq!(cpu.ax, 0x1234);
            },
        );
    }

    #[test]
    fn only_offset_indexed_mov() {
        compile_and_test_str(
            " 
            mov ax, [0x1000]
            ",
            1,
            |cpu: &CPU, _: &Memory| {
                assert_eq!(cpu.ax, 0x9090);
            },
        );
    }

    #[test]
    fn offset_8bit_index_mov() {
        compile_and_test_str(
            "
            org 100h
            .data 
            _var db 0x20
            var dw 0x1234
            code: 
            mov bx, 0x100 
            mov si, 0x02
            mov ax, [bx+si+0x01]
            ",
            4,
            |cpu: &CPU, _: &Memory| {
                assert_eq!(cpu.ax, 0x1234);
            },
        );
    }

    #[test]
    fn offset_16bit_index_mov() {
        compile_and_test_str(
            "
            org 100h
            .data 
            _var dw 0x20
            var dw 0x1234
            code: 
            mov bx, 0x02
            mov si, 0x02
            mov ax, [bx+si+0x100]
            ",
            4,
            |cpu: &CPU, _: &Memory| {
                assert_eq!(cpu.ax, 0x1234);
            },
        );
    }
}

#[cfg(test)]
mod test_8bit {
    use crate::{
        cpu::{instructions::test_macro::compile_and_test_str, CPU},
        memory::Memory,
    };

    #[test]
    fn no_offset_indexed_mov() {
        compile_and_test_str(
            "
            org 100h
            .data 
            var dw 0x1234
            code: 
            mov bx, 0x100 
            mov si, 0x02
            mov al, [bx+si]
            ",
            4,
            |cpu: &CPU, _: &Memory| {
                assert_eq!(cpu.get_ax_low(), 0x34);
            },
        );
    }

    #[test]
    fn only_offset_indexed_mov() {
        compile_and_test_str(
            " 
            mov bh, [0x1000]
            ",
            1,
            |cpu: &CPU, _: &Memory| {
                assert_eq!(cpu.get_bx_high(), 0x90);
            },
        );
    }

    #[test]
    fn offset_8bit_index_mov() {
        compile_and_test_str(
            "
            org 100h
            .data 
            _var db 0x20
            var dw 0x1234
            code: 
            mov bx, 0x100 
            mov si, 0x02
            mov dh, [bx+si+0x02]
            ",
            4,
            |cpu: &CPU, _: &Memory| {
                assert_eq!(cpu.get_dx_high(), 0x12);
            },
        );
    }

    #[test]
    fn offset_16bit_index_mov() {
        compile_and_test_str(
            "
            org 100h
            .data 
            _var dw 0x20
            var dw 0x1234
            code: 
            mov bx, 0x02
            mov si, 0x02
            mov cl, [bx+si+0x100]
            ",
            4,
            |cpu: &CPU, _: &Memory| {
                assert_eq!(cpu.get_cx_low(), 0x34);
            },
        );
    }
}
