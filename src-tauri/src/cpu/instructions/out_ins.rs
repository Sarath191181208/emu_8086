use crate::{cpu::CPU, memory::Memory};

impl CPU {
    pub(in crate::cpu) fn execute_out_8bit_al(&mut self, mem: &mut Memory) {
        let port = self.consume_byte(mem);
        let val = self.get_ax_low();
        self.set_port(port, val);
    }

    pub(in crate::cpu) fn execute_out_8bit_ax(&mut self, mem: &mut Memory) {
        let port = self.consume_byte(mem);
        let val = self.ax;
        self.set_port_word(port, val);
    }

    pub(in crate::cpu) fn execute_out_dx_al(&mut self) {
        let port = (self.dx & 0xFF) as u8;
        let val = self.get_ax_low();
        self.set_port(port, val);
    }

    pub(in crate::cpu) fn execute_out_dx_ax(&mut self) {
        let port = (self.dx & 0xFF) as u8;
        let val = self.ax;
        self.set_port_word(port, val);
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::instructions::test_macro::execute_code;

    #[test]
    fn out_al_10() {
        let code = "
            mov al, 0x10
            OUT 0x80, AL";
        let (cpu, _) = execute_code(code);
        assert_eq!(cpu.get_port(0x80), 0x10);
    }

    #[test]
    fn out_ax_20() {
        let code = "
            mov ax, 0x1020
            OUT 0x80, AX";
        let (cpu, _) = execute_code(code);
        assert_eq!(cpu.get_port_word(0x80), 0x1020);
    }

    #[test]
    fn out_al_dx() {
        let code = "
            mov al, 0x10
            mov dx, 0x80
            OUT DX, AL";
        let (cpu, _) = execute_code(code);
        assert_eq!(cpu.get_port(0x80), 0x10);
    }

    #[test]
    fn out_ax_dx() {
        let code = "
            mov ax, 0x1020
            mov dx, 0x80
            OUT DX, AX";
        let (cpu, _) = execute_code(code);
        assert_eq!(cpu.get_port_word(0x80), 0x1020);
    }
}
