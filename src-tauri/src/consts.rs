pub type Byte = u8;
pub type Word = u16;


#[repr(transparent)]
pub struct U20(u32);

impl U20 {
    pub const MIN: U20 = U20(0);
    pub const MAX: U20 = U20(1 << 20);

    pub fn new(value: u32) -> U20 {
        assert!(value <= Self::MAX.0);
        U20(value)
    }

    pub fn get(&self) -> u32 {
        self.0
    }

    pub fn as_segment_offset(&self) -> (u16, u16) {
        let value = self.0;
        // i.e value = 0x000F_FFFF
        // segment = 0x000F, offset = 0xFFFF
        let segment = (value >> 16) as u16;
        let offset= (value & 0xFFFF) as u16;
        (segment, offset)
    }
}

impl From<u32> for U20 {
    fn from(value: u32) -> Self {
        U20::new(value)
    }
}

impl From<u16> for U20 {
    fn from(value: u16) -> Self {
        U20::new(value as u32)
    }
}

impl From<u8> for U20 {
    fn from(value: u8) -> Self {
        U20::new(value as u32)
    }
}

impl From<U20> for u32 {
    fn from(value: U20) -> Self {
        value.get()
    }
}

impl std::ops::Add for U20 {
    type Output = U20;

    fn add(self, other: U20) -> U20 {
        U20::new(self.0 + other.0)
    }
}

impl std::ops::Sub for U20 {
    type Output = U20;

    fn sub(self, other: U20) -> U20 {
        U20::new(self.0 - other.0)
    }
}

impl std::ops::Mul for U20 {
    type Output = U20;

    fn mul(self, other: U20) -> U20 {
        U20::new(self.0 * other.0)
    }
}

impl std::ops::Div for U20 {
    type Output = U20;

    fn div(self, other: U20) -> U20 {
        U20::new(self.0 / other.0)
    }
}

impl std::ops::Rem for U20 {
    type Output = U20;

    fn rem(self, other: U20) -> U20 {
        U20::new(self.0 % other.0)
    }
}

impl std::ops::BitAnd for U20 {
    type Output = U20;

    fn bitand(self, other: U20) -> U20 {
        U20::new(self.0 & other.0)
    }
}

impl std::ops::BitOr for U20 {
    type Output = U20;

    fn bitor(self, other: U20) -> U20 {
        U20::new(self.0 | other.0)
    }
}

impl std::ops::BitXor for U20 {
    type Output = U20;

    fn bitxor(self, other: U20) -> U20 {
        U20::new(self.0 ^ other.0)
    }
}

impl std::ops::Not for U20 {
    type Output = U20;

    fn not(self) -> U20 {
        U20::new(!self.0)
    }
}