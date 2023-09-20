pub mod instructions;
pub mod registers16bit;
pub mod registers8bit;
pub mod assembler_directives;

#[derive(Debug, Clone, PartialEq, Eq)]
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

    // Error
    Character(String),
}

#[derive(Debug, Clone)]
pub(crate) struct Token {
    pub token_type: Assembly8086Tokens,

    pub line_number: u32,
    pub column_number: u32,
    pub token_length: u32,
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
}
