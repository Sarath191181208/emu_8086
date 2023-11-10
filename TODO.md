## Bugs: 
- [ ] 🐞: jmp to number 
- [ ] 🐞: not optimizing reg and num in add ins
- [ ] 🐞:  Running of macros.
- [ ] 🐞: Data in memeory doesn't change after second compilation.
- [ ] 🐞: Look into the .CODE crash
- [ ] 

## Warnings: 
- [ ] For same parameter names in macro definitions.
- [ ] Unused macros.
## To support: 

- [ ] Better error messages.
- [ ] Support Some `interrupts`.
- [ ] inline macro editor fn.
- [ ] creating a formatter.
- [ ] Change interrupt flag name to `INTERRUPT_FLAG`.
- [ ] 🛠️ Refactor:  Make the react ui composable .
- [ ] Create a commmit interface for timestamp making mem and cpu work easily.
- [ ] Look into why CX is loded with some value 
- [ ] Setting up a CI/CD pipeline for frontend.
- [=] Implement `PROC` work.
  - [ ] Make the `HLT` execution.
- [ ] Adding more addressing modes to two operand instructions.
  - [ ] Support `MOV b.[bx+10], val` addressing.

- [x] Look into aux flag for `or` ins.
- [x] Support `ADC` and `CMP` instructions.
- [x] Support 1234:5778 addressing, [bx+10] addressing for `jmp` ins.
- [x] Support execution of `and` ins.
- [x] `push` and `pop` for `regs`.    
- [x] Support offset.
- [x] Support _ in nums.
- [x] Support vector indexing.
- [x] Support for ctrl + click for procs and macros.
- [x] add support for addressing mode 
- [x] add the labels into the symbol table
- [x] add calc offset in compile itself 
- [x] parse the data segments
- [x] Make `loop` ins work.
- [=] Implement `PROC` work.
  - [x] Make the `RET` compilation.
  - [x] Make the `RET` execution.
  - [x] Make the `CALL` compilation.
  - [x] Make the `CALL` execution.
- [x] Make Different `JMP` work.
- [x] support bin numbers
- [x] support var ptr declearation.
- [ ] Adding more instructions
    - [ ] DIV
    - [x] SUB
    - [x] MUL
    - [x] CMP
    - [x] JMP
    - [x] AND, OR, XOR, NOT
    - [x] CALL, RET
    - [x] PUSH, POP
    - [x] INT
