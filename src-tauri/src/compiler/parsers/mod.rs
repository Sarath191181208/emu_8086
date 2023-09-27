pub mod add;
pub mod dec;
pub mod inc;
pub mod jmp;
pub mod mov;
pub mod mul;
pub mod sub;
pub mod directives;
pub mod var;

pub(in crate::compiler::parsers) mod pattern_extractors;
pub(in crate::compiler::parsers) mod utils;
