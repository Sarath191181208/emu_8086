use crate::{
    consts::{Byte, U20},
    cpu::CPU,
    memory::Memory,
};

impl CPU {
    pub(in crate::cpu) fn execute_pop_es(&mut self, mem: &mut Memory) {
        let val = self.pop_stack(mem);
        self.extra_segment = val;
    }
    pub(in crate::cpu) fn execute_pop_ss(&mut self, mem: &mut Memory) {
        let val = self.pop_stack(mem);
        self.stack_segment = val;
    }
    pub(in crate::cpu) fn execute_pop_ds(&mut self, mem: &mut Memory) {
        let val = self.pop_stack(mem);
        self.data_segment = val;
    }

    pub(in crate::cpu) fn execute_pop_word_register(&mut self, mem: &mut Memory, ins: Byte) {
        let stack_val = self.pop_stack(mem);
        let instruction_byte_of_pop_ax = 0x58;
        let idx = ins - instruction_byte_of_pop_ax;
        self.set_16bit_register_by_index(idx, stack_val);
    }

    pub(in crate::cpu) fn execute_pop_indexed_addressing_no_offset(&mut self, mem: &mut Memory) {
        let type_of_idx_addressing = self.consume_instruction(mem);
        match type_of_idx_addressing {
            0x06 => {
                // i.e pointer addressing
                let address = self.consume_word(mem);
                let stack_val = self.pop_stack(mem);
                self.write_word_from_pointer(mem, address, stack_val);
            }
            0x00..=0x07 => {
                // i.e indexed addressing
                let reg_idx = type_of_idx_addressing;
                let offset = self.get_offset_from_index_of_indexed_registers(reg_idx);
                let stack_val = self.pop_stack(mem);
                self.write_word_to_u20(mem, offset, stack_val);
            }
            _ => panic!("Invalid instruction byte for pop indexed addressing without offset"),
        }
    }

    pub(in crate::cpu) fn execute_pop_indexed_addressing_with_8bit_offset(
        &mut self,
        mem: &mut Memory,
    ) {
        let type_of_idx_addressing = self.consume_instruction(mem);

        match type_of_idx_addressing {
            0x40..=0x47 => {
                let u8_offset = self.consume_instruction(mem);
                let reg_idx = type_of_idx_addressing - 0x40;
                let offset = self.get_offset_from_index_of_indexed_registers(reg_idx);
                let offset = offset + U20::from(u8_offset);
                let stack_val = self.pop_stack(mem);
                self.write_word_to_u20(mem, offset, stack_val);
            }
            _ => panic!("Invalid instruction byte for pop indexed addressing with 8bit offset"),
        }
    }

    pub(in crate::cpu) fn execute_pop_indexed_addressing_with_16bit_offset(
        &mut self,
        mem: &mut Memory,
    ) {
        let type_of_idx_addressing = self.consume_instruction(mem);

        match type_of_idx_addressing {
            0x80..=0x87 => {
                let reg_idx = type_of_idx_addressing - 0x80;
                let u16_offset = self.consume_word(mem);
                let offset = self.get_offset_from_index_of_indexed_registers(reg_idx);
                let offset = offset + U20::from(u16_offset);
                let stack_val = self.pop_stack(mem);
                self.write_word_to_u20(mem, offset, stack_val);
            }
            _ => panic!("Invalid instruction byte for pop indexed addressing with 8bit offset"),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        cpu::{instructions::test_macro::compile_and_test_str, CPU},
        memory::Memory,
    };

    #[test]
    fn test_pop_16bit_reg_and_segments() {
        compile_and_test_str(
            "
            org 100h
            code:

            push 0x109
            push 0x108
            push 0x107

            push 0x106
            push 0x105
            push 0x104
            push 0x103 
            push 0x102
            push 0x101
            push 0x100

            pop ax
            pop bx
            pop cx
            pop dx
            pop bp
            pop si 
            pop di 

            pop es
            pop ss
            pop ds
            ",
            22,
            |cpu: &CPU, _: &Memory| {
                assert_eq!(cpu.ax, 0x100);
                assert_eq!(cpu.bx, 0x101);
                assert_eq!(cpu.cx, 0x102);
                assert_eq!(cpu.dx, 0x103);
                assert_eq!(cpu.base_pointer, 0x104);
                assert_eq!(cpu.source_index, 0x105);
                assert_eq!(cpu.destination_index, 0x106);

                assert_eq!(cpu.extra_segment, 0x107);
                assert_eq!(cpu.stack_segment, 0x108);
                assert_eq!(cpu.data_segment, 0x109);
            },
        );
    }

    #[test]
    fn pop_indexed_addressing() {
        compile_and_test_str(
            "
            org 100h
            .data
            var  dw 0x1234
            var2 dw 0x2345
            var3 dw 0x3456
            var4 dw 0x4567

            code:
            push var
            push var2
            push var3
            push var4

            mov bx, 0x100
            mov si, 0x02
            pop [bx+si] ; var1 = var4

            mov bx, 0x100
            pop [bx + 0x04] ; var2 = var3

            mov bx, 0x100
            pop var3 ; var3 = var2

            mov bx, 0x08
            pop [bx + 0x100] ; var4 = var1
            ",
            22,
            |cpu: &CPU, mem: &Memory| {
                assert_eq!(cpu.read_word_from_pointer(mem, 0x102), 0x4567);
                assert_eq!(cpu.read_word_from_pointer(mem, 0x104), 0x3456);
                assert_eq!(cpu.read_word_from_pointer(mem, 0x106), 0x2345);
                assert_eq!(cpu.read_word_from_pointer(mem, 0x108), 0x1234);
            },
        );
    }
}
