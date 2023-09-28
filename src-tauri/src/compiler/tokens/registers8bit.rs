use strum_macros::{Display, EnumString, IntoStaticStr};

#[derive(Debug, Clone, PartialEq, Eq, EnumString, Display, IntoStaticStr)]
pub(crate) enum Registers8bit {
    #[strum(ascii_case_insensitive)]
    AH,
    #[strum(ascii_case_insensitive)]
    AL,
    #[strum(ascii_case_insensitive)]
    BH,
    #[strum(ascii_case_insensitive)]
    BL,
    #[strum(ascii_case_insensitive)]
    CH,
    #[strum(ascii_case_insensitive)]
    CL,
    #[strum(ascii_case_insensitive)]
    DH,
    #[strum(ascii_case_insensitive)]
    DL,
}

impl Registers8bit {
    pub fn get_as_idx(&self) -> u8 {
        match self {
            Registers8bit::AL => 0,
            Registers8bit::CL => 1,
            Registers8bit::DL => 2,
            Registers8bit::BL => 3,
            Registers8bit::AH => 4,
            Registers8bit::CH => 5,
            Registers8bit::DH => 6,
            Registers8bit::BH => 7,
        }
    }
}
