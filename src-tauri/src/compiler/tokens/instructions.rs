use strum_macros::{Display, EnumIter, EnumString, IntoStaticStr};

#[derive(Debug, Clone, IntoStaticStr, Display, PartialEq, Eq, EnumString, EnumIter, Hash)]
#[strum(ascii_case_insensitive)]
pub(crate) enum Instructions {
    Add,
    And,
    Call,
    Dec,
    EndP,
    Hlt,
    In,
    Inc,
    Int,
    Iret,
    Jmp,
    Lea,
    Les,
    Loop,
    Mov,
    Mul,
    Or,
    Out,
    Pop,
    Proc,
    Push,
    Ret,
    Sub,
    Test,
    Xor,
}
