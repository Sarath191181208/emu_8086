use crate::{cpu::CPU, memory::Memory};

use super::utils::AddressingMode;

// fn xor_16bit_and_set_flags(cpu: &mut CPU, val1: u16, val2: u16) -> Option<u16> {
//     let res = val1 ^ val2;
//     Some(res)
// }

// fn xor_8bit_and_set_flags(cpu: &mut CPU, val1: u8, val2: u8) -> Option<u8> {
//     let res = val1 ^ val2;
//     Some(res)
// }

impl CPU {
    pub(in crate::cpu) fn execute_xchg_16bit_regs_including_mem(&mut self, mem: &mut Memory) {
        match self.consume_bytes_and_parse_double_ins(mem) {
            AddressingMode::Address(reg_idx, addr) => {
                let reg_val = self.get_16bit_register_by_index(reg_idx);
                let addr_val = self.read_word_from_u20(mem, addr.clone());
                self.set_16bit_register_by_index(reg_idx, addr_val);
                self.write_word_to_u20(mem, addr, reg_val);
            }
            AddressingMode::Reg(operand_1_index, operand_2_index) => {
                let operand_1_val = self.get_16bit_register_by_index(operand_1_index);
                let operand_2_val = self.get_16bit_register_by_index(operand_2_index);
                self.set_16bit_register_by_index(operand_1_index, operand_2_val);
                self.set_16bit_register_by_index(operand_2_index, operand_1_val);
            }
        }
    }

    pub(in crate::cpu) fn execute_xchg_8bit_reg_including_mem(&mut self, mem: &mut Memory) {
        match self.consume_bytes_and_parse_double_ins(mem) {
            AddressingMode::Address(reg_idx, addr) => {
                let reg_val = self.get_8bit_register_by_index(reg_idx);
                let addr_val = self.read_byte_from_u20(mem, addr.clone());
                self.set_8bit_register_by_index(reg_idx, addr_val);
                self.write_byte_to_u20(mem, addr, reg_val);
            }
            AddressingMode::Reg(operand_1_idx, operand_2_idx) => {
                let operand_1_val = self.get_8bit_register_by_index(operand_1_idx);
                let operand_2_val = self.get_8bit_register_by_index(operand_2_idx);
                self.set_8bit_register_by_index(operand_1_idx, operand_2_val);
                self.set_8bit_register_by_index(operand_2_idx, operand_1_val);
            }
        }
    }

    pub(in crate::cpu) fn execute_xchg_ax(&mut self, opcode: u8){
        let reg_idx = opcode % 8;
        let reg_val = self.get_16bit_register_by_index(reg_idx);
        let ax_val = self.ax ;

        self.set_ax(reg_val);
        self.set_16bit_register_by_index(reg_idx, ax_val);
    }
}

#[cfg(test)]
mod xchg_exec_tests {
    use crate::cpu::instructions::test_macro::{run_code, execute_code};

    #[test]
    fn test_8bit_mem_and_reg() {
        let code = "
            MOV CH,      0x10
            MOV [0x100], 0x01
            XCHG [0x100], CH

            MOV AH, 0x20 
            MOV BH, 0x02 
            XCHG AH, BH
        ";
        let (cpu, mem) = run_code(code, 3);
        assert_eq!(cpu.read_byte_from_pointer(&mem, 0x100), 0x10);
        assert_eq!(cpu.get_cx_high(), 0x01);
    }

    #[test]
    fn test_16bit_mem_and_reg() {
        let code = "
            MOV  CX   , 0x1000
            MOV  BX   , 0x100 
            MOV  [0x100] , 0x100
            XCHG [BX] , CX

            MOV AX, 0x2000 
            MOV BX, 0x0200 
            XCHG AX, BX

            MOV SP, 0x3000
            MOV SI, 0x0300
            XCHG SP, SI
        ";
        let (cpu, mem) = execute_code(code);
        assert_eq!(cpu.read_word_from_pointer(&mem, 0x100), 0x1000);
        assert_eq!(cpu.cx, 0x0100);
        assert_eq!(cpu.ax, 0x0200);
        assert_eq!(cpu.bx, 0x2000);
        assert_eq!(cpu.stack_pointer, 0x0300);
        assert_eq!(cpu.source_index, 0x3000);
    }
}
