pub mod add;
pub mod dec;
pub mod directives;
pub mod inc;
pub mod jmp;
pub mod loop_ins;
pub mod mov;
pub mod mul;
pub mod sub;
pub mod var;
pub mod call;

pub(in crate::compiler::parsers) mod pattern_extractors;
pub(in crate::compiler) mod utils;
