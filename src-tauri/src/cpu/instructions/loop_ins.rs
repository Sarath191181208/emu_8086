use crate::{cpu::CPU, memory::Memory};

impl CPU {
    pub(in crate::cpu) fn execute_loop_8bit(&mut self, mem: &mut Memory) {
        let offset = self.consume_instruction(mem);
        let cx = self.cx;
        if cx == 0 {
            return;
        }
        self.set_cx(cx - 1);
        match offset {
            0x80..=0xFF => {
                let offset = 0xFF - offset + 1;
                let ip = self.instruction_pointer;
                self.set_instruction_pointer(ip.wrapping_sub(offset as u16));
            }
            0x00..=0x7F => {
                let ip = self.instruction_pointer;
                self.set_instruction_pointer(ip.wrapping_add(offset as u16));
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::cpu::instructions::test_macro::run_code;

    #[test]
    fn no_offset_indexed_add() {
        let code = "
            mov cx, 0x10
            label: 
            inc ax 
            loop label
            ";
        let (cpu, _) = run_code(code, 0x30);
        assert_eq!(cpu.ax, 0x11);
    }
}
