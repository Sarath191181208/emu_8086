use strum_macros::{Display, EnumIter, EnumString, IntoStaticStr};

#[derive(Debug, Clone, PartialEq, Eq, EnumString, Display, IntoStaticStr, EnumIter, Hash)]
pub(crate) enum DefineData {
    #[strum(ascii_case_insensitive)]
    Dw,
    #[strum(ascii_case_insensitive)]
    Db,
}
