use strum_macros::{Display, EnumIter, EnumString, IntoStaticStr};

#[derive(Debug, Clone, PartialEq, Eq, EnumString, Display, IntoStaticStr, EnumIter, Hash)]
#[strum(ascii_case_insensitive)]
pub(crate) enum DefineData {
    Dw,
    Db,
}
