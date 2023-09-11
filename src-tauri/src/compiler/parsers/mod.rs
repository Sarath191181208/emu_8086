pub mod add;
pub mod dec;
pub mod inc;
pub mod jmp;
pub mod mov;
pub mod sub;
pub mod mul;

pub(in crate::compiler::parsers) mod pattern_extractors;
pub(in crate::compiler::parsers) mod utils;
