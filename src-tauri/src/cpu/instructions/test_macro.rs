#[macro_export]
macro_rules! generate_test {
    ($test_name:ident, $instructions:expr, $compare: expr) => {
        paste::item! {
            #[test]
            fn [<test_ $test_name>]() {
                let mut cpu = CPU::new();
                let mut mem = Memory::new();
                cpu.reset(&mut mem);

                $instructions(&mut cpu, &mut mem);

                cpu.execute(&mut mem);

                $compare(&cpu, &mem);
            }
        }
    };
}