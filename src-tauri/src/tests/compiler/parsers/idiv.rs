use crate::{compile_and_compare_ins, compiler::compile_str, test_compile};
use pretty_assertions::assert_eq;

compile_and_compare_ins!(
    test_idiv_16bit,
    "
        IDIV AX
        IDIV BX 
        IDIV [0x100]      
        IDIV [BX+SI]
        IDIV [BX+SI+0x10]
        IDIV [BX+SI+0x100]
    ",
    vec![
        0xF7, 0xF8, // DIV AX
        0xF7, 0xFB, // DIV BX
        0xF7, 0x3E, 0x00, 0x01, // DIV [0x100]
        0xF7, 0x38, // DIV [BX+SI]
        0xF7, 0x78, 0x10, // DIV [BX+SI+0x10]
        0xF7, 0xB8, 0x00, 0x01, // DIV [BX+SI+0x100]
    ]
);

compile_and_compare_ins!(
    test_idiv_8bit,
    "
        IDIV AL
        IDIV BL
        IDIV b.[0x100]
        IDIV b.[BX+SI]
        IDIV b.[BX+SI+0x10]
        IDIV b.[BX+SI+0x100]
    ",
    vec![
        0xF6, 0xF8, // DIV AL
        0xF6, 0xFB, // DIV BL
        0xF6, 0x3E, 0x00, 0x01, // DIV b.[0x100]
        0xF6, 0x38, // DIV b.[BX+SI]
        0xF6, 0x78, 0x10, // DIV b.[BX+SI+0x10]
        0xF6, 0xB8, 0x00, 0x01 // DIV b.[BX+SI+0x100]
    ]
);
