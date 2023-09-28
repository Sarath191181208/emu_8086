use strum_macros::{Display, EnumString, IntoStaticStr, EnumIter};

#[derive(Debug, Clone, PartialEq, Eq, EnumString, Display, IntoStaticStr, EnumIter)]
pub(crate) enum DefineData {
    #[strum(ascii_case_insensitive)]
    Dw,
    #[strum(ascii_case_insensitive)]
    Db,
}
