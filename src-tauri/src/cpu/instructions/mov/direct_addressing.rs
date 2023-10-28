use crate::{cpu::CPU, memory::Memory};

impl CPU {
    pub(in crate::cpu) fn execute_mov_ax_direct_addressing(&mut self, mem: &mut Memory) {
        let addr = self.consume_word(mem);
        self.ax = mem.read_word(self.data_segment, addr);
    }

    pub(in crate::cpu) fn execute_mov_al_direct_addressing(&mut self, mem: &mut Memory) {
        let addr = self.consume_word(mem);
        let data = mem.read_byte(self.data_segment, addr);
        self.set_ax_low(data);
    }

    pub(in crate::cpu) fn execute_mov_direct_addressing_ax(&mut self, mem: &mut Memory) {
        let addr = self.consume_word(mem);
        mem.write_word(self.data_segment, addr, self.ax);
    }

    pub(in crate::cpu) fn execute_mov_direct_addressing_al(&mut self, mem: &mut Memory) {
        let addr = self.consume_word(mem);
        mem.write_byte(self.data_segment, addr, self.get_ax_low());
    }

    pub(in crate::cpu) fn execute_mov_direct_addressing_immediate_word(
        &mut self,
        mem: &mut Memory,
    ) {
        let addr = self.consume_word(mem);
        let data = self.consume_word(mem);
        mem.write_word(self.data_segment, addr, data);
    }

    pub(in crate::cpu) fn execute_mov_direct_addressing_immediate_byte(
        &mut self,
        mem: &mut Memory,
    ) {
        let addr = self.consume_word(mem);
        let data = self.consume_instruction(mem);
        mem.write_byte(self.data_segment, addr, data);
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::instructions::test_macro::run_code;

    #[test]
    fn mov_ax_var() {
        let code = "
            org 100h
            .data 
            var dw 0x1234
            code: 
            mov ax, var
            ";
        let (cpu, _) = run_code(code, 2);
        assert_eq!(cpu.ax, 0x1234);
    }
}
