use strum_macros::{Display, EnumIter, EnumString, IntoStaticStr};

#[derive(Debug, Clone, IntoStaticStr, Display, PartialEq, Eq, EnumString, EnumIter, Hash)]
#[strum(ascii_case_insensitive)]
pub(crate) enum Instructions {
    Adc,
    Add,
    And,
    Call,
    Cmp,
    Dec,
    EndP,
    Hlt,
    In,
    Inc,
    Int,
    Iret,
    Ja,
    Jae,
    Jb,
    Jbe,
    Jc,
    Jcxz,
    Je,
    Jg,
    Jge,
    Jl,
    Jle,
    Jna,
    Jnae,
    Jnb,
    Jnbe,
    Jnc,
    Jne,
    Jng,
    Jnge,
    Jnl,
    Jnle,
    Jno,
    Jnp,
    Jns,
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
    Sbb,
    Sub,
    Test,
    Xchg,
    Xor,
}
