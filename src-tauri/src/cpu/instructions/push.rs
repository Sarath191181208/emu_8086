use crate::{cpu::CPU, memory::Memory, consts::Byte};

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

}
