## SUB Instruction

### Description
It subtracts the source operand from the destination operand and stores the result in the destination operand. The SUB instruction performs integer subtraction. It evaluates the result for both signed and unsigned integer operands and sets the OF and CF flags to indicate an overflow in the signed or unsigned result, respectively. The SF flag indicates the sign of the signed result.

### Altered Flags
* OF, SF, ZF, AF, PF, CF

Addressing modes for the SUB instruction are as follows:

1. Register addressing mode, size of instruction is byte
```asm
    SUB AL, BL ;   0x2A 0xC0..FF
    SUB AX, BX ;   0x2B 0xC0..FF
```

2. inmediate addressing mode, size of instruction is byte
```asm
    SUB AL, 0x10 ; 0x2C 0x10
    SUB AX, 0x10 ; 0x2D 0x10
```
```asm
    SUB BX, 0xFFFF; 0x83 0xE8..0xEF 0xFF
    SUB BX, 0x0001; 0x81 0xE8..0xEF 0x01 0x00

    SUB BL, 0xFF;   0x80 0xE8..0xEF 0xFF
```