use super::SignedU16;

type Offset = SignedU16;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IndexedAddressingTypes {
    BX(Option<Offset>),
    BP(Option<Offset>),
    SI(Option<Offset>),
    DI(Option<Offset>),
    BxSi(Option<Offset>),
    BxDi(Option<Offset>),
    BpSi(Option<Offset>),
    BpDi(Option<Offset>),
    Offset(Offset),
}

impl IndexedAddressingTypes {
    pub fn get_as_idx(&self) -> Result<u8, &'static str> {
        match self {
            IndexedAddressingTypes::BxSi(_) => Ok(0),
            IndexedAddressingTypes::BxDi(_) => Ok(1),
            IndexedAddressingTypes::BpSi(_) => Ok(2),
            IndexedAddressingTypes::BpDi(_) => Ok(3),
            IndexedAddressingTypes::SI(_) => Ok(4),
            IndexedAddressingTypes::DI(_) => Ok(5),
            IndexedAddressingTypes::BP(_) => Ok(6),
            IndexedAddressingTypes::BX(_) => Ok(7),
            IndexedAddressingTypes::Offset(_) => Err("Offset is not a valid index register"),
        }
    }

    pub fn get_offset(&self) -> Option<Offset> {
        match self {
            IndexedAddressingTypes::BxSi(offset) => *offset,
            IndexedAddressingTypes::BxDi(offset) => *offset,
            IndexedAddressingTypes::BpSi(offset) => *offset,
            IndexedAddressingTypes::BpDi(offset) => *offset,
            IndexedAddressingTypes::SI(offset) => *offset,
            IndexedAddressingTypes::DI(offset) => *offset,
            IndexedAddressingTypes::BP(offset) => *offset,
            IndexedAddressingTypes::BX(offset) => *offset,
            IndexedAddressingTypes::Offset(offset) => Some(*offset),
        }
    }
}
