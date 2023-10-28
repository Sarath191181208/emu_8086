---
title: MOV
description: Copies operand2 into operand1.
---

## Syntax

```asm6502
MOV operand1, operand2
```

## Attributes

| Attribute | Description |
|-----------|-------------|
| **Size**  | Byte, Word, or Doubleword. |
| **Operands** | The destination operand is a general-purpose register or memory location. The source operand is a general-purpose register, memory location, or immediate value. |
| **Action** | Copies the source operand (second operand) to the destination operand (first operand). |
| **Flags** | No Flags are changed |

## Supported Modes 

### Register Addressing Mode 

This is a mode in which the operand is specified in the instruction itself. The operand is either a general-purpose register or a segment register. 

```asm
MOV AX, BX
MOV BL, CH
```

### Immediate Addressing Mode

This is a mode in which the operand is specified in the instruction itself. The operand is either a general-purpose register or a segment register. 

```asm
MOV AX, 1234H
MOV BL, 12H
```

### Direct Addressing Mode

This is a mode in which the address of the operand is specified in the instruction itself. The operand is located in the memory. 

```asm
MOV AX, [1234H]
MOV CL, [1234H]
```

or equivalently

```asm
org 100h 
.data 
    Var1 dw 1234H
code: 
    MOV AX, Var1
```

### Register Indirect Addressing Mode

This is a mode in which the address of the operand is contained in the register specified in the instruction. The operand is located in the memory. 

```asm
MOV AX, [BX]
```

### Base-Plus-Index Addressing Mode

This is a mode in which the address of the operand is specified by the sum of a base register and an index register. The operand is located in the memory. 

```asm
MOV AX, [BX+SI]
```

### Base-Plus-Index-Plus-Displacement Addressing Mode

This is a mode in which the address of the operand is specified by the sum of a base register, an index register, and a displacement. The operand is located in the memory. 

```asm
MOV AX, [BX+SI+1234H]
```





