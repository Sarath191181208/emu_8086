#[cfg(test)]
mod tests_directives {
    use crate::{compiler::compile_str, test_compile};

    test_compile!(
        define_org,
        "
        org 0x100
        mov ax, 0x1234
        ",
        |instructions: &Vec<u8>| {
            assert_eq!(instructions, &[0xB8, 0x34, 0x12]);
        }
    );

    test_compile!(
        test_data_as_jmp, 
        "
            org 100h
            .data 
                mov ax, bx ; 3
                mov cx, dx ; 3
                inc ax     ; 1
                inc al     ; 1
            code: 
                mov ax, bx
        ",
        |instructions: &Vec<u8>| {
        assert_eq!(instructions, &[0x05, 0x34, 0x12]);
    }
    );
}
