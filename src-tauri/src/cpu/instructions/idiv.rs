use crate::{cpu::CPU, generate_single_line_execution_ins, memory::Memory};

use super::utils::AddressingMode;

fn execute_idiv_operation(cpu: &mut CPU, val: u16) {
    // AX = (DX AX) / operand
    // DX = remainder (modulus)
    let num = cpu.ax as u32;
    let num = ((cpu.dx as u32) << 16 | num) as i32;
    let val = val as i32;

    let quotient = num / val;
    let remainder = num % val;
    cpu.set_ax((quotient & 0xFFFF) as u16);
    cpu.set_dx((remainder & 0xFFFF) as u16);
}

fn execute_idiv_8bit_operation(cpu: &mut CPU, val: u8) {
    // AL = AX / operand
    // AH = remainder (modulus)
    let quotient = (cpu.ax as i16) / (val as i16);
    let remainder = (cpu.ax as i16) % (val as i16);
    cpu.set_ax_low(quotient as i8 as u8);
    cpu.set_ax_high(remainder as u8);
}

impl CPU {
    generate_single_line_execution_ins!(idiv, execute_idiv_operation, execute_idiv_8bit_operation);
}

#[cfg(test)]
mod div_execution_tests {
    use crate::cpu::instructions::test_macro::execute_code;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_idiv_standard() {
        let code = "
            MOV AX, 0xFFFF
            MOV DX, 0x00
            MOV CX, 0xF0
            IDIV CX
        ";

        let (cpu, _) = execute_code(code);
        let quotient = cpu.ax;
        let remainder = cpu.dx;
        assert_eq!(quotient, 0x0111);
        assert_eq!(remainder, 0x0F);
    }

    #[test]
    fn test_idiv_8bit_reg() {
        let code = "
            MOV AX, -203
            MOV BL, 4
            IDIV BL
        ";
        let (cpu, _) = execute_code(code);
        let quotient = cpu.get_ax_low();
        let remainder = cpu.get_ax_high();
        assert_eq!(quotient, 0xCE);
        assert_eq!(remainder, 0xFD)
    }

    #[test]
    fn test_idiv_mem() {
        let code = "
            MOV AX, -203 
            MOV  b.[0x100], 4
            IDIV b.[0x100]
        ";

        let (cpu, _) = execute_code(code);
        assert_eq!(cpu.ax, 0xFDCE)
    }
}
