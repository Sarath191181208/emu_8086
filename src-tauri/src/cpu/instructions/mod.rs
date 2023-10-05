pub mod add;
pub mod dec;
pub mod inc;
pub mod jmp;
pub mod mov;
pub mod mul;
pub mod sub;

pub mod loop_ins;
pub mod test_macro;
pub mod ret;

pub(in crate::cpu::instructions) mod utils;
