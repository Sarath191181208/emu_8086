use crate::compiler::{
    compilation_error::CompilationError,
    tokens::{registers16bit::Registers16bit, Token, Assembly8086Tokens},
    CompiledBytes,
};

pub fn get_as_0xc0_0xff_pattern(high_reg_idx: u8, low_reg_idx: u8) -> u8 {
    let ins = (0xC0) | (high_reg_idx / 2) << 4;
    let ins2 = low_reg_idx | (high_reg_idx % 2) << 3;
    ins | ins2
}

pub(crate) fn get_idx_from_reg(
    token: &Token,
    reg: &Registers16bit,
) -> Result<u8, CompilationError> {
    match reg.get_as_idx() {
        Ok(idx) => Ok(idx),
        Err(err) => Err(CompilationError::new(
            token.line_number,
            token.column_number,
            token.token_length,
            err,
        )),
    }
}

pub(crate) fn push_instruction(
    compiled_bytes: &mut Vec<u8>,
    ins: Vec<u8>,
    token: &Token,
    compiled_bytes_ref: &mut Vec<CompiledBytes>,
) {
    for byte in ins.iter() {
        compiled_bytes.push(*byte);
    }
    compiled_bytes_ref.push(CompiledBytes::new(
        ins,
        token.line_number,
        token.column_number,
    ));
}

pub(crate) fn if_num_8bit_to_16bit(token: Assembly8086Tokens) -> Assembly8086Tokens{
    match token {
        Assembly8086Tokens::Number8bit(num) => {
            let num = num as u16;
            Assembly8086Tokens::Number16bit(num)
        }
        _ => token,
    }
}