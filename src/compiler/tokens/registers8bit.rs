#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Registers8bit {
    AH,
    AL,
    BH,
    BL,
    CH,
    CL,
    DH,
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