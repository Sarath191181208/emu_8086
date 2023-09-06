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
