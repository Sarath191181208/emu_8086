use strum_macros::{EnumString, Display};

#[derive(Debug, Clone, PartialEq, Eq, EnumString, Display)]
pub(crate) enum Registers16bit {
    #[strum(ascii_case_insensitive)]
    AX,
    #[strum(ascii_case_insensitive)]
    BX,
    #[strum(ascii_case_insensitive)]
    CX,
    #[strum(ascii_case_insensitive)]
    DX,
    #[strum(ascii_case_insensitive)]
    SI,
    #[strum(ascii_case_insensitive)]
    DI,
    #[strum(ascii_case_insensitive)]
    BP,
    #[strum(ascii_case_insensitive)]
    SP,
    #[strum(ascii_case_insensitive)]
    CS,
    #[strum(ascii_case_insensitive)]
    DS,
    #[strum(ascii_case_insensitive)]
    ES,
    #[strum(ascii_case_insensitive)]
    SS,
    #[strum(ascii_case_insensitive)]
    IP,
}

impl Registers16bit {
    pub fn get_as_idx(&self) -> Result<u8, &'static str> {
        match self {
            Registers16bit::AX => Ok(0),
            Registers16bit::CX => Ok(1),
            Registers16bit::DX => Ok(2),
            Registers16bit::BX => Ok(3),
            Registers16bit::SP => Ok(4),
            Registers16bit::BP => Ok(5),
            Registers16bit::SI => Ok(6),
            Registers16bit::DI => Ok(7),
            _ => Err("Invalid register for this operation"),
        }
    }
}
