use strum_macros::EnumString;

#[derive(Debug, Clone, PartialEq, Eq, EnumString)]
pub(crate) enum Instructions {
    #[strum(ascii_case_insensitive)]
    Mov,
    #[strum(ascii_case_insensitive)]
    Add,
    #[strum(ascii_case_insensitive)]
    Inc,
    #[strum(ascii_case_insensitive)]
    Dec,
    #[strum(ascii_case_insensitive)]
    Sub,
    #[strum(ascii_case_insensitive)]
    Mul,
    #[strum(ascii_case_insensitive)]
    Jmp
}

// impl the Display trait for Instructions
impl std::fmt::Display for Instructions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instructions::Mov => write!(f, "Mov"),
            Instructions::Add => write!(f, "Add"),
            Instructions::Inc => write!(f, "Inc"),
            Instructions::Dec => write!(f, "Dec"),
            Instructions::Sub => write!(f, "Sub"),
            Instructions::Mul => write!(f, "Mul"),
            Instructions::Jmp => write!(f, "Jmp")
        }
    }
}
