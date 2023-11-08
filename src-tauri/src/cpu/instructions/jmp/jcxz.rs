use crate::{cpu::CPU, generate_8bit_jmp_method, memory::Memory};

fn exec_fn(cpu: &mut CPU, offset: i16) -> Option<u16> {
    let ip = cpu.instruction_pointer;
    // if -ve sub else address
    if cpu.cx != 0 {
        return None;
    }
    if offset < 0 {
        Some(ip.wrapping_sub(offset.unsigned_abs()))
    } else {
        Some(ip.wrapping_add(offset.unsigned_abs()))
    }
}

impl CPU {
    generate_8bit_jmp_method!(jmp_if_cx_zero, exec_fn);
}
