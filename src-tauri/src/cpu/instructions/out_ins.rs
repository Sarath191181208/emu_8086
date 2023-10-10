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
    use crate::{
        cpu::{instructions::test_macro::compile_and_test_str, CPU},
        memory::Memory,
    };

    #[test]
    fn out_al_10() {
        compile_and_test_str(
            "
            mov al, 0x10
            OUT 0x80, AL",
            2,
            |cpu: &CPU, _: &Memory| {
                assert_eq!(cpu.get_port(0x80), 0x10);
            },
        );
    }

    #[test]
    fn out_ax_20() {
        compile_and_test_str(
            "
            mov ax, 0x1020
            OUT 0x80, AX",
            2,
            |cpu: &CPU, _: &Memory| {
                assert_eq!(cpu.get_port_word(0x80), 0x1020);
            },
        );
    }

    #[test]
    fn out_al_dx() {
        compile_and_test_str(
            "
            mov al, 0x10
            mov dx, 0x80
            OUT DX, AL",
            3,
            |cpu: &CPU, _: &Memory| {
                cpu.ports.print_non_empty_prots();
                assert_eq!(cpu.get_port(0x80), 0x10);
            },
        );
    }

    #[test]
    fn out_ax_dx() {
        compile_and_test_str(
            "
            mov ax, 0x1020
            mov dx, 0x80
            OUT DX, AX",
            3,
            |cpu: &CPU, _: &Memory| {
                assert_eq!(cpu.get_port_word(0x80), 0x1020);
            },
        );
    }
}
