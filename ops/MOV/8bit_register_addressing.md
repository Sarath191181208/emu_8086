## MOV instruction
```asm
    MOV AL, BH
```

**For next cycle**
| Byte code | opeartion | 
|:---:|:---:|
| 0xC0 | MOV AL, AL |
| 0xC1 | MOV AL, CL |
| 0xC2 | MOV AL, DL |
| 0xC3 | MOV AL, BL |
| 0xC4 | MOV AL, AH |
| 0xC5 | MOV AL, CH |
| 0xC6 | MOV AL, DH |
| 0xC7 | MOV AL, BH |
| 0xC8 | MOV CL, AL |
| 0xC9 | MOV CL, CL |
| 0xCA | MOV CL, DL |
| 0xCB | MOV CL, BL |
| 0xCC | MOV CL, AH |
| 0xCD | MOV CL, CH |
| 0xCE | MOV CL, DH |
| 0xCF | MOV CL, BH |
| 0xD0 | MOV DL, AL |
| 0xD1 | MOV DL, CL |
| 0xD2 | MOV DL, DL |
| 0xD3 | MOV DL, BL |
| 0xD4 | MOV DL, AH |
| 0xD5 | MOV DL, CH |
| 0xD6 | MOV DL, DH |
| 0xD7 | MOV DL, BH |
| 0xD8 | MOV BL, AL |
| 0xD9 | MOV BL, CL |
| 0xDA | MOV BL, DL |
| 0xDB | MOV BL, BL |
| 0xDC | MOV BL, AH |
| 0xDD | MOV BL, CH |
| 0xDE | MOV BL, DH |
| 0xDF | MOV BL, BH |
| 0xE0 | MOV AH, AL |
| 0xE1 | MOV AH, CL |
| 0xE2 | MOV AH, DL |
| 0xE3 | MOV AH, BL |
| 0xE4 | MOV AH, AH |
| 0xE5 | MOV AH, CH |
| 0xE6 | MOV AH, DH |
| 0xE7 | MOV AH, BH |
| 0xE8 | MOV CH, AL |
| 0xE9 | MOV CH, CL |
| 0xEA | MOV CH, DL |
| 0xEB | MOV CH, BL |
| 0xEC | MOV CH, AH |
| 0xED | MOV CH, CH |
| 0xEE | MOV CH, DH |
| 0xEF | MOV CH, BH |
| 0xF0 | MOV DH, AL |
| 0xF1 | MOV DH, CL |
| 0xF2 | MOV DH, DL |
| 0xF3 | MOV DH, BL |
| 0xF4 | MOV DH, AH |
| 0xF5 | MOV DH, CH |
| 0xF6 | MOV DH, DH |
| 0xF7 | MOV DH, BH |
| 0xF8 | MOV BH, AL |
| 0xF9 | MOV BH, CL |
| 0xFA | MOV BH, DL |
| 0xFB | MOV BH, BL |
| 0xFC | MOV BH, AH |
| 0xFD | MOV BH, CH |
| 0xFE | MOV BH, DH |
| 0xFF | MOV BH, BH |

