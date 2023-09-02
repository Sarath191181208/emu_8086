## INC Instruction 

### Description
It increments the destination operand by 1. The destination operand can be a register or a memory location. The INC instruction affects the OF, SF, ZF, AF, and PF flags.   

### Altered Flags
* OF, SF, ZF, AF, PF

Addressing modes for the INC instruction are as follows:
1. Register addressing mode, size of instruction is byte
```asm
    INC AL ;   0xFE 0xC0..0xC7
    INC AX ;   40..47
```

2. Memory addressing mode
```asm
    INC [BX+SI]  ; 0xFE 0x00
    INC [BX+DI]  ; 0xFE 0x01
    INC [BP+SI]  ; 0xFE 0x02
    INC [BP+DI]  ; 0xFE 0x03
    INC [SI]     ; 0xFE 0x04
    INC [DI]     ; 0xFE 0x05
    INC [0x1234] ; 0xFE 0x06 0x34 0x12
    INC [BX]     ; 0xFE 0x07
```
