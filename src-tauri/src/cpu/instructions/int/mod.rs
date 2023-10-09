use crate::{
    cpu::{interrupt::Interrupt, CPU},
    memory::Memory,
};

pub mod exec_interrupt;
pub mod procedures;

impl CPU {
    pub(crate) fn execute_bios_di(&mut self, mem: &mut Memory) -> Interrupt {
        self.consume_instruction(mem); // 0xFF
        self.consume_instruction(mem); // 0xCD
        let interrupt_arg = self.consume_byte(mem);

        match interrupt_arg {
            0x21 => {
                let val = self.get_dx_low() as char;
                Interrupt::Print(val.to_string())
            }

            _ => {
                panic!("Unknown interrupt: {}", interrupt_arg);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        cpu::{instructions::test_macro::compile_and_test_str, CPU},
        memory::Memory,
    };

    #[test]
    fn no_offset_indexed_add() {
        compile_and_test_str(
            "
            mov ax, 0xff

            int 0x21
",
            3,
            |cpu: &CPU, mem: &Memory| {
                // cpu.print_stack(mem);
                assert_eq!(cpu.get_code_segment(), 0xF400);
                assert_eq!(cpu.get_instruciton_pointer(), 0x204);
                assert_eq!(cpu.stack_pointer, 0xFFF8)
            },
        );
    }
}
