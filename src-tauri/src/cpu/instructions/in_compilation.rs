use crate::{cpu::CPU, memory::Memory};

impl CPU {
    pub(in crate::cpu) fn execute_in_al_8bit(&mut self, mem: &mut Memory) {
        let port = self.consume_byte(mem);
        let port_val = self.get_port(port);
        self.set_ax_low(port_val);
    }

    pub(in crate::cpu) fn execute_in_ax_8bit(&mut self, mem: &mut Memory) {
        let port = self.consume_byte(mem);
        let port_val = self.get_port_word(port);
        self.set_ax(port_val);
    }

    pub(in crate::cpu) fn execute_in_al_dx(&mut self) {
        let port = (self.dx & 0xFF) as u8;
        let port_val = self.get_port(port);
        self.set_ax_low(port_val);
    }

    pub(in crate::cpu) fn execute_in_ax_dx(&mut self) {
        let port = (self.dx & 0xFF) as u8;
        let port_val = self.get_port_word(port);
        self.set_ax(port_val);
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
            MOV AL, 0x10
            IN AL, 0x80",
            2,
            |cpu: &CPU, _: &Memory| {
                assert_eq!(cpu.ax, 0);
            },
        );
    }

    #[test]
    fn execute_in_ax_8bit() {
        compile_and_test_str(
            "
            MOV AX, 0x1010
            IN AX, 0x80",
            2,
            |cpu: &CPU, _: &Memory| {
                assert_eq!(cpu.ax, 0);
            },
        );
    }

    #[test]
    fn execute_in_al_dx() {
        compile_and_test_str(
            "
            MOV DX, 0x80
            MOV AL, 0x10
            IN AL, DX",
            3,
            |cpu: &CPU, _: &Memory| {
                assert_eq!(cpu.ax, 0);
            },
        );
    }

    #[test]
    fn execute_in_ax_dx() {
        compile_and_test_str(
            "
            MOV DX, 0x80
            MOV AX, 0x1010
            IN AX, DX",
            3,
            |cpu: &CPU, _: &Memory| {
                assert_eq!(cpu.ax, 0);
            },
        );
    }
}