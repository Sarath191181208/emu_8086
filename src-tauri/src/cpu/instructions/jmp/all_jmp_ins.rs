use crate::{cpu::CPU, generate_16bit_jmp_label_method, generate_8bit_jmp_method, memory::Memory};

fn get_new_ip(ip: u16, offset: i16) -> u16 {
    if offset < 0 {
        ip.wrapping_sub(offset.unsigned_abs())
    } else {
        ip.wrapping_add(offset.unsigned_abs())
    }
}

fn exec_cf_zf_0_jmp(cpu: &mut CPU, offset: i16) -> Option<u16> {
    if cpu.zero_flag != false || cpu.carry_flag != false {
        return None;
    }

    let ip = cpu.instruction_pointer;
    // if -ve sub else address
    let new_ip = get_new_ip(ip, offset);
    Some(new_ip)
}

impl CPU {
    generate_8bit_jmp_method!(ja, exec_cf_zf_0_jmp);
    generate_16bit_jmp_label_method!(jnbe, exec_cf_zf_0_jmp);
}
