use crate::{
    compiler::{
        compilation_error::CompilationError,
        tokenized_line::TokenizedLine,
        tokens::{
            indexed_addressing_types::IndexedAddressingTypes, registers16bit::Registers16bit,
            registers8bit::Registers8bit, Assembly8086Tokens, SignedU16, Token,
        },
        types_structs::{
            ArrayIndex, Label, VariableAddressMap, VariableReferenceMap, VariableType,
        },
        CompiledBytesReference,
    },
    utils::Either,
};

// define a static string
pub const THIS_SHOULDNT_HAPPEN: &str = "This shouldn't happen, Please report this!";

fn get_as_0xnf_in_0x3f_increment_pattern(n: u8, high_reg_idx: u8, low_reg_idx: u8) -> u8 {
    let ins = n | (high_reg_idx / 2) << 4;
    let ins2 = low_reg_idx | (high_reg_idx % 2) << 3;
    ins | ins2
}

pub fn get_as_0x00_0x3f_pattern(high_reg_idx: u8, low_reg_idx: u8) -> u8 {
    get_as_0xnf_in_0x3f_increment_pattern(0x00, high_reg_idx, low_reg_idx)
}

pub fn get_as_0x40_0x7f_pattern(high_reg_idx: u8, low_reg_idx: u8) -> u8 {
    get_as_0xnf_in_0x3f_increment_pattern(0x40, high_reg_idx, low_reg_idx)
}

pub fn get_as_0x80_0xbf_pattern(high_reg_idx: u8, low_reg_idx: u8) -> u8 {
    get_as_0xnf_in_0x3f_increment_pattern(0x80, high_reg_idx, low_reg_idx)
}

pub fn get_as_0xc0_0xff_pattern(high_reg_idx: u8, low_reg_idx: u8) -> u8 {
    get_as_0xnf_in_0x3f_increment_pattern(0xC0, high_reg_idx, low_reg_idx)
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

pub(in crate::compiler) fn push_instruction(
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

pub(super) fn check_token<'a>(
    tokenized_line: &'a TokenizedLine<'a>,
    previous_token: &'a Token,
    i: usize,
    expected_token: &Assembly8086Tokens,
) -> Result<(), CompilationError> {
    let token = tokenized_line.get(
        i,
        format!(
            "Expected {:?} after {:?} got nothing",
            expected_token, previous_token
        )
        .to_string(),
        None,
    )?;

    if &token.token_type != expected_token {
        return Err(CompilationError::new_without_suggestions(
            token.line_number,
            token.column_number,
            token.token_length,
            &format!(
                "Expected {:?} after {:?} got {:?}",
                expected_token, previous_token.token_type, token.token_type
            ),
        ));
    }

    Ok(())
}

pub(super) fn check_comma<'a>(
    tokenized_line: &'a TokenizedLine<'a>,
    previous_token: &'a Token,
    i: usize,
) -> Result<(), CompilationError> {
    check_token(
        tokenized_line,
        previous_token,
        i,
        &Assembly8086Tokens::Comma,
    )
}

pub(super) fn get_token_as_label(token: &Token) -> &Label {
    match &token.token_type {
        Assembly8086Tokens::Character(label) => label,
        _ => unreachable!(),
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

pub(crate) fn iterate_with_seperator(
    start_index: usize,
    end_index: usize,
    tokenized_line: &TokenizedLine,
    seperator: &Assembly8086Tokens,
    mut callback: impl FnMut(&Token) -> Result<(), CompilationError>,
) -> Result<usize, CompilationError> {
    let mut i = start_index;
    while i < end_index {
        let token = match tokenized_line.get(
            i,
            "This shouldn't happen, Report this! Err: iterate_with_seperator:174".to_string(),
            None,
        ) {
            Ok(token) => token,
            Err(err) => return Err(err),
        };

        callback(token)?;
        i += 1;
        if i < end_index {
            check_token(tokenized_line, token, i, seperator)?;
            i += 1;
        }
    }

    Ok(i)
}

// pub(in crate::compiler::parsers) fn unimplemented_instruction_addressing_mode(
//     token: &Token,
//     len: usize,
// ) -> CompilationError {
//     CompilationError::new_without_suggestions(
//         token.line_number,
//         token.column_number,
//         len as u32,
//         "This addressing mode is not implemented yet",
//     )
// }

impl IndexedAddressingTypes {
    pub(super) fn get_index_or_err(&self, token: &Token) -> Result<u8, CompilationError> {
        match self.get_as_idx() {
            Ok(idx) => Ok(idx),
            Err(err) => Err(CompilationError::error_with_token(token, err)),
        }
    }
}

impl Registers16bit {
    pub(super) fn get_index_or_err(&self, token: &Token) -> Result<u8, CompilationError> {
        match self.get_as_idx() {
            Ok(idx) => Ok(idx),
            Err(err) => Err(CompilationError::error_with_token(token, err)),
        }
    }
}

impl CompilationError {
    pub(super) fn default() -> Self {
        CompilationError::new_without_suggestions(
            0,
            0,
            0,
            "This shouldn't happen, Please report this!",
        )
    }
}

impl SignedU16 {
    pub(super) fn as_either_u8_or_u16(
        &self,
        token: &Token,
    ) -> Result<Either<u8, u16>, CompilationError> {
        match &self.as_num() {
            Ok(num) => match num {
                Either::Left(num) => Ok(Either::Left(*num)),
                Either::Right(num) => Ok(Either::Right(*num)),
            },
            Err(err) => Err(CompilationError::new_without_suggestions(
                token.line_number,
                token.column_number,
                token.token_length,
                err,
            )),
        }
    }

    pub(super) fn as_u16(&self) -> u16 {
        if self.is_negative {
            return 0xFFFF - self.val + 1;
        }
        self.val
    }
}
