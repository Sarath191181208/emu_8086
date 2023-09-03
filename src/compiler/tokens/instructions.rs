use strum_macros::EnumString;

#[derive(Debug, Clone, PartialEq, Eq, EnumString)]
pub(crate) enum Instructions {
    MOV,
    ADD,
    INC,
    DEC,
}