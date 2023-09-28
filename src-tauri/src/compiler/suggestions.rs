use serde::Serialize;

use super::{tokens::{instructions::Instructions, registers16bit::Registers16bit, registers8bit::Registers8bit}, types_structs::{Label, Variable}};


#[derive(Debug)]
pub enum SuggestionType {
    Instruction(&'static str),

    Registers16bit(&'static str),
    Registers8bit(&'static str),
    
    Variables16bit(Variable),
    Variables8bit(Variable),

    Label(Label),

    Constant16bit(u16),
    Constant8bit(u8),
}

impl Serialize for SuggestionType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        match self {
            SuggestionType::Instruction(instruction) => {
                serializer.serialize_str(&instruction.to_string())
            }
            SuggestionType::Registers16bit(register) => {
                serializer.serialize_str(&register.to_string())
            }
            SuggestionType::Registers8bit(register) => {
                serializer.serialize_str(&register.to_string())
            }
            SuggestionType::Variables16bit(variable) => {
                serializer.serialize_str(&variable.to_string())
            }
            SuggestionType::Variables8bit(variable) => {
                serializer.serialize_str(&variable.to_string())
            }
            SuggestionType::Label(label) => {
                serializer.serialize_str(&label.to_string())
            }
            SuggestionType::Constant16bit(constant) => {
                serializer.serialize_u16(*constant)
            }
            SuggestionType::Constant8bit(constant) => {
                serializer.serialize_u8(*constant)
            }
        }
    }
}