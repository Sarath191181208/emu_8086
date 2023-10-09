use crate::{cpu::CPU, memory::Memory};

impl CPU {
    pub(in crate::cpu) fn execute_interrupt(&mut self, mem: &mut Memory) {
        // push the current instruction pointer to the stack
        self.push_stack(mem, self.get_flags_as_16bit_number());
        self.push_stack(mem, self.get_code_segment());
        let next_ins_offset = 0x01;
        self.push_stack(mem, self.get_instruciton_pointer() + next_ins_offset);  

        // convert interrupt flag to a 16 bit number
        self.interrupt_disable_flag = true;

        // get the interrupt vector
        let interrupt_arg = self.consume_byte(mem);

        match interrupt_arg{
            0x10 => {
                // move cs to F400 and ip to 0190
                self.set_code_segment(0xF400);
                self.set_instruction_pointer(0x0190);
            }
            0x21 => {
                // move cs to F400 and ip to 0190
                self.set_code_segment(0xF400);
                self.set_instruction_pointer(0x0200);
            }

            _ => {
                panic!("Unknown interrupt: {}", interrupt_arg);
            }
        }


    }
}
