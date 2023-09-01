## ADD instruction 

### Description
It adds the source operand to the destination operand and stores the result in the destination operand. The ADD instruction performs integer addition. It evaluates the result for both signed and unsigned integer operands and sets the OF and CF flags to indicate an overflow in the signed or unsigned result, respectively. The SF flag indicates the sign of the signed result.

### Altered Flags 
* OF, SF, ZF, AF, PF, CF

Addressing modes for the ADD instruction are as follows:

1. Register addressing mode, size of instruction is byte
```asm
    ADD AL, BL ;   0x02 0xD8
    ADD AX, BX ;   0x03 0xD8
```

2. Immediate addressing mode

The AX, AL registers have different opcodes for immediate addressing mode. The AL register has 0x04 opcode and AX register has 0x05 opcode.
```asm
    ADD AL, 0x12  ; 0x04 0x12
    ADD AX, 0x1234; 0x05 0x12 0x34

```

```asm
    ADD BL, 0x12  ; 0x80 0xC1..0xC7 0x12
    ADD BX, 0x1234; 0x81 0xC1..0xC7 0x12 0x34
    ADD BX, 0xFFAE; 0x83 0xC1..0xC7 0xAE
```
The third case only has 3 instructions while it should have four because the immediate value is 16 bits. The reason is that the assembler is smart enough to know that the immediate value can be represented in 8 bits and hence it uses the 0x83 opcode. The 0x83 opcode is used for 8 bit immediate values and the 0x81 opcode is used for 16 bit immediate values.
For the operation it masks and converts and adds.