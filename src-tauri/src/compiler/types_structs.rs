use super::Token;
use serde::Serialize;
use unicase::UniCase;

pub type Label = UniCase<String>;
pub type LineNumber = u16;
pub type IsLabelBeforeRef = bool;

#[derive(Debug, Serialize)]
pub struct CompiledBytes {
    pub bytes: Vec<u8>,
    pub line_number: u32,
    pub column_number: u32,
}

impl CompiledBytes {
    pub fn new(bytes: Vec<u8>, line_number: u32, column_number: u32) -> Self {
        Self {
            bytes,
            line_number,
            column_number,
        }
    }
}

pub(crate) struct CompiledLine {
    pub compiled_bytes: Vec<u8>,
    pub compiled_bytes_ref: Vec<CompiledBytes>,
    pub labels: Vec<String>,
    pub label_idx_map: std::collections::HashMap<String, (Token, u16)>,
}

impl CompiledLine {
    pub fn new() -> Self {
        Self {
            compiled_bytes: Vec::new(),
            compiled_bytes_ref: Vec::new(),
            labels: Vec::new(),
            label_idx_map: std::collections::HashMap::new(),
        }
    }

    pub fn extend(&mut self, other: Self) {
        self.compiled_bytes.extend(other.compiled_bytes);
        self.compiled_bytes_ref.extend(other.compiled_bytes_ref);
        self.label_idx_map.extend(other.label_idx_map);
    }
}
