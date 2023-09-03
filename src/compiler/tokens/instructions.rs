use strum_macros::EnumString;

#[derive(Debug, Clone, PartialEq, Eq, EnumString)]
pub(crate) enum Instructions {
    #[strum(ascii_case_insensitive)]
    Mov,
    #[strum(ascii_case_insensitive)]
    Add,
    #[strum(ascii_case_insensitive)]
    Inc,
    #[strum(ascii_case_insensitive)]
    Dec,
}
