use strum_macros::{Display, EnumIter, EnumString, IntoStaticStr};

#[derive(Debug, Clone, PartialEq, Eq, EnumString, Display, IntoStaticStr, EnumIter, Hash)]
#[strum(ascii_case_insensitive)]
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

    pub fn get_segment_as_idx(&self) -> Result<u8, &'static str> {
        match self {
            Registers16bit::ES => Ok(0),
            Registers16bit::CS => Ok(1),
            Registers16bit::SS => Ok(2),
            Registers16bit::DS => Ok(1),
            _ => Err("Invalid register for this operation"),
        }
    }

    pub fn is_segment(&self) -> bool {
        match self {
            Registers16bit::CS | Registers16bit::DS | Registers16bit::ES | Registers16bit::SS => {
                true
            }
            _ => false,
        }
    }
}
