pub mod add;
pub mod dec;
pub mod inc;
pub mod jmp;
pub mod mov;
pub mod mul;
pub mod sub;

pub mod call;
pub mod in_compilation;
pub mod int;
pub mod iret;
pub mod loop_ins;
pub mod out_ins;
pub mod pop;
pub mod push;

pub mod ret;
pub mod test_ins;

pub mod test_macro;

pub(in crate::cpu::instructions) mod utils;
