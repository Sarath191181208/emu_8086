use crate::{
    consts::{Byte, Word},
    cpu::CPU,
    Memory,
};

impl CPU {
    fn mov_16bit_register_addressing(&mut self, instruction: Byte) {
        // Let's say that the instruction is MOV AX, BX
        // The instruction is 0x88 0xD8
        // The first byte is the opcode, the second byte is the operands
        // splitting the operands as the operands are 8 they repeat in a pattern
        // AX, BX, CX, DX, SP, BP, SI, DI
        // To obtain the source register we need to mask the last 4 bits of the second byte i.e 8
        // To obtain the destination register we need to mask the first 4 bits of the second byte
        let registers = [
            self.ax, self.cx, self.dx, self.bx,
            self.stack_pointer, self.base_pointer,
            self.source_index, self.destination_index
        ];

        let source_idx = instruction & 0x0F;
        let reg: Word = registers[(source_idx % 8) as usize];
        // the write index must be as follows
        // 0xC0 ... 0xC7 -> AX i.e 0
        // 0xC8 ... 0xCF -> CX i.e 1
        // 0xD0 ... 0xD7 -> DX i.e 2
        // singe the first 4 bits are C = (1100) and D = (1101) we can just shift the instruction by 4 and mask the last 2 bits
        // therefore (C or D or E or F) & 0b00110000 = 0b00110000 the last two bits correspond to the write index
        // C = ( 000 | 001 ) a, c
        // D = ( 010 | 011 ) d, b
        // E = ( 100 | 101 )
        let prefix = (instruction & 0b00110000) >> 3;
        let write_idx = if source_idx > 7 {
            prefix | 0x01
        } else {
            prefix
        };

        match write_idx {
            0x00 => self.ax = reg,
            0x01 => self.cx = reg,
            0x02 => self.dx = reg,
            0x03 => self.bx = reg,
            0x04 => self.stack_pointer = reg,
            0x05 => self.base_pointer = reg,
            0x06 => self.source_index = reg,
            0x07 => self.destination_index = reg,
            _ => panic!("Invalid register index! This can't happen!"),
        }
    }

    pub(in crate::cpu) fn execute_mov_word(&mut self, mem: &mut Memory) {
        let instruction = self.consume_instruction(mem);
        match instruction {
            0xC0..=0xFF => {
                self.mov_16bit_register_addressing(instruction);
            }
            x => panic!("MOV instruction not implemented! for {}", x),
        }
    }

    fn mov_8bit_register_addressing(&mut self, instruction: Byte){
        let registers = [
            self.get_ax_low(), self.get_cx_low(), self.get_dx_low(), self.get_bx_low(),
            self.get_ax_high(), self.get_cx_high(), self.get_dx_high(), self.get_bx_high(),
        ];

        let source_idx = instruction & 0x0F;
        let reg: Byte = registers[(source_idx % 8) as usize];

        let prefix = (instruction & 0b00110000) >> 3;
        let write_idx = if source_idx > 7 {
            prefix | 0x01
        } else {
            prefix
        };

        match write_idx {
            0x00 => self.set_ax_low(reg),
            0x01 => self.set_cx_low(reg),
            0x02 => self.set_dx_low(reg),
            0x03 => self.set_bx_low(reg),
            0x04 => self.set_ax_high(reg),
            0x05 => self.set_cx_high(reg),
            0x06 => self.set_dx_high(reg),
            0x07 => self.set_bx_high(reg),
            _ => panic!("Invalid register index! This can't happen!"),   
        }
    }

    pub(in crate::cpu) fn execute_mov_byte(&mut self, mem: &mut Memory) {
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
        mem.write_byte(0xFFFC, 0x8B);
        mem.write_byte(0xFFFD, 0xC1);
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
        // 0x8B 0xC2
        mem.write_byte(0xFFFC, 0x8B);
        mem.write_byte(0xFFFD, 0xC2);
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

        mem.write_byte(0xFFFC, 0x8B);
        mem.write_byte(0xFFFD, 0xD0);
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

        mem.write_byte(0xFFFC, 0x8B);
        mem.write_byte(0xFFFD, 0xDA);
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

        mem.write_byte(0xFFFC, 0x8B);
        mem.write_byte(0xFFFD, 0xE5);
        cpu.execute(&mut mem);
        assert_eq!(cpu.stack_pointer, cpu.base_pointer);
        assert_eq!(cpu.stack_pointer, 0xFF00);
        assert_eq!(cpu.base_pointer, 0xFF00);
    }
}

#[cfg(test)]
mod mov_8bit_register_addressing_tests{
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
        mem.write_byte(0xFFFC, 0x8A);
        mem.write_byte(0xFFFD, 0xC1);
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
        mem.write_byte(0xFFFC, 0x8A);
        mem.write_byte(0xFFFD, 0xC2);
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
        mem.write_byte(0xFFFC, 0x8A);
        mem.write_byte(0xFFFD, 0xC3);
        cpu.execute(&mut mem);
        assert_eq!(cpu.get_ax_low(), cpu.get_bx_low());
        assert_eq!(cpu.get_ax_low(), 0xFF);
        assert_eq!(cpu.get_bx_low(), 0xFF);
    }

    #[test]
    fn test_mov_8bit_register_addressing_bx_low_dx_high(){
        let mut cpu = CPU::new();
        let mut mem = Memory::new();
        cpu.reset(&mut mem);
        cpu.set_dx_high(0xFF);

        // check this operation
        // MOV BL, DH
        // 0x8A 0xF3
        mem.write_byte(0xFFFC, 0x8A);
        mem.write_byte(0xFFFD, 0xF3);
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
        mem.write_byte(0xFFFC, 0x8A);
        mem.write_byte(0xFFFD, 0xE1);
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
        mem.write_byte(0xFFFC, 0x8A);
        mem.write_byte(0xFFFD, 0xE2);
        cpu.execute(&mut mem);
        assert_eq!(cpu.get_ax_high(), cpu.get_dx_low());
    }
}
