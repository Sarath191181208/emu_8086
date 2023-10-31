use crate::{cpu::CPU, memory::Memory};

impl CPU {
    pub(in crate::cpu) fn execute_iret(&mut self, mem: &mut Memory) {
        let ip = self.pop_stack(mem);
        let cs = self.pop_stack(mem);
        let flags = self.pop_stack(mem);

        self.set_flags_from_u16(flags);
        self.set_code_segment(cs);
        self.set_instruction_pointer(ip);
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::instructions::test_macro::run_code;

    #[test]
    fn no_offset_indexed_add() {
        let code = "
            mov ax, 0xff
            int 0x21
            inc ax
";
        let (cpu, _) = run_code(code, 5);
        // cpu.print_stack(mem);
        assert_eq!(cpu.get_code_segment(), 0x100);
        assert_eq!(cpu.get_instruciton_pointer(), 0x06);
        assert_eq!(cpu.stack_pointer, 0xFFFE);
        assert_eq!(cpu.ax, 0x100);
    }
}
