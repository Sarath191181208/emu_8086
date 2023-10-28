use crate::{compiler::compile_lines, cpu::CPU, memory::Memory};

#[deprecated(note="please use `run_code` instead")]
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

#[deprecated(note="please use `run_code` instead")]
#[macro_export]
macro_rules! generate_test_with_cycles {
    ($test_name:ident, $instructions:expr, $compare: expr, $times:expr) => {
        paste::item! {
            #[test]
            fn [<test_ $test_name>]() {
                let mut cpu = CPU::new();
                let mut mem = Memory::new();
                cpu.reset(&mut mem);
                $instructions(&mut cpu, &mut mem);
                for _ in 0..$times {
                    cpu.execute(&mut mem);
                }

                $compare(&cpu, &mem);
            }
        }
    };
}

fn compile_code_for_tests(code: &str, cpu: &mut CPU, mem: &mut Memory) {
    let (compiled_bytes, _, is_org_defined) = compile_lines(code, false).unwrap();
    if is_org_defined {
        cpu.set_org_defined()
    }
    cpu.write_instructions(mem, &compiled_bytes);
}

#[deprecated(note="please use `run_code` instead")]
pub fn compile_and_test_str(code: &str, cycles: usize, expected_fn: fn(&CPU, &Memory)) {
    let mut cpu = CPU::new();
    let mut mem = Memory::new();
    cpu.reset(&mut mem);

    compile_code_for_tests(code, &mut cpu, &mut mem);

    for _ in 0..cycles {
        cpu.execute(&mut mem);
    }

    expected_fn(&cpu, &mem);
}


pub fn run_code(code: &str, cycles: usize) -> (CPU, Memory){
    let mut cpu = CPU::new();
    let mut mem = Memory::new();
    cpu.reset(&mut mem);

    compile_code_for_tests(code, &mut cpu, &mut mem);

    for _ in 0..cycles {
        cpu.execute(&mut mem);
    }

    (cpu, mem)
}