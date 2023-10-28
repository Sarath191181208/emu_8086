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
    use crate::cpu::instructions::test_macro::run_code;

    #[test]
    fn ret_test(){
        let (cpu, _) = run_code("PUSH 0x03 \n ret", 2);
        assert_eq!(cpu.instruction_pointer, 0x03);
    }
    
}
