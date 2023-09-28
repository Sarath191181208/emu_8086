use serde::Serialize;

use super::types_structs::{Label, Variable};


#[derive(Debug)]
pub enum SuggestionType {
    Instruction(&'static str),

    Registers16bit(&'static str),
    Registers8bit(&'static str),

    DefineData(&'static str),
    
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
        match &self{
            SuggestionType::Instruction(x) => serializer.serialize_str(x),
            SuggestionType::Registers16bit(x) => serializer.serialize_str(x),
            SuggestionType::Registers8bit(x) => serializer.serialize_str(x),
            SuggestionType::DefineData(x) => serializer.serialize_str(x),
            SuggestionType::Variables16bit(x) => serializer.serialize_str(&x.to_string()),
            SuggestionType::Variables8bit(x) => serializer.serialize_str(&x.to_string()),
            SuggestionType::Label(x) => serializer.serialize_str(&x.to_string()),
            SuggestionType::Constant16bit(x) => serializer.serialize_u16(*x),
            SuggestionType::Constant8bit(x) => serializer.serialize_u8(*x),
        }
    }
}
