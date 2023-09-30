use serde::{ser::SerializeMap, Serialize};

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
        S: serde::Serializer,
    {
        match &self {
            // SuggestionType::Instruction(x) => serializer.
            // conert instruction into a dict {instruction: x, type: "instruction"}
            SuggestionType::Instruction(x) => {
                let mut s = serializer.serialize_map(Some(2))?;
                s.serialize_entry("value", x)?;
                s.serialize_entry("type", "instruction")?;
                s.end()
            }
            SuggestionType::Registers16bit(x) => {
                let mut s = serializer.serialize_map(Some(2))?;
                s.serialize_entry("value", x)?;
                s.serialize_entry("type", "register16bit")?;
                s.end()
            }
            SuggestionType::Registers8bit(x) => {
                let mut s = serializer.serialize_map(Some(2))?;
                s.serialize_entry("value", x)?;
                s.serialize_entry("type", "register8bit")?;
                s.end()
            }
            SuggestionType::DefineData(x) => {
                let mut s = serializer.serialize_map(Some(2))?;
                s.serialize_entry("value", x)?;
                s.serialize_entry("type", "define")?;
                s.end()
            }
            SuggestionType::Variables16bit(x) => {
                let mut s = serializer.serialize_map(Some(2))?;
                s.serialize_entry("value", &x.to_string())?;
                s.serialize_entry("type", "variable16bit")?;
                s.end()
            }
            SuggestionType::Variables8bit(x) => {
                let mut s = serializer.serialize_map(Some(2))?;
                s.serialize_entry("value", &x.to_string())?;
                s.serialize_entry("type", "variable8bit")?;
                s.end()
            }
            SuggestionType::Label(x) => {
                let mut s = serializer.serialize_map(Some(2))?;
                s.serialize_entry("value", &x.to_string())?;
                s.serialize_entry("type", "label")?;
                s.end()
            }
            SuggestionType::Constant16bit(x) => {
                let mut s = serializer.serialize_map(Some(2))?;
                s.serialize_entry("value", x)?;
                s.serialize_entry("type", "constant16bit")?;
                s.end()
            }
            SuggestionType::Constant8bit(x) => {
                let mut s = serializer.serialize_map(Some(2))?;
                s.serialize_entry("value", x)?;
                s.serialize_entry("type", "constant8bit")?;
                s.end()
            }
        }
    }
}
