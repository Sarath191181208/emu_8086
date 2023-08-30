type Byte = u8;
type Word = u16;

struct Memory{
    mem: [Byte; 0xFFFF],
}

impl Memory {
    fn new() -> Memory {
        Memory{
            mem: [0; 0xFFFF],
        }
    }

    fn reset(&mut self) {
        self.mem = [0; 0xFFFF];
    }

    fn read(&self, address: Word) -> Byte {
        self.mem[address as usize]
    }

    fn write(&mut self, address: Word, data: Byte) {
        self.mem[address as usize] = data;
    }
}

struct CPU{

    // Memory
    program_counter: Word,
    stack_pointer: Word,

    // Registers
    accumulator: Byte,
    x_register: Byte,
    y_register: Byte,

    // Status Flags
    carry_flag: bool,
    zero_flag: bool,
    interrupt_disable_flag: bool,
    decimal_mode_flag: bool,
    break_command_flag: bool,
    overflow_flag: bool,
    negative_flag: bool,
}

impl CPU {
    fn new() -> CPU {
        CPU{
            program_counter: 0x0000,
            stack_pointer: 0x0000,
            accumulator: 0x00,
            x_register: 0x00,
            y_register: 0x00,
            carry_flag: false,
            zero_flag: false,
            interrupt_disable_flag: false,
            decimal_mode_flag: false,
            break_command_flag: false,
            overflow_flag: false,
            negative_flag: false,
        }
    } 

    fn reset(&mut self, mem: &mut Memory) {
        self.program_counter = 0xFFFC;
        self.stack_pointer = 0x0100;

        self.accumulator = 0x00;
        self.x_register = 0x00;
        self.y_register = 0x00;
        
        self.carry_flag = false;
        self.zero_flag = false;
        self.interrupt_disable_flag = false;
        self.decimal_mode_flag = false;
        self.break_command_flag = false;
        self.overflow_flag = false;
        self.negative_flag = false;

        mem.reset();
    }
}


fn main() {
    let mut cpu = CPU::new();
    let mut mem = Memory::new();
    cpu.reset(&mut mem); // Reset the CPU
    println!("{}", cpu.program_counter);
}
