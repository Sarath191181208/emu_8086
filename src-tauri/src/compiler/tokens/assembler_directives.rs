use strum_macros::{EnumString, Display};

#[derive(Debug, Clone, PartialEq, Default, Eq, EnumString, Display)]
pub(crate) enum AssemblerDirectives {
    #[strum(ascii_case_insensitive)]
    #[default]
    Org,
    #[strum(ascii_case_insensitive, serialize = ".data")]
    Data,
    #[strum(ascii_case_insensitive, serialize = ".code")]
    Code,
}
