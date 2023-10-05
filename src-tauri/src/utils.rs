use serde::Serialize;

use crate::compiler::tokens::Token;

pub enum Either<T, U> {
    Left(T),
    Right(U),
}

// have a special implmentation for T = u8 and U = u16
impl Either<u8, u16> {
    pub fn get_as_u16(&self) -> u16 {
        match &self {
            Either::Left(x) => *x as u16,
            Either::Right(x) => *x,
        }
    }

    pub fn get_as_bytes(&self) -> Vec<u8> {
        match &self {
            Either::Left(x) => vec![*x],
            Either::Right(x) => x.to_le_bytes().to_vec(),
        }
    }
}

impl Either<i8, i16> {
    pub fn get_as_unsigned(&self) -> Either<u8, u16> {
        match &self {
            Either::Left(x) => Either::Left(*x as u8),
            Either::Right(x) => Either::Right(*x as u16),
        }
    }
}

impl From<i8> for Either<i8, i16> {
    fn from(x: i8) -> Self {
        Self::Left(x)
    }
}

impl From<i16> for Either<i8, i16> {
    fn from(x: i16) -> Self {
        let is_x_in_i8_range = x <= i8::MAX as i16 && x >= i8::MIN as i16;
        if is_x_in_i8_range {
            Self::Left(x as i8)
        } else {
            Self::Right(x)
        }
    }
}

// also impl from usize check the least possilbe and assign i8 or i16
// also check the min limit
impl From<usize> for Either<i8, i16> {
    fn from(x: usize) -> Self {
        let is_x_in_i8_range = x <= i8::MAX as usize && x >= i8::MIN as usize;
        if is_x_in_i8_range {
            Self::Left(x as i8)
        } else {
            Self::Right(x as i16)
        }
    }
}

#[derive(Debug, Serialize)]
pub struct TokenPosition {
    line_number: usize,
    column_number: usize,
    length: usize,
}

impl TokenPosition {
    pub fn new(line_number: usize, column_number: usize, length: usize) -> Self {
        Self {
            line_number,
            column_number,
            length,
        }
    }
}

// impl a from Token
impl From<&Token> for TokenPosition {
    fn from(token: &Token) -> Self {
        Self {
            line_number: token.line_number as usize,
            column_number: token.column_number as usize,
            length: token.token_length as usize,
        }
    }
}
