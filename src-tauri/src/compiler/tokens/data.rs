use strum_macros::{Display, EnumString};

#[derive(Debug, Clone, PartialEq, Eq, EnumString, Display)]
pub(crate) enum DefineData {
    #[strum(ascii_case_insensitive)]
    Dw,
    #[strum(ascii_case_insensitive)]
    Db,
}
