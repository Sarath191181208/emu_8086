use crate::{compile_and_compare_ins, compiler::compile_str, test_compile};
use pretty_assertions::assert_eq;

compile_and_compare_ins!(
    test_jz,
    "
        org 100h

        start:
        jz label 

        mov [0x100], 0x100 
        mov [0x100], 0x100 
        mov [0x100], 0x100 
        mov [0x100], 0x100 
        mov [0x100], 0x100 
        mov [0x100], 0x100  
        mov [0x100], 0x100 
        mov [0x100], 0x100 
        mov [0x100], 0x100 
        mov [0x100], 0x100 
        mov [0x100], 0x100 
        mov [0x100], 0x100 
        mov [0x100], 0x100 
        mov [0x100], 0x100 
        mov [0x100], 0x100 
        mov [0x100], 0x100 
        mov [0x100], 0x100 
        mov [0x100], 0x100           
        mov [0x100], 0x100 
        mov [0x100], 0x100 
        mov [0x100], 0x100  

        jz label

        label:  

        jz label
         
        jz 0x10   
        jz start
        jz 0x00 
        ",
    get_compiled_bytes()
);

fn get_compiled_bytes() -> Vec<u8> {
    let starting_bytes = [0x75, 0x03, 0xE9, 0x80, 0x00];

    let mid_bytes_repeat = vec![0xC7, 0x06, 0x00, 0x01, 0x00, 0x01].repeat(21);

    let after_bytes = [
        0x74, 0x00, 0x74, 0xFE, 0x74, 0x0E, 0x75, 0x03, 0xE9, 0x72, 0xFF, 0x74, 0xFE,
    ];

    let mut compiled_bytes = starting_bytes.to_vec();
    compiled_bytes.extend(mid_bytes_repeat);
    compiled_bytes.extend(after_bytes);
    compiled_bytes
}
