## MOV( 0x8B ) Instruction 

### Description
It copies the data from the source operand to the destination operand. The source operand can be a register or a memory location, while the destination operand can be a register, a memory location or a segment register. The MOV instruction does not affect any flag in the EFLAGS register.
Addressing modes for the MOV instruction are as follows:

1. Register addressing mode, size of instruction is byte
```asm
    MOV AX, BX
```
2. Immediate addressing mode
```asm
    MOV AX, 0x1234
```
3. Memory addressing mode
```asm
    MOV AX, [0x1234]
```
4. Register indirect addressing mode
```asm
    MOV AX, [BX]
```
5. Register relative addressing mode
```asm
    MOV AX, [BX+SI]
```
6. Base relative addressing mode
```asm
    MOV AX, [BX+SI+8-bitoffset]
```
7. Base relative addressing mode
```asm
    MOV AX, [BX+SI+16-bitoffset]
```
8. Base relative addressing mode
```asm
    MOV AX, [16-bitoffset]
```
