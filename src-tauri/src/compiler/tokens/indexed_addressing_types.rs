type Offset = u16;

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
}
