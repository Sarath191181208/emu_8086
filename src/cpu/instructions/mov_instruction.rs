use crate::{Memory, cpu::CPU, consts::{Byte, Word}};

impl CPU{

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
        
        let reg: Word = registers[((instruction & 0x0F) % 8)as usize];
        // the write index must be as follows 
        // 0xC0 ... 0xC7 -> AX i.e 0
        // 0xC8 ... 0xCF -> CX i.e 1
        // 0xD0 ... 0xD7 -> DX i.e 2
        let write_idx = match instruction > 8 {
            true =>  (instruction & 0b00110000) >> 4 + 1,
            false => (instruction & 0b00110000) >> 4, 
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

    pub(crate) fn execute_mov(&mut self, mem: &mut Memory) {
        let instruction = self.consume_instruction(mem);
        match instruction{
            0xC0 ..= 0xFF => {
                self.mov_16bit_register_addressing(instruction);
            },
            x => panic!("MOV instruction not implemented! for {}", x),
        }
    }

}