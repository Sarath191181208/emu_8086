use strum_macros::{Display, EnumIter, EnumString, IntoStaticStr};

#[derive(Debug, Clone, IntoStaticStr, Display, PartialEq, Eq, EnumString, EnumIter, Hash)]
#[strum(ascii_case_insensitive)]
pub(crate) enum Instructions {
    Mov,
    Add,
    Inc,
    Dec,
    Sub,
    Mul,
    Jmp,
    Test,
    Loop,
    Hlt,
    Ret,

    Push,
    Pop,

    Proc,
    EndP,
    Call,

    Int,
    Iret,

    In,
    Out,
}
