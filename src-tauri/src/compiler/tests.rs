// write tests
// run tests with cargo test

#[macro_export]
macro_rules! test_compile {
    ($name:ident, $code:expr, $expected_fn:expr) => {
        #[test]
        fn $name() {
            let (inst, _) = match compile_str($code, false) {
                Ok(instructions) => instructions,
                Err(e) => {
                    // e.print_compilation_error($code);
                    for err in e {
                        err.print_compilation_error($code);
                    }
                    assert!(false);
                    return;
                }
            };
            $expected_fn(&inst);
        }
    };
}

#[macro_export]
macro_rules! compile_and_compare_ins {
    ($test_name: ident, $code: expr, $expected_ins: expr) => {
        test_compile!($test_name, $code, |compiled_instructions: &Vec<u8>| {
            assert_eq!(compiled_instructions, &$expected_ins);
        });
    };
}

#[allow(dead_code)]
pub(super) fn generate_num_ins(size: u16) -> String {
    let mut ins = String::new();
    for _ in 0..size {
        ins.push_str("INC AX\n");
    }
    ins
}
