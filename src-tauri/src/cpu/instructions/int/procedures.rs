use crate::{cpu::CPU, memory::Memory};

impl CPU {
    fn wirte_0xffff_with_iret_and_single_ins_mid(&mut self, mem: &mut Memory, offset: u16, ins: u8) {
        mem.write_instructions(0xF400, offset, &vec![0xFF, 0xFF, 0xCD, ins, 0xCF]);
    }
    pub(crate) fn write_0x10_interrupt_procedure(&mut self, mem: &mut Memory) {
        self.wirte_0xffff_with_iret_and_single_ins_mid(mem, 0x190, 0x10);
    }

    pub(crate) fn write_0x21_interrupt_procedure(&mut self, mem: &mut Memory) {
        self.wirte_0xffff_with_iret_and_single_ins_mid(mem, 0x200, 0x21);
    }
}
