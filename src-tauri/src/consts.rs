pub type Byte = u8;
pub type Word = u16;

#[derive(Debug)]
#[repr(transparent)]
pub struct U20(u32);

impl std::fmt::Display for U20 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let is_negative = self.is_negative();
        let value = self.get_value();
        if is_negative {
            write!(f, "-{:#06x}", value)
        } else {
            write!(f, "{:#6x}", value)
        }
    }
}

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
        let offset = (value & 0xFFFF) as u16;
        (segment, offset)
    }

    fn is_negative(&self) -> bool {
        // check if the num is u8
        if self.0 <= 0xFF && self.0 > 0x80 {
            return true;
        }
        if self.0 <= 0xFFFF && self.0 > 0x8000 {
            return true;
        }
        false
    }

    fn get_value(&self) -> u32 {
        if self.is_negative() {
            match self.0 {
                0x80..=0xFF => return 0xFF - self.0 + 1,
                0x8000..=0xFFFF => return 0xFFFF - self.0 + 1,
                _ => return self.0,
            }
        }
        self.0
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
        match (self.is_negative(), other.is_negative()) {
            (true, true) | (false, false) => {
                let (res, _) = self.get_value().overflowing_add(other.get_value());
                U20::new(res)
            }
            (true, false) | (false, true) => {
                let (res, _) = self.get_value().overflowing_sub(other.get_value());
                U20::new(res)
            }
        }
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
