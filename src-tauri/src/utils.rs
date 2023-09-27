pub enum Either<T, U> {
    Left(T),
    Right(U),
}

// have a special implmentation for T = u8 and U = u16
impl Either<u8, u16> {
    pub fn get_as_u16(&self) -> u16 {
        match &self {
            Either::Left(x) => *x as u16,
            Either::Right(x) => *x,
        }
    }
}
