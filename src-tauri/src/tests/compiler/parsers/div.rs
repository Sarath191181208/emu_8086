use crate::{compile_and_compare_ins, compiler::compile_str, test_compile};
use pretty_assertions::assert_eq;

compile_and_compare_ins!(
    test_div_16bit,
    "
        DIV AX
        DIV BX 
        DIV [0x100]      
        DIV [BX+SI]
        DIV [BX+SI+0x10]
        DIV [BX+SI+0x100]
    ",
    vec![
        0xF7, 0xF0, // DIV AX
        0xF7, 0xF3, // DIV BX
        0xF7, 0x36, 0x00, 0x01, // DIV [0x100]
        0xF7, 0x30, // DIV [BX+SI]
        0xF7, 0x70, 0x10, // DIV [BX+SI+0x10]
        0xF7, 0xB0, 0x00, 0x01, // DIV [BX+SI+0x100]
    ]
);

compile_and_compare_ins!(
    test_div_8bit, 
    "
        DIV AL
        DIV BL
        DIV b.[0x100]
        DIV b.[BX+SI]
        DIV b.[BX+SI+0x10]
        DIV b.[BX+SI+0x100]
    ", 
    vec![
        0xF6, 0xF0, // DIV AL
        0xF6, 0xF3, // DIV BL
        0xF6, 0x36, 0x00, 0x01, // DIV b.[0x100]
        0xF6, 0x30, // DIV b.[BX+SI]
        0xF6, 0x70, 0x10, // DIV b.[BX+SI+0x10]
        0xF6, 0xB0, 0x00, 0x01 // DIV b.[BX+SI+0x100]
    ]
);
