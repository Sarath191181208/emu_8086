pub mod add;
pub mod dec;
pub mod inc;
pub mod jmp;
pub mod mov;
pub mod mul;
pub mod sub;

pub mod call;
pub mod loop_ins;
pub mod ret;
pub mod int;

pub mod test_macro;

pub(in crate::cpu::instructions) mod utils;
