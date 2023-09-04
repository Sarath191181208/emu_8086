use crate::compiler::{tokens::{Token, registers16bit::Registers16bit}, compilation_error::CompilationError, CompiledBytes};

pub fn get_as_0xc0_0xff_pattern(high_reg_idx: u8, low_reg_idx: u8) -> u8 {
    let ins = (0xC0) | (high_reg_idx / 2) << 4;
    let ins2 = low_reg_idx | (high_reg_idx % 2) << 3;
    ins | ins2
}

pub(crate) fn get_idx_from_reg(token: &Token, reg: &Registers16bit) -> Result<u8, CompilationError> {
    match reg.get_as_idx() {
        Ok(idx) => Ok(idx),
        Err(err) => {
            return Err(CompilationError::new(
                token.line_number,
                token.column_number,
                token.token_length,
                err,
            ));
        }
    }
}

pub(crate) fn push_instruction(compiled_bytes: &mut Vec<u8>, ins: Vec<u8>, token: &Token, compiled_bytes_ref:  &mut Vec<CompiledBytes>) {
    for byte in ins.iter() {
        compiled_bytes.push(*byte);
    }
    compiled_bytes_ref.push(CompiledBytes::new(
        ins,
        token.line_number,
        token.column_number,
    ));
}
