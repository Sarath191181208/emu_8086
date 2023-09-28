use strum_macros::{EnumString, IntoStaticStr, Display};

#[derive(Debug, Clone, IntoStaticStr, Display, PartialEq, Eq, EnumString)]
pub(crate) enum Instructions {
    #[strum(ascii_case_insensitive)]
    Mov,
    #[strum(ascii_case_insensitive)]
    Add,
    #[strum(ascii_case_insensitive)]
    Inc,
    #[strum(ascii_case_insensitive)]
    Dec,
    #[strum(ascii_case_insensitive)]
    Sub,
    #[strum(ascii_case_insensitive)]
    Mul,
    #[strum(ascii_case_insensitive)]
    Jmp
}