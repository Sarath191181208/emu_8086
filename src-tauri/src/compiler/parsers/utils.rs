use crate::compiler::{
    compilation_error::CompilationError,
    tokenized_line::TokenizedLine,
    tokens::{
        registers16bit::Registers16bit, registers8bit::Registers8bit, Assembly8086Tokens, Token,
    },
    types_structs::{ArrayIndex, Label, VariableAddressMap, VariableReferenceMap, VariableType},
    CompiledBytesReference,
};

pub fn get_as_0xc0_0xff_pattern(high_reg_idx: u8, low_reg_idx: u8) -> u8 {
    let ins = (0xC0) | (high_reg_idx / 2) << 4;
    let ins2 = low_reg_idx | (high_reg_idx % 2) << 3;
    ins | ins2
}

pub(super) fn get_idx_from_reg(
    token: &Token,
    reg: &Registers16bit,
) -> Result<u8, CompilationError> {
    match reg.get_as_idx() {
        Ok(idx) => Ok(idx),
        Err(err) => Err(CompilationError::new_without_suggestions(
            token.line_number,
            token.column_number,
            token.token_length,
            err,
        )),
    }
}

pub(super) fn push_instruction(
    compiled_bytes: &mut Vec<u8>,
    ins: Vec<u8>,
    token: &Token,
    compiled_bytes_ref: &mut Vec<CompiledBytesReference>,
) {
    for byte in ins.iter() {
        compiled_bytes.push(*byte);
    }
    compiled_bytes_ref.push(CompiledBytesReference::new(
        ins,
        token.line_number,
        token.column_number,
    ));
}

#[macro_export]
macro_rules! convert_and_push_instructions {
    ($compiled_bytes:ident, $compiled_bytes_ref:ident, ($($token:expr => $bytes:expr),+)) => {
        $(
            push_instruction($compiled_bytes, $bytes, $token, $compiled_bytes_ref);
        )+
    };
}

pub(super) fn if_num_8bit_to_16bit(token: Assembly8086Tokens) -> Assembly8086Tokens {
    match token {
        Assembly8086Tokens::Number8bit(num) => {
            let num = num as u16;
            Assembly8086Tokens::Number16bit(num)
        }
        _ => token,
    }
}

pub(super) fn get_16bit_register(token: &Token) -> &Registers16bit {
    match &token.token_type {
        Assembly8086Tokens::Register16bit(reg) => reg,
        _ => unreachable!(),
    }
}

pub(super) fn get_8bit_register(token: &Token) -> &Registers8bit {
    match &token.token_type {
        Assembly8086Tokens::Register8bit(reg) => reg,
        _ => unreachable!(),
    }
}

pub(super) fn get_idx_from_token(token: &Token) -> Result<u8, CompilationError> {
    let reg = get_16bit_register(token);
    get_idx_from_reg(token, reg)
}

pub(super) fn check_comma<'a>(
    tokenized_line: &'a TokenizedLine<'a>,
    previous_token: &'a Token,
    i: usize,
) -> Result<(), CompilationError> {
    let sepertor_token = tokenized_line.get(
        i,
        format!("Expected , after {:?} got nothing", previous_token).to_string(),
        None,
    )?;
    if sepertor_token.token_type != Assembly8086Tokens::Comma {
        return Err(CompilationError::new_without_suggestions(
            sepertor_token.line_number,
            sepertor_token.column_number,
            sepertor_token.token_length,
            &format!(
                "Expected , after {:?} got {:?}",
                previous_token.token_type, sepertor_token
            ),
        ));
    }
    Ok(())
}

pub(super) fn get_token_as_label(token: &Token) -> &Label {
    match &token.token_type {
        Assembly8086Tokens::Character(label) => label,
        _ => unreachable!(),
    }
}

pub(super) fn is_variable_defined_as_16bit(
    map: &Option<&VariableAddressMap>,
    label: &Label,
) -> bool {
    match map {
        None => false,
        Some(map) => match map.get(label) {
            None => false,
            Some((reg_type, _)) => match reg_type {
                VariableType::Byte => false,
                VariableType::Word => true,
            },
        },
    }
}

pub(super) fn get_label_address_or_push_into_ref(
    idx: ArrayIndex,
    label: &Label,
    var_type: VariableType,
    variable_abs_offset_bytes_map: &VariableAddressMap,
    var_ref_map: &mut VariableReferenceMap,
) -> [u8; 2] {
    match variable_abs_offset_bytes_map.get(label) {
        Some((_, abs_addr)) => {
            let ins = (abs_addr & 0xFF) as u8;
            let ins2 = (abs_addr >> 8) as u8;
            [ins, ins2]
        }
        None => {
            let placeholder = [0x00, 0x00];
            var_ref_map.insert(label.clone(), (var_type, idx));
            placeholder
        }
    }
}
