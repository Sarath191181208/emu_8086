use crate::{cpu::CPU, memory::Memory, consts::{Byte, U20}};

impl CPU {
    pub(in crate::cpu) fn execute_push_es(&mut self, mem: &mut Memory) {
        let val = self.extra_segment;
        self.push_stack(mem, val);
    }
    pub(in crate::cpu) fn execute_push_cs(&mut self, mem: &mut Memory) {
        let value = self.code_segment;
        self.push_stack(mem, value);
    }
    pub(in crate::cpu) fn execute_push_ss(&mut self, mem: &mut Memory) {
        let value = self.stack_segment;
        self.push_stack(mem, value);
    }
    pub(in crate::cpu) fn execute_push_ds(&mut self, mem: &mut Memory) {
        let value = self.data_segment;
        self.push_stack(mem, value);
    }

    pub(in crate::cpu) fn execute_push_word_register(&mut self, mem: &mut Memory, ins: Byte){
        let instruction_byte_of_push_ax = 0x50;
        let value = self.get_16bit_register_by_index(ins - instruction_byte_of_push_ax);
        self.push_stack(mem, value);
    }

    
    pub(in crate::cpu) fn execute_push_indexed_addressing_no_offset (&mut self, mem: &mut Memory){
        let type_of_idx_addressing = self.consume_instruction(mem);
        match type_of_idx_addressing{
            0x36 => { // i.e pointer addressing 
                let address = self.consume_word(mem);
                let value = self.read_word_from_pointer(mem, address);
                self.push_stack(mem, value);
            },
            0x30..=0x37 => { // i.e indexed addressing
                let reg_idx = type_of_idx_addressing - 0x30;
                let offset = self.get_offset_from_index_of_indexed_registers(reg_idx);
                let value = self.read_word_from_u20(mem, offset);
                self.push_stack(mem, value);
            }
            _ => panic!("Invalid instruction byte for push indexed addressing without offset")
        }
    }

    pub(in crate::cpu) fn execute_push_indexed_addressing_with_8bit_offset (&mut self, mem: &mut Memory){
        let type_of_idx_addressing = self.consume_instruction(mem);
        
        match type_of_idx_addressing{
            0x70..=0x77 => {
                let u8_offset = self.consume_instruction(mem);
                let reg_idx = type_of_idx_addressing - 0x70;
                let offset = self.get_offset_from_index_of_indexed_registers(reg_idx);
                let offset = offset + U20::from(u8_offset);
                let value = self.read_word_from_u20(mem, offset);
                self.push_stack(mem, value);
            }
            _ => panic!("Invalid instruction byte for push indexed addressing with 8bit offset")
        }
    }

}

#[cfg(test)]
mod test {
    use crate::{
        cpu::{instructions::test_macro::compile_and_test_str, CPU},
        memory::Memory,
    };

    macro_rules! single_segment_push_fixture {
        ($segment_name: ident) => {
        paste::item! {

            #[test]
            fn [<push_ $segment_name>](){
                compile_and_test_str(
                    &format!(
                        "
                        org 100h 
                        .data 
                        var dw 0x1000 
                        code: 
                        push {}
                        ",
                        stringify!($segment_name)
                    ),
                    2,
                    |cpu: &CPU, mem: &Memory| {
                        assert_eq!(cpu.stack_pointer, 0xFFFC);
                        assert_eq!(cpu.read_word_from_pointer(mem, 0xFFFC), 0x0700);
                    },
                );
            }
        }
    }
}

    single_segment_push_fixture!(cs);
    single_segment_push_fixture!(ds);
    single_segment_push_fixture!(ss);
    single_segment_push_fixture!(es);

    #[test]
    fn push_bp() {
        compile_and_test_str(
            "
            org 100h 
            .data 
            var dw 0x1000 
            code: 
            mov bp, 0x101
            push bp
            ",
            3,
            |cpu: &CPU, mem: &Memory| {
                // cpu.print_stack(mem);
                assert_eq!(cpu.stack_pointer, 0xFFFC);
                assert_eq!(cpu.read_word_from_pointer(mem, 0xFFFC), 0x101);
            },
        );
    }

    #[test]
    fn push_var() {
        compile_and_test_str(
            "
            org 100h 
            .data 
            var dw 0x101 
            code: 
            push var
            ",
            3,
            |cpu: &CPU, mem: &Memory| {
                // cpu.print_stack(mem);
                assert_eq!(cpu.stack_pointer, 0xFFFC);
                assert_eq!(cpu.read_word_from_pointer(mem, 0xFFFC), 0x101);
            },
        );
    }

    #[test]
    fn push_var_with_offset() {
        compile_and_test_str(
            "
            org 100h
            .data
            var dw 0x101
            code:
            mov bx, 0x102
            push [bx]
            ",
            3,
            |cpu: &CPU, mem: &Memory| {
                // cpu.print_stack(mem);
                assert_eq!(cpu.stack_pointer, 0xFFFC);
                assert_eq!(cpu.read_word_from_pointer(mem, 0xFFFC), 0x101);
            },
        );
    }

    #[test]
    fn push_var_with_8bit_offset() {
        compile_and_test_str(
            "
            org 100h
            .data
            var dw 0x101
            code:
            mov bx, 0x102
            push [bx + 1]
            ",
            3,
            |cpu: &CPU, mem: &Memory| {
                // cpu.print_stack(mem);
                assert_eq!(cpu.stack_pointer, 0xFFFC);
                assert_eq!(cpu.read_word_from_pointer(mem, 0xFFFC), 0xBB01);
            },
        );
    }

}
