use super::{
    suggestions::SuggestionType,
    tokens::{registers16bit::Registers16bit, registers8bit::Registers8bit},
    types_structs::{VariableAddressMap, VariableType, LabelAddressMap},
};
use strum::IntoEnumIterator;

pub(super) fn get_org_100h() -> Vec<SuggestionType> {
    vec![SuggestionType::Constant16bit(0x100)]
}

// pub(super) fn get_all_instructions_suggestions() -> Vec<SuggestionType> {
//     Instructions::iter()
//         .map(|x| SuggestionType::Instruction(x.into()))
//         .collect()
// }

pub(super) fn get_8bit_number_suggestion() -> Vec<SuggestionType> {
    vec![SuggestionType::Constant8bit(0)]
}

pub(super) fn get_16bit_number_suggestion() -> Vec<SuggestionType> {
    vec![SuggestionType::Constant16bit(0)]
}

pub(super) fn get_all_labels_suggestions(label_line_num_map: &LabelAddressMap) -> Vec<SuggestionType>{
    label_line_num_map
        .iter()
        .map(|(x, _)| SuggestionType::Label(x.clone()))
        .collect()
}

pub(super) fn get_all_registers_and_variable_suggestions(
    variable_address_map: Option<&VariableAddressMap>,
) -> Vec<SuggestionType> {
    let mut suggestions = Vec::new();
    suggestions.extend(get_all_16bit_registers_suggestions());
    suggestions.extend(get_all_8bit_registers_suggestions());
    suggestions.extend(get_all_variables_suggestions(variable_address_map));
    suggestions
}

pub(super) fn get_all_16bit_registers_suggestions() -> Vec<SuggestionType> {
    Registers16bit::iter()
        .map(|x| SuggestionType::Registers16bit(x.into()))
        .collect()
}

pub(super) fn get_all_8bit_registers_suggestions() -> Vec<SuggestionType> {
    Registers8bit::iter()
        .map(|x| SuggestionType::Registers8bit(x.into()))
        .collect()
}

pub(super) fn get_all_variables_suggestions(
    variable_address_map: Option<&VariableAddressMap>,
) -> Vec<SuggestionType> {
    variable_address_map
        .unwrap_or(&VariableAddressMap::new())
        .iter()
        .map(|(x, y)| match y.0 {
            VariableType::Byte => SuggestionType::Variables8bit(x.clone()),
            VariableType::Word => SuggestionType::Variables16bit(x.clone()),
        })
        .collect()
}

pub(super) fn get_all_16bit_variables_suggestions(
    variable_address_map: Option<&VariableAddressMap>,
) -> Vec<SuggestionType> {
    variable_address_map
        .unwrap_or(&VariableAddressMap::new())
        .iter()
        .filter(|(_, y)| y.0 == VariableType::Word)
        .map(|(x, _)| SuggestionType::Variables16bit(x.clone()))
        .collect()
}

pub(super) fn get_all_8bit_variables_suggestions(
    variable_address_map: Option<&VariableAddressMap>,
) -> Vec<SuggestionType> {
    variable_address_map
        .unwrap_or(&VariableAddressMap::new())
        .iter()
        .filter(|(_, y)| y.0 == VariableType::Byte)
        .map(|(x, _)| SuggestionType::Variables8bit(x.clone()))
        .collect()
}

// pub(super) fn get_all_define_data_suggestions() -> Vec<SuggestionType> {
//     DefineData::iter()
//         .map(|x| SuggestionType::DefineData(x.into()))
//         .collect()
// }
