use std::collections::HashMap;

use crate::utils::Either;

use super::Token;
use serde::Serialize;
use unicase::UniCase;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VariableType {
    Byte,
    Word,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProcDefitionType{
    Proc,
    EndP,
}

pub type ArrayIndex = usize; // i.e index of array
pub type Label = UniCase<String>; // case insensitive
pub type LabelLength = usize; // i.e len of label
pub type TokenIndex = usize; // i.e index of token in line
pub type LineNumber = usize; // i.e line number
pub type CompiledBytesIndexedLineNumber = usize; // i.e line number of this particular token in compiled bytes
pub type NumberOfBytes = u16; // i.e size of mem
pub type IsLabelBeforeRef = bool; // i.e is label before reference
pub type LabelAddressMap = HashMap<Label, LineNumber>;
pub(crate) type LabelRefrenceList = Vec<(Label, Token, LineNumber, LineNumber)>;

pub type Variable = Label;
// The map used to store where a particular variable is being referenced
pub type VariableReferenceMap = HashMap<Variable, (VariableType, ArrayIndex)>;
// The map used to store where a particular variable is being defined
pub type VariableAddressMap = HashMap<Variable, (VariableType, NumberOfBytes)>;
// The map used to store where a particular variable is being defined  (i.e. the line number)
pub type VariableAddressDefinitionMap = HashMap<Variable, (VariableType, LineNumber)>;
// The list used to store where a particular variable is being referenced
pub type VariableReferenceList<'a> = Vec<(Label, VariableType, LineNumber, LineNumber)>;

pub type ProcDefinitionMap = HashMap<Label, ProcDefitionType>;
pub type ProcOffsetDefinitionMap = HashMap<Label, Either<i8, i16>>;
pub type ProcReferenceMap = HashMap<Label, ArrayIndex>;
pub type ProcReferenceList = Vec<(Label, CompiledBytesIndexedLineNumber, LineNumber)>;

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
    pub variable_reference_map: VariableReferenceMap,
    pub variable_abs_address_map: VariableAddressDefinitionMap,

    pub proc_definition_map: ProcDefinitionMap,
    pub proc_reference_map: ProcReferenceMap,
}

impl CompiledLine {
    pub fn new() -> Self {
        Self {
            compiled_bytes: Vec::new(),
            compiled_bytes_ref: Vec::new(),
            labels: Vec::new(),
            label_idx_map: std::collections::HashMap::new(),
            variable_reference_map: std::collections::HashMap::new(),
            variable_abs_address_map: std::collections::HashMap::new(),
            proc_definition_map: ProcDefinitionMap::new(),
            proc_reference_map: ProcReferenceMap::new(),
        }
    }

    pub fn extend(&mut self, other: Self) {
        self.compiled_bytes.extend(other.compiled_bytes);
        self.compiled_bytes_ref.extend(other.compiled_bytes_ref);
        self.label_idx_map.extend(other.label_idx_map);
        self.variable_reference_map
            .extend(other.variable_reference_map);
        self.variable_abs_address_map
            .extend(other.variable_abs_address_map);
        self.proc_definition_map.extend(other.proc_definition_map);
        self.proc_reference_map.extend(other.proc_reference_map);
    }
}
