use unicase::UniCase;

use crate::utils::Either;

use self::{
    assembler_directives::AssemblerDirectives, indexed_addressing_types::IndexedAddressingTypes,
};

pub mod assembler_directives;
pub mod data;
pub mod indexed_addressing_types;
pub mod instructions;
pub mod registers16bit;
pub mod registers8bit;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) enum Assembly8086Tokens {
    // Resisters
    // ex: ax, bx, cx, dx, si, di, bp, sp, cs, ds, es, ss, ip
    // ah, al, bh, bl, ch, cl, dh, dl
    Register16bit(registers16bit::Registers16bit),
    Register8bit(registers8bit::Registers8bit),
    // Instructions
    // ex: mov, add, sub, mul, div, inc, dec, jmp, je, jne, jg, jge, jl, jle,
    // call, ret, push, pop, cmp, nop, hlt, lea, xchg, xlat, aaa, aas, aam, aad, cbw,
    // cwd, clc, stc, cmc, cli, sti, cld, std, sahf, lahf, pushf, popf, movsb, movsw,
    // movsd, movsq, cmpsb, cmpsw, cmpsd, cmpsq, scasb, scasw, scasd, scasq, lodsb, lodsw, lodsd,
    // lodsq, stosb, stosw, stosd, stosq, rep, repe,
    Instruction(instructions::Instructions),
    // Directives
    // ex: .data, .code, .model, .stack, .const, .byte, .word, .dword, .qword, .tbyte, .float, .double, .real4, .real8, .real10, .xmmword, .ymmword, .zmmword, .bnd, .oword, .tword, .far, .near, .proc, .endp, .public, .assume, .include, .include_lib, .model, .stack, .const, .byte, .word, .dword, .qword,
    // .tbyte, .float, .double, .real4, .real8, .real10, .xmmword, .ymmword, .zmmword, .bnd, .oword, .tword, .far, .near, .proc, .endp, .public, .assume, .include, .include_lib
    // Directive,
    AssemblerDirectives(AssemblerDirectives),

    // Comments
    // ex: ;, //, #
    Comment,

    // Labels
    // ex: label:
    // Label,

    // Numbers
    Number16bit(u16),
    Number8bit(u8),

    // Strings
    // ex: "Hello World"
    // String,

    // Operators
    // ex: +, -, *, /, %, &, |, ^, ~, <<, >>, &&, ||, !, ==,
    // Operators,

    // Separators
    // ex: (, ), [, ], {, }, ,, ., :, ;, ?, @, `
    Space,
    Comma,
    Colon,
    OpenSquareBracket,
    CloseSquareBracket,
    Plus,
    Minus,

    // i.e this like this
    // [bx], [dx], [si], [di]
    // [bx|dx + si|di]
    // [bx|dx + si|di + 0x10]
    // [0x1234]
    IndexedAddressing(IndexedAddressingTypes),
    ByteIndexedAddressing(IndexedAddressingTypes),

    // Define data
    Data(data::DefineData),

    // Label
    Character(UniCase<String>),
}

// impl the Display trait for Assembly8086Tokens
impl std::fmt::Display for Assembly8086Tokens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Assembly8086Tokens::Register16bit(register) => write!(f, "{}", register),
            Assembly8086Tokens::Register8bit(register) => write!(f, "{}", register),
            Assembly8086Tokens::Instruction(instruction) => write!(f, "{}", instruction),
            Assembly8086Tokens::Comment => write!(f, "Comment"),
            Assembly8086Tokens::Number16bit(number) => write!(f, "{}", number),
            Assembly8086Tokens::Number8bit(number) => write!(f, "{}", number),
            Assembly8086Tokens::Space => write!(f, "Space"),
            Assembly8086Tokens::Comma => write!(f, "Comma"),
            Assembly8086Tokens::Colon => write!(f, "Colon"),
            Assembly8086Tokens::Character(character) => write!(f, "{}", character),
            Assembly8086Tokens::Data(data) => write!(f, "{}", data),
            Assembly8086Tokens::AssemblerDirectives(directive) => write!(f, "{}", directive),
            Assembly8086Tokens::OpenSquareBracket => write!(f, "["),
            Assembly8086Tokens::CloseSquareBracket => write!(f, "]"),
            Assembly8086Tokens::Plus => write!(f, "+"),
            Assembly8086Tokens::IndexedAddressing(_) => write!(f, "IndexedAddressing"),
            Assembly8086Tokens::ByteIndexedAddressing(_) => write!(f, "ByteIndexedAddressing"),
            Assembly8086Tokens::Minus => write!(f, "-"),
        }
    }
}

#[derive(Debug, Clone, Eq)]
pub(crate) struct Token {
    pub token_type: Assembly8086Tokens,

    pub line_number: u32,
    pub column_number: u32,
    pub token_length: u32,
}

impl std::hash::Hash for Token {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.token_type.hash(state);
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.token_type == other.token_type
    }
}

impl Token {
    pub fn new(
        token_type: Assembly8086Tokens,
        line_number: u32,
        column_number: u32,
        token_length: u32,
    ) -> Token {
        Token {
            token_type,
            line_number,
            column_number,
            token_length,
        }
    }

    pub fn is_abs_eq(&self, other: &Token) -> bool {
        self.token_type == other.token_type
            && self.token_length == other.token_length
            && self.line_number == other.line_number
            && self.column_number == other.column_number
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct SignedU16 {
    pub val: u16,
    pub is_negative: bool,
}

impl From<[u8; 2]> for SignedU16 {
    fn from(val: [u8; 2]) -> Self {
        // convert the bytes to u16 and check if it's negative
        // if it is then set the is_negative to true
        let val = u16::from_le_bytes(val);
        Self {
            val,
            is_negative: false,
        }
    }
}

impl From<Either<u8, u16>> for SignedU16 {
    fn from(val: Either<u8, u16>) -> Self {
        Self::new(val.get_as_u16())
    }
}

impl From<i16> for SignedU16 {
    fn from(val: i16) -> Self {
        let is_negative = val < 0;
        let val = val.unsigned_abs();
        Self { val, is_negative }
    }
}

impl SignedU16 {
    pub fn new(val: u16) -> Self {
        Self {
            val,
            is_negative: false,
        }
    }

    pub fn to_le_bytes_vec(&self) -> Vec<u8> {
        match self.as_num().unwrap() {
            Either::Left(val) => val.to_le_bytes().to_vec(),
            Either::Right(val) => val.to_le_bytes().to_vec(),
        }
    }

    pub fn overflowing_add(self, other: Self) -> (Self, bool) {
        let val = self.as_i16();
        let other = other.as_i16();
        let (val, is_overflow) = val.overflowing_add(other);
        (Self::from(val), is_overflow)
    }

    pub fn overflowing_sub(self, other: Self) -> (Self, bool) {
        let val = self.as_i16();
        let other = other.as_i16();
        let (val, is_overflow) = val.overflowing_sub(other);
        (Self::from(val), is_overflow)
    }

    pub(crate) fn negate(self) -> Self {
        Self {
            val: self.val,
            is_negative: !self.is_negative,
        }
    }

    fn as_i16(self) -> i16 {
        match self.as_num() {
            Ok(num) => match num {
                Either::Left(num) => (num as i8) as i16,
                Either::Right(num) => num as i16,
            },
            Err(_) => 0,
        }
    }

    pub(crate) fn as_num_token(self) -> Result<Assembly8086Tokens, &'static str> {
        match self.as_num() {
            Ok(num) => match num {
                Either::Left(num) => Ok(Assembly8086Tokens::Number8bit(num)),
                Either::Right(num) => Ok(Assembly8086Tokens::Number16bit(num)),
            },
            Err(err) => Err(err),
        }
    }

    pub(crate) fn as_num(self) -> Result<Either<u8, u16>, &'static str> {
        // check if less than u8::MAX
        if self.val < 0x80 {
            let val = self.val as u8;
            let val_i8 = val as i8 * if self.is_negative { -1 } else { 1 };
            return Ok(Either::Left(val_i8 as u8));
        }
        Ok(Either::Right(
            (self.val as i16 * if self.is_negative { -1 } else { 1 }) as u16,
        ))
    }
}
