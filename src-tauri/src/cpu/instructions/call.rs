use crate::{cpu::CPU, memory::Memory};

impl CPU {
    pub(in crate::cpu) fn execute_call_and_16bitaddr(&mut self, mem: &mut Memory) {
        let offset = self.consume_word(mem);
        let curr_ip = self.instruction_pointer;
        self.push_stack(mem, curr_ip);
        self.set_instruction_pointer_from_16bitoffset(offset);
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::instructions::test_macro::run_code;

    #[test]
    fn no_offset_indexed_add() {
        let code = "
        inc ax 
    PROC main 

            inc ax
    ENDP main 

    CALL main 
    ; this is a commet 
    ; ahh! 
    inc ax 
    ";
    let (cpu, _) = run_code(code, 4);
    assert_eq!(cpu.ax, 3);
    }
}
