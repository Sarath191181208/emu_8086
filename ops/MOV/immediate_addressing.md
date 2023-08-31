## MOV instruction 
```asm
    MOV AX, 0x1234
    MOV AL, 0x12
```

**For next cycle**
| Byte code | opeartion |
|:---:|:---:|
| 0xB0 | MOV AL, !Next Ins |
| 0xB1 | MOV CL, !Next Ins |
| 0xB2 | MOV DL, !Next Ins |
| 0xB3 | MOV BL, !Next Ins |
| 0xB4 | MOV AH, !Next Ins |
| 0xB5 | MOV CH, !Next Ins |
| 0xB6 | MOV DH, !Next Ins |
| 0xB7 | MOV BH, !Next Ins |
| 0xB8 | MOV AX, !Next 2 Ins |
| 0xB9 | MOV CX, !Next 2 Ins |
| 0xBA | MOV DX, !Next 2 Ins |
| 0xBB | MOV BX, !Next 2 Ins |
| 0xBC | MOV SP, !Next 2 Ins |
| 0xBD | MOV BP, !Next 2 Ins |
| 0xBE | MOV SI, !Next 2 Ins |
| 0xBF | MOV DI, !Next 2 Ins |
