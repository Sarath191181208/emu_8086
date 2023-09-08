use crate::{consts::Byte, cpu::CPU, Memory};

impl CPU {
    fn mov_16bit_register_addressing(&mut self, instruction: Byte) {
        let (source_index, write_index) = self.get_index_from_c0_ff_pattern(instruction);
        let reg = self.get_16bit_register_by_index(source_index % 8);
        self.set_16bit_register_by_index(write_index, reg);
    }

    pub(in crate::cpu) fn execute_mov_register_word(&mut self, mem: &mut Memory) {
        let instruction = self.consume_instruction(mem);
        match instruction {
            0xC0..=0xFF => {
                self.mov_16bit_register_addressing(instruction);
            }
            x => panic!("MOV instruction not implemented! for {}", x),
        }
    }

    fn mov_8bit_register_addressing(&mut self, instruction: Byte) {
        let (source_index, write_index) = self.get_index_from_c0_ff_pattern(instruction);
        let reg = self.get_8bit_register_by_index(source_index % 8);
        self.set_8bit_register_by_index(write_index, reg);
    }

    pub(in crate::cpu) fn execute_mov_register_byte(&mut self, mem: &mut Memory) {
        let instruction = self.consume_instruction(mem);
        match instruction {
            0xC0..=0xFF => {
                self.mov_8bit_register_addressing(instruction);
            }
            x => panic!("MOV instruction not implemented! for {}", x),
        }
    }
}

#[cfg(test)]
mod mov_16bit_register_addressing_tests {
    use super::CPU;
    use crate::memory::Memory;

    #[test]
    fn test_mov_16bit_register_addressing_ax_cx() {
        let mut cpu = CPU::new();
        let mut mem = Memory::new();
        cpu.reset(&mut mem);
        cpu.cx = 0xFF00;

        // check this operation
        // MOV AX, CX
        // 0x8B 0xC1
        cpu.write_instructions(&mut mem, &[0x8B, 0xC1]);

        cpu.execute(&mut mem);
        assert_eq!(cpu.ax, cpu.cx);
        assert_eq!(cpu.ax, 0xFF00);
        assert_eq!(cpu.cx, 0xFF00);
    }

    #[test]
    fn test_mov_16bit_register_addressing_ax_dx() {
        let mut cpu = CPU::new();
        let mut mem = Memory::new();
        cpu.reset(&mut mem);
        cpu.dx = 0xFF00;
        // check this operation
        // MOV AX, DX
        cpu.write_instructions(&mut mem, &[0x8B, 0xC2]);
        cpu.execute(&mut mem);
        assert_eq!(cpu.ax, cpu.dx);
        assert_eq!(cpu.ax, 0xFF00);
        assert_eq!(cpu.dx, 0xFF00);
    }

    #[test]
    fn test_mov_16bit_register_addressing_dx_ax() {
        let mut cpu = CPU::new();
        let mut mem = Memory::new();
        cpu.reset(&mut mem);
        cpu.ax = 0xFF00;

        // check this operation
        // MOV DX, AX
        // 0x8B 0xD0

        cpu.write_instructions(&mut mem, &[0x8B, 0xD0]);
        cpu.execute(&mut mem);
        assert_eq!(cpu.dx, cpu.ax);
        assert_eq!(cpu.dx, 0xFF00);
        assert_eq!(cpu.ax, 0xFF00);
    }

    #[test]
    fn test_mov_16bit_register_addressing_bx_dx() {
        let mut cpu = CPU::new();
        let mut mem = Memory::new();
        cpu.reset(&mut mem);
        cpu.dx = 0xFF00;

        // check this operation
        // MOV BX, DX
        // 0x8B 0xDA

        cpu.write_instructions(&mut mem, &[0x8B, 0xDA]);
        cpu.execute(&mut mem);
        assert_eq!(cpu.bx, cpu.dx);
        assert_eq!(cpu.bx, 0xFF00);
        assert_eq!(cpu.dx, 0xFF00);
    }

    #[test]
    fn test_mov_16bit_register_addressing_sp_bp() {
        let mut cpu = CPU::new();
        let mut mem = Memory::new();
        cpu.reset(&mut mem);
        cpu.base_pointer = 0xFF00;

        // check this operation
        // MOV SP, BP
        // 0x8B 0xE5

        cpu.write_instructions(&mut mem, &[0x8B, 0xE5]);
        cpu.execute(&mut mem);
        assert_eq!(cpu.stack_pointer, cpu.base_pointer);
        assert_eq!(cpu.stack_pointer, 0xFF00);
        assert_eq!(cpu.base_pointer, 0xFF00);
    }
}

#[cfg(test)]
mod mov_8bit_register_addressing_tests {
    use super::CPU;
    use crate::memory::Memory;

    #[test]
    fn test_mov_8bit_register_addressing_ax_low_cx_low() {
        let mut cpu = CPU::new();
        let mut mem = Memory::new();
        cpu.reset(&mut mem);
        cpu.set_cx_low(0xFF);

        // check this operation
        // MOV AL, CL
        // 0x8A 0xC1
        cpu.write_instructions(&mut mem, &[0x8A, 0xC1]);
        cpu.execute(&mut mem);
        assert_eq!(cpu.get_ax_low(), cpu.get_cx_low());
        assert_eq!(cpu.get_ax_low(), 0xFF);
        assert_eq!(cpu.get_cx_low(), 0xFF);
    }

    #[test]
    fn test_mov_8bit_register_addressing_ax_low_dx_low() {
        let mut cpu = CPU::new();
        let mut mem = Memory::new();
        cpu.reset(&mut mem);
        cpu.set_dx_low(0xFF);

        // check this operation
        // MOV AL, DL
        // 0x8A 0xC2
        cpu.write_instructions(&mut mem, &[0x8A, 0xC2]);
        cpu.execute(&mut mem);
        assert_eq!(cpu.get_ax_low(), cpu.get_dx_low());
        assert_eq!(cpu.get_ax_low(), 0xFF);
        assert_eq!(cpu.get_dx_low(), 0xFF);
    }

    #[test]
    fn test_mov_8bit_register_addressing_ax_low_bx_low() {
        let mut cpu = CPU::new();
        let mut mem = Memory::new();
        cpu.reset(&mut mem);
        cpu.set_bx_low(0xFF);

        // check this operation
        // MOV AL, BL
        // 0x8A 0xC3
        cpu.write_instructions(&mut mem, &[0x8A, 0xC3]);
        cpu.execute(&mut mem);
        assert_eq!(cpu.get_ax_low(), cpu.get_bx_low());
        assert_eq!(cpu.get_ax_low(), 0xFF);
        assert_eq!(cpu.get_bx_low(), 0xFF);
    }

    #[test]
    fn test_mov_8bit_register_addressing_bx_low_dx_high() {
        let mut cpu = CPU::new();
        let mut mem = Memory::new();
        cpu.reset(&mut mem);
        cpu.set_dx_high(0xFF);

        // check this operation
        // MOV BL, DH
        // 0x8A 0xF3
        cpu.write_instructions(&mut mem, &[0x8A, 0xDE]);
        cpu.execute(&mut mem);
        assert_eq!(cpu.get_bx_low(), cpu.get_dx_high());
        assert_eq!(cpu.get_bx_low(), 0xFF);
        assert_eq!(cpu.get_dx_high(), 0xFF);
    }

    #[test]
    fn test_mov_8bit_register_addressing_ax_high_cx_high() {
        let mut cpu = CPU::new();
        let mut mem = Memory::new();
        cpu.reset(&mut mem);
        cpu.set_cx_high(0xFF);

        // check this operation
        // MOV AH, CH
        // 0x8A 0xE1
        cpu.write_instructions(&mut mem, &[0x8A, 0xE5]);
        cpu.execute(&mut mem);
        assert_eq!(cpu.get_ax_high(), cpu.get_cx_high());
        assert_eq!(cpu.get_ax_high(), 0xFF);
        assert_eq!(cpu.get_cx_high(), 0xFF);
    }

    #[test]
    fn test_mov_8bit_register_addressing_ax_high_dx_low() {
        let mut cpu = CPU::new();
        let mut mem = Memory::new();

        cpu.reset(&mut mem);
        cpu.set_dx_low(0xFF);

        // check this operation
        // MOV AH, DL
        // 0x8A 0xE2
        cpu.write_instructions(&mut mem, &[0x8A, 0xE2]);
        cpu.execute(&mut mem);
        assert_eq!(cpu.get_ax_high(), cpu.get_dx_low());
    }
}
