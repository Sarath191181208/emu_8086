## MOV instruction
```asm
    MOV AX, BX
```

**For next cycle**
| Byte code | opeartion | 
|:---:|:---:|
| 0xC0 | MOV AX, AX |
| 0xC1 | MOV AX, CX |
| 0xC2 | MOV AX, DX |
| 0xC3 | MOV AX, BX |
| 0xC4 | MOV AX, SP |
| 0xC5 | MOV AX, BP |
| 0xC6 | MOV AX, SI |
| 0xC7 | MOV AX, DI |
| 0xC8 | MOV CX, AX |
| 0xC9 | MOV CX, CX |
| 0xCA | MOV CX, DX |
| 0xCB | MOV CX, BX |
| 0xCC | MOV CX, SP |
| 0xCD | MOV CX, BP |
| 0xCE | MOV CX, SI |
| 0xCF | MOV CX, DI |
| 0xD0 | MOV DX, AX |
| 0xD1 | MOV DX, CX |
| 0xD2 | MOV DX, DX |
| 0xD3 | MOV DX, BX |
| 0xD4 | MOV DX, SP |
| 0xD5 | MOV DX, BP |
| 0xD6 | MOV DX, SI |
| 0xD7 | MOV DX, DI |
| 0xD8 | MOV BX, AX |
| 0xD9 | MOV BX, CX |
| 0xDA | MOV BX, DX |
| 0xDB | MOV BX, BX |
| 0xDC | MOV BX, SP |
| 0xDD | MOV BX, BP |
| 0xDE | MOV BX, SI |
| 0xDF | MOV BX, DI |
| 0xE0 | MOV SP, AX |
| 0xE1 | MOV SP, CX |
| 0xE2 | MOV SP, DX |
| 0xE3 | MOV SP, BX |
| 0xE4 | MOV SP, SP |
| 0xE5 | MOV SP, BP |
| 0xE6 | MOV SP, SI |
| 0xE7 | MOV SP, DI |
| 0xE8 | MOV BP, AX |
| 0xE9 | MOV BP, CX |
| 0xEA | MOV BP, DX |
| 0xEB | MOV BP, BX |
| 0xEC | MOV BP, SP |
| 0xED | MOV BP, BP |
| 0xEE | MOV BP, SI |
| 0xEF | MOV BP, DI |
| 0xF0 | MOV SI, AX |
| 0xF1 | MOV SI, CX |
| 0xF2 | MOV SI, DX |
| 0xF3 | MOV SI, BX |
| 0xF4 | MOV SI, SP |
| 0xF5 | MOV SI, BP |
| 0xF6 | MOV SI, SI |
| 0xF7 | MOV SI, DI |
| 0xF8 | MOV DI, AX |
| 0xF9 | MOV DI, CX |
| 0xFA | MOV DI, DX |
| 0xFB | MOV DI, BX |
| 0xFC | MOV DI, SP |   
| 0xFD | MOV DI, BP |
| 0xFE | MOV DI, SI |
| 0xFF | MOV DI, DI |
