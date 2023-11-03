pub mod add;
pub mod dec;
pub mod inc;
pub mod jmp;
pub mod mov;
pub mod mul;
pub mod sbb;
pub mod sub;

pub mod and;
pub mod call;
pub mod in_compilation;
pub mod int;
pub mod iret;
pub mod lea;
pub mod les;
pub mod loop_ins;
pub mod or;
pub mod out_ins;
pub mod pop;
pub mod push;
pub mod ret;
pub mod test_ins;
pub mod test_macro;
pub(in crate::cpu::instructions) mod utils;
pub mod xchg;
pub mod xor;
