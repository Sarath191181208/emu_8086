use crate::{cpu::CPU, memory::Memory};

impl CPU {
    pub(in crate::cpu) fn execute_ret(&mut self, mem: &mut Memory) {
        // pop the stack and set the instruction pointer to the value
        let ptr = self.pop_stack(mem);
        self.set_instruction_pointer(ptr);
    }
}

#[cfg(test)]
mod test {
    use crate::{
        cpu::{instructions::test_macro::{compile_and_test_str, compile_code_for_tests}, CPU},
        memory::Memory, generate_test_with_cycles,
    };

        generate_test_with_cycles!(
        ret_test,
        (|cpu: &mut CPU, mem: &mut Memory| {
            compile_code_for_tests("ret", cpu, mem);
            cpu.push_stack(mem, 0x03);
        }),
        (|cpu: &CPU, _: &Memory| {
            assert_eq!(cpu.instruction_pointer, 0x03);
        }),
        1
    );
}