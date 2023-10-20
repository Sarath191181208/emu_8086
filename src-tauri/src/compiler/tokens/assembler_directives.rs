use strum_macros::{Display, EnumString};

#[derive(Debug, Clone, PartialEq, Default, Eq, EnumString, Display, Hash)]
#[strum(ascii_case_insensitive)]
pub(crate) enum AssemblerDirectives {
    #[default]
    Org,
    #[strum(serialize = ".data")]
    Data,
    #[strum(serialize = ".code")]
    Code,

    Macro,
    EndM,

    Offset,
}
