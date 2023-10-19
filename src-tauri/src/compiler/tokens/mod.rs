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

impl SignedU16 {
    pub fn new(val: u16) -> Self {
        Self {
            val,
            is_negative: false,
        }
    }

    pub fn overflowing_add(self, other: Self) -> (Self, bool) {
        match (self.is_negative, other.is_negative) {
            (false, false) | (true, true) => {
                let (res, overflow) = self.val.overflowing_add(other.val);
                (
                    Self {
                        val: res,
                        is_negative: self.is_negative,
                    },
                    overflow,
                )
            }
            (false, true) | (true, false) => {
                let (res, overflow) = self.val.overflowing_sub(other.val);
                (
                    Self {
                        val: res,
                        is_negative: if other.val > self.val {
                            other.is_negative
                        } else {
                            self.is_negative
                        },
                    },
                    overflow,
                )
            }
        }
    }

    pub fn overflowing_sub(self, other: Self) -> (Self, bool) {
        self.overflowing_add(other.negate())
    }

    pub(crate) fn negate(self) -> Self {
        Self {
            val: self.val,
            is_negative: !self.is_negative,
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
        if !self.is_negative {
            if self.val < 0x80 {
                return Ok(Either::Left(self.val as u8));
            } else {
                return Ok(Either::Right(self.val));
            }
        }
        if self.is_negative && self.val <= 0x80 {
            Ok(Either::Left(0xFF - (self.val as u8) + 1_u8))
        } else if self.val < 0x7FFF {
            Ok(Either::Right(0xFFFF - (self.val) + 1_u16))
        } else {
            Err("Number is too big to be converted to 16 bit, Because the number is negative and the number is greater than 0x7FFF")
        }
    }
}
