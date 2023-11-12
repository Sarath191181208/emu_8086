use crate::{cpu::CPU, generate_single_line_execution_ins, memory::Memory};

use super::utils::AddressingMode;

fn execute_div_operation(cpu: &mut CPU, val: u16) {
    // AX = (DX AX) / operand
    // DX = remainder (modulus)
    let num = cpu.ax as u32;
    let num = (cpu.dx as u32) << 16 | num;
    let val = val as u32;

    let num_str = format!("0x{:X}", num);
    let val_str = format!("0x{:X}", val);
    dbg!(num_str, val_str);

    let quotient = num / val;
    let remainder = num % val;
    cpu.set_ax((quotient & 0xFFFF) as u16);
    cpu.set_dx((remainder & 0xFFFF) as u16);
}

fn execute_div_8bit_operation(cpu: &mut CPU, val: u8) {
    // AL = AX / operand
    // AH = remainder (modulus)
    let quotient = (cpu.ax as u16) / (val as u16);
    let remainder = (cpu.ax as u16) % (val as u16);
    cpu.set_ax_low(quotient as u8);
    cpu.set_ax_high(remainder as u8);
}

impl CPU {
    generate_single_line_execution_ins!(div, execute_div_operation, execute_div_8bit_operation);
}

#[cfg(test)]
mod div_execution_tests {
    use crate::cpu::instructions::test_macro::execute_code;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_div_standard() {
        let code = "
            MOV AX, 0x1234
            MOV DX, 0x2
            MOV CX, 0x3
            DIV CX
        ";

        let (cpu, _) = execute_code(code);
        let quotient = cpu.ax;
        let remainder = cpu.dx;
        assert_eq!(quotient, 0xB0BC);
        assert_eq!(remainder, 0x0);
    }

    #[test]
    fn test_div_8bit_reg() {
        let code = "
            MOV AX, 0x1234
            MOV BL, 0xF1
            DIV BL
        ";
        let (cpu, _) = execute_code(code);
        let quotient = cpu.get_ax_low();
        let remainder = cpu.get_ax_high();
        assert_eq!(quotient, 0x13);
        assert_eq!(remainder, 0x51)
    }

    #[test]
    fn test_div_mem() {
        let code = "
            MOV AX, 0x1234
            MOV [0x100], 0x345
            DIV [0x100]
        ";

        let (cpu, _) = execute_code(code);
        let quotient = cpu.ax;
        let remainder = cpu.dx;
        assert_eq!(quotient, 0x05);
        assert_eq!(remainder, 0x1DB);
    }
}
