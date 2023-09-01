## MOV Instruction 

### Description
It copies the data from the source operand to the destination operand. The source operand can be a register or a memory location, while the destination operand can be a register, a memory location or a segment register. The MOV instruction does not affect any flag in the EFLAGS register.
Addressing modes for the MOV instruction are as follows:

1. Register addressing mode, size of instruction is byte
```asm
    MOV AL, BL ;   0x8A 0xC0..=0xFF
    MOV AX, BX ;   0x8B 0xC0..=0xFF
```

1. Immediate addressing mode
```asm
    MOV AL, 0x12  ; 0xB0..=0xB7 0x12
    MOV AX, 0x1234; 0xB8..=0xBF 0x12 0x34
```
1. Memory addressing mode
```asm
    MOV AX, [0x1234]
```
1. Register indirect addressing mode
```asm
    MOV AX, [BX]
```
1. Register relative addressing mode
```asm
    MOV AX, [BX+SI]
```
1. Base relative addressing mode
```asm
    MOV AX, [BX+SI+8-bitoffset]
```
1. Base relative addressing mode
```asm
    MOV AX, [BX+SI+16-bitoffset]
```
1. Base relative addressing mode
```asm
    MOV AX, [16-bitoffset]
```
