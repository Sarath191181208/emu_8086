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
