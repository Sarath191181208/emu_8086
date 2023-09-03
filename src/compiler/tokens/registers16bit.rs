use strum_macros::EnumString;

#[derive(Debug, Clone, PartialEq, Eq, EnumString)]
pub(crate) enum Registers16bit {
    AX,
    BX,
    CX,
    DX,
    SI,
    DI,
    BP,
    SP,
    CS,
    DS,
    ES,
    SS,
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
