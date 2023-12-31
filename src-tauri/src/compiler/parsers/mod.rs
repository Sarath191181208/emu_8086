pub mod adc;
pub mod add;
pub mod and;
pub mod call;
pub mod cmp;
pub mod dec;
pub mod directives;
pub mod in_ins;
pub mod inc;
pub mod jmp;
pub mod lea;
pub mod les;
pub mod loop_ins;
pub mod mov;
pub mod mul;
pub mod or;
pub mod out_ins;
pub mod pop;
pub mod push;
pub mod sbb;
pub mod shl;
pub mod sub;
pub mod test_ins;
pub mod var;
pub mod xchg;
pub mod xor;

pub(in crate::compiler) mod pattern_extractors;
pub(in crate::compiler) mod utils;
