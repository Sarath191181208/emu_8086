use std::collections::HashMap;

use super::Token;
use serde::Serialize;
use unicase::UniCase;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VariableType {
    Byte,
    Word,
}

pub type ArrayIndex = usize; // i.e index of array
pub type Label = UniCase<String>; // case insensitive
pub type LabelLength = usize; // i.e len of label
pub type TokenIndex = usize; // i.e index of token in line
pub type LineNumber = usize; // i.e line number
pub type NumberOfBytes = u16; // i.e size of mem
pub type IsLabelBeforeRef = bool; // i.e is label before reference
pub type LabelAddressMap = HashMap<Label, LineNumber>;

pub type VariableReferenceMap = HashMap<Label, (VariableType, ArrayIndex)>;
pub type VariableAddressMap = HashMap<Label, (VariableType, NumberOfBytes)>;
pub type VariableAddressDefinitionMap = HashMap<Label, (VariableType, LineNumber)>;

// This struct is used to store the compiled bytes of the coverted line
// for ex: MOV AX, BX
// will be converted to
// [
//      CompiledBytes { bytes: [0x8B], line_number: 1, column_number: 1 }
//      CompiledBytes { bytes: [0xC3], line_number: 1, column_number: 2 }
// ]
#[derive(Debug, Serialize)]
pub struct CompiledBytesReference {
    pub bytes: Vec<u8>,
    pub line_number: u32,
    pub column_number: u32,
}

/// Initializer for CompiledBytesReference
impl CompiledBytesReference {
    pub fn new(bytes: Vec<u8>, line_number: u32, column_number: u32) -> Self {
        Self {
            bytes,
            line_number,
            column_number,
        }
    }
}

/// This is a helper struct to pack and send all the vars needed to compile a line
#[derive(Debug)]
pub(crate) struct CompiledLine {
    pub compiled_bytes: Vec<u8>,
    pub compiled_bytes_ref: Vec<CompiledBytesReference>,
    pub labels: Vec<String>,
    pub label_idx_map: std::collections::HashMap<String, (Token, TokenIndex)>,
    pub label_reference_map: VariableReferenceMap,
    pub label_abs_address_map: VariableAddressDefinitionMap,
}

impl CompiledLine {
    pub fn new() -> Self {
        Self {
            compiled_bytes: Vec::new(),
            compiled_bytes_ref: Vec::new(),
            labels: Vec::new(),
            label_idx_map: std::collections::HashMap::new(),
            label_reference_map: std::collections::HashMap::new(),
            label_abs_address_map: std::collections::HashMap::new(),
        }
    }

    pub fn extend(&mut self, other: Self) {
        self.compiled_bytes.extend(other.compiled_bytes);
        self.compiled_bytes_ref.extend(other.compiled_bytes_ref);
        self.label_idx_map.extend(other.label_idx_map);
        self.label_reference_map.extend(other.label_reference_map);
        self.label_abs_address_map
            .extend(other.label_abs_address_map);
    }
}
