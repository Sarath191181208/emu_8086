"use strict";(self.webpackChunkdocs=self.webpackChunkdocs||[]).push([[5784],{7898:(e,t,n)=>{n.d(t,{PM:()=>d,PP:()=>g,Py:()=>c,VY:()=>i,W7:()=>u,YH:()=>E,kp:()=>p,lh:()=>o,mL:()=>m,qT:()=>h,vR:()=>s});var r=n(7294),a=n(614),l=n(3612);function i(e){return r.createElement(r.Fragment,null,r.createElement("p",null,"This is a mode in which the operand is specified in the instruction itself. The operand is either a general-purpose register or a segment register."),r.createElement("p",null,"Example:"),r.createElement(a.Z,{language:"asm6502",showLineNumbers:!0},`${e.instructionName} AX, BX \n${e.instructionName} BL, CH`),r.createElement("hr",null))}function s(e){return r.createElement(r.Fragment,null,r.createElement("p",null,"This is a mode in which the operand is specified in the instruction itself. The operand is a constant."),r.createElement("p",null,"Example:"),r.createElement(a.Z,{language:"asm6502",showLineNumbers:!0},`${e.instructionName} AX, 0FFh \n${e.instructionName} BX, 0Bh  \n${e.instructionName} CL, 0h`),r.createElement("hr",null))}function d(e){return r.createElement(r.Fragment,null,r.createElement("p",null,"This is a mode in which the operand is specified in the instruction itself. The operand is a memory location."),r.createElement("p",null,"Example:"),r.createElement(a.Z,{language:"asm6502"},e.instructionName," AX, [0x100] ",r.createElement("br",null),e.instructionName," AL, b.[0x100] ",r.createElement("br",null)),"(or) Alternatively you can specify the memory location using the `variable assignment`",r.createElement("p",null,"Example"),r.createElement(a.Z,{language:"asm6502",showLineNumbers:!0},`ORG 100h \n.DATA\n  VAR1 DB 0FFh\n  VAR2 DW 0Bh\nCODE:\n  ${e.instructionName} AL, Var1 \n  ${e.instructionName} BX, Var2 \n  ${e.instructionName} CL, b.[Var2] \n  ${e.instructionName} DX, w.[Var1]`),r.createElement("hr",null))}function m(e){return r.createElement(r.Fragment,null,r.createElement("p",null,"This is a mode in which the operand is specified in the instruction itself. The operand is a memory location whose address is contained in a register."),r.createElement("p",null,"Example:"),r.createElement(a.Z,{language:"asm6502"},e.instructionName," AX, [BX] ",r.createElement("br",null),e.instructionName," AL, b.[BX] ",r.createElement("br",null)),r.createElement("hr",null))}function c(e){return r.createElement(r.Fragment,null,r.createElement("p",null,"This is a mode in which the operand is specified in the instruction itself. The operand is a memory location whose address is contained in a register and an offset."),r.createElement("p",null,"Example:"),r.createElement(a.Z,{language:"asm6502"},e.instructionName," AX, [BX+SI] ",r.createElement("br",null),e.instructionName," AL, b.[BX+SI] ",r.createElement("br",null)),r.createElement("hr",null))}function o(e){return r.createElement(r.Fragment,null,r.createElement("p",null,"This is a mode in which the operand is specified in the instruction itself. The operand is a memory location whose address is contained in a register and an offset and a displacement."),r.createElement("p",null,"Example:"),r.createElement(a.Z,{language:"asm6502"},e.instructionName," AX, [BX+SI+10h] ",r.createElement("br",null),e.instructionName," AX, [BX+SI+100h] ",r.createElement("br",null),e.instructionName," AL, b.[BX+SI+10h] ",r.createElement("br",null),e.instructionName," AL, b.[BX+SI+100h] ",r.createElement("br",null)),r.createElement("hr",null))}function u(e){return r.createElement(r.Fragment,null,r.createElement("p",null,"This is a mode in which the operand is specified as a register while the other operand is specified as a memory location. The second operand is a memory location whose address is contained in a register (or) an offset. The second operand might also have an displacement."),r.createElement("p",null,"Example: "),r.createElement(a.Z,{language:"asm6502",showLineNumbers:!0},`org 100h\ndata\n    VAR DB 0FFh\n    VAR2 DW 0Bh\ncode:\n    ${e.instructionName} BX, Var\n    ${e.instructionName} BX, w.[Var]\n    ${e.instructionName} BX, w.[Var2]\n    ${e.instructionName} AX, [BX]\n    ${e.instructionName} DX, [BX+SI]\n    ${e.instructionName} SP, [BX+SI+10h]\n    ${e.instructionName} DI, [BX+SI+100h]\n`))}function g(e){return r.createElement(r.Fragment,null,r.createElement("p",null,"This is a mode in which the operand is specified as a memory location while the other operand is specified as a register. The first operand is a memory location whose address is contained in a register (or) an offset. The first operand might also have an displacement."),r.createElement("p",null,"Example: "),r.createElement(a.Z,{language:"asm6502",showLineNumbers:!0},`org 100h\ndata\n    VAR DB 0FFh\n    VAR2 DW 0Bh\ncode:\n    ${e.instructionName} Var, BX\n    ${e.instructionName} w.[Var], BX\n    ${e.instructionName} w.[Var2], BX\n    ${e.instructionName} [BX], AX\n    ${e.instructionName} [BX+SI], DX\n    ${e.instructionName} [BX+SI+10h], SP\n    ${e.instructionName} [BX+SI+100h], DI\n`))}function h(e){return r.createElement(l.Z,{type:"danger",title:"Warning, This feature is still not supported."},r.createElement("p",null,"This is a mode in which the operand is specified as a memory location while the other operand is specified as a constant. The first operand is a memory location whose address is contained in a register (or) an offset. The first operand might also have an displacement."),r.createElement("p",null,"Example: "),r.createElement(a.Z,{language:"asm6502",showLineNumbers:!0},`${e.instructionName} [BX], 0FFh\n${e.instructionName} [BX+SI], 0FFh\n${e.instructionName} [BX+SI+10h], 0FFh\n${e.instructionName} [BX+SI+100h], 0FFh\n`))}function p(e){return r.createElement(r.Fragment,null,r.createElement("p",null,"This is a mode in which the operand is specified as a memory location while the other operand is specified as a constant. The first operand is a memory location whose address is specified directly in the instruction."),r.createElement("p",null,"Example: "),r.createElement(a.Z,{language:"asm6502",showLineNumbers:!0},`${e.instructionName} [BX], 0FFh\n${e.instructionName} [BX+SI], 0FFh\n${e.instructionName} [BX+SI+10h], 0FFh\n${e.instructionName} [BX+SI+100h], 0FFh\n`))}function E(e){return r.createElement("table",null,r.createElement("thead",null,r.createElement("tr",null,r.createElement("th",null,"Operand"),r.createElement("th",null,"Opcode"),r.createElement("th",null,"Size(bytes)"),r.createElement("th",null,"Example instruction"))),r.createElement("tbody",null,r.createElement("tr",null,r.createElement("td",null,"reg16, indirect mem"),r.createElement("td",null,e.reg_16bit_and_anything_ins," 0x00..=0x3F"),r.createElement("td",null,"2"),r.createElement("td",null,`${e.instructionName} AX, [BX]`)),r.createElement("tr",null,r.createElement("td",null,"reg16, direct mem"),r.createElement("td",null,e.reg_16bit_and_anything_ins," ","0x06 | reg_idx << 3"),r.createElement("td",null,"4"),r.createElement("td",null,`${e.instructionName} DX, [0x100]`)),r.createElement("tr",null,r.createElement("td",null,"reg16, indirect mem with 8bit offset"),r.createElement("td",null,e.reg_16bit_and_anything_ins," 0x40..=0x7F 0x00..=0xFF"),r.createElement("td",null,"3"),r.createElement("td",null,`${e.instructionName} AX, [BX+0x10]`)),r.createElement("tr",null,r.createElement("td",null,"reg16, indirect mem with 16bit offset"),r.createElement("td",null,e.reg_16bit_and_anything_ins," 0x80..=0xBF 0x00..=0xFF"),r.createElement("td",null,"4"),r.createElement("td",null,`${e.instructionName} AX, [BX+0x100]`)),r.createElement("tr",null,r.createElement("td",null,"reg16, reg16"),r.createElement("td",null,e.reg_16bit_and_anything_ins," 0xC0..=0xFF"),r.createElement("td",null,"2"),r.createElement("td",null,`${e.instructionName} AX, BX`)),r.createElement("tr",null,r.createElement("td",null,"reg8, indirect mem"),r.createElement("td",null,e.reg_8bit_and_anything_ins," 0x00..=0x3F"),r.createElement("td",null,"2"),r.createElement("td",null,`${e.instructionName} AL, [BX]`)),r.createElement("tr",null,r.createElement("td",null,"reg8, direct mem"),r.createElement("td",null,e.reg_8bit_and_anything_ins," ","0x06 | reg_idx << 3"),r.createElement("td",null,"4"),r.createElement("td",null,`${e.instructionName} CL, [0x100]`)),r.createElement("tr",null,r.createElement("td",null,"reg8, indirect mem with 8bit offset"),r.createElement("td",null,e.reg_8bit_and_anything_ins," 0x40..=0x7F 0x00..=0xFF"),r.createElement("td",null,"3"),r.createElement("td",null,`${e.instructionName} AL, [BX+0x10]`)),r.createElement("tr",null,r.createElement("td",null,"reg8, indirect mem with 16bit offset"),r.createElement("td",null,e.reg_8bit_and_anything_ins," 0x80..=0xBF 0x00..=0xFF"),r.createElement("td",null,"4"),r.createElement("td",null,`${e.instructionName} AL, [BX+0x100]`)),r.createElement("tr",null,r.createElement("td",null,"reg8, reg8"),r.createElement("td",null,e.reg_8bit_and_anything_ins," 0xC0..=0xFF"),r.createElement("td",null,"2"),r.createElement("td",null,`${e.instructionName} AL, BL`)),r.createElement("tr",null,r.createElement("td",null,"indirect mem, reg 16"),r.createElement("td",null,e.indexed_addressing_and_anyting_ins," 0x00..=0x3F"),r.createElement("td",null,"2"),r.createElement("td",null,`${e.instructionName} [BX+SI], AX`)),r.createElement("tr",null,r.createElement("td",null,"direct mem, reg 16"),r.createElement("td",null,e.indexed_addressing_and_anyting_ins," ","0x06 | reg_idx << 3"),r.createElement("td",null,"4"),r.createElement("td",null,`${e.instructionName} [0x100], BP`)),r.createElement("tr",null,r.createElement("td",null,"indirect mem with 8bit offset, reg 16"),r.createElement("td",null,e.indexed_addressing_and_anyting_ins," 0x40..=0x7F 0x00..=0xFF"),r.createElement("td",null,"3"),r.createElement("td",null,`${e.instructionName} [BX+SI+0x10], CX`)),r.createElement("tr",null,r.createElement("td",null,"indirect mem with 16bit offset, reg 16"),r.createElement("td",null,e.indexed_addressing_and_anyting_ins," 0x80..=0xBF 0x00..=0xFF"),r.createElement("td",null,"4"),r.createElement("td",null,`${e.instructionName} [BX+SI+0x100], DX`)),r.createElement("tr",null,r.createElement("td",null,"direct mem, reg 8"),r.createElement("td",null,e.addr_and_8bit_reg," ","0x06 | reg_idx << 3"),r.createElement("td",null,"4"),r.createElement("td",null,`${e.instructionName} [0x100], AL`)),r.createElement("tr",null,r.createElement("td",null,"AL, num"),r.createElement("td",null,e.al_and_num_ins," 0x00..=0xFF 0x00..=0xFF"),r.createElement("td",null,"2"),r.createElement("td",null,`${e.instructionName} AL, 0x10`)),r.createElement("tr",null,r.createElement("td",null,"AX, num"),r.createElement("td",null,e.ax_and_num_ins," 0x00..=0xFF 0x00..=0xFF"),r.createElement("td",null,"3"),r.createElement("td",null,`${e.instructionName} AX, 0x100`)),r.createElement("tr",null,r.createElement("td",null,"reg16, num16"),r.createElement("td",null,e.reg16bit_and_16bit_num," ",e.reg_num_sub_ins,"+reg_idx 0x00..=0xFF 0x00..=0xFF"),r.createElement("td",null,"3"),r.createElement("td",null,`${e.instructionName} DX, 0x100`)),r.createElement("tr",null,r.createElement("td",null,"reg16, num8"),r.createElement("td",null,e.reg16bit_and_8bit_num," ",e.reg_num_sub_ins,"+reg_idx 0x00..=0xFF"),r.createElement("td",null,"2"),r.createElement("td",null,`${e.instructionName} CX, 0x10`)),r.createElement("tr",null,r.createElement("td",null,"reg8, num"),r.createElement("td",null,e.reg8bit_and_num," ",e.reg_num_sub_ins,"+reg_idx 0x00..=0xFF"),r.createElement("td",null,"2"),r.createElement("td",null,`${e.instructionName} AL, 0x10`)),r.createElement("tr",null,r.createElement("td",null,"direct address, num16"),r.createElement("td",null,e.addr16bit_and_16bit_num," ",e.addr_num_sub_ins,"+reg_idx"," ","[0x00..=0xFF 0x00..=0xFF] 0x00..=0xFF 0x00..=0xFF"),r.createElement("td",null,"6"),r.createElement("td",null,`${e.instructionName} [0x100], 0x100`)),r.createElement("tr",null,r.createElement("td",null,"direct address, num8"),r.createElement("td",null,e.addr16bit_and_16bit_num," ",e.addr_num_sub_ins,"+reg_idx"," ","[0x00..=0xFF 0x00..=0xFF] 0x00..=0xFF"," "),r.createElement("td",null,"5"),r.createElement("td",null,`${e.instructionName} [0x100], 0x10`)),r.createElement("tr",null,r.createElement("td",null,"addr8, num"),r.createElement("td",null,e.addr8bit_and_num," ",e.addr_num_sub_ins,"+reg_idx [0x00..=0xFF 0x00..=0xFF] 0x00..=0xFF"),r.createElement("td",null,"2"),r.createElement("td",null,`${e.instructionName} b.[0x100], 0x10`))))}},3749:(e,t,n)=>{n.d(t,{Z:()=>s});var r=n(7294);const a={tooltip:"tooltip_OfI_",tooltiptext:"tooltiptext_kMd9",bottomTooltiptext:"bottomTooltiptext_PBXK"};function l(e){let{children:t,text:n,toolTipPosition:l="top"}=e;const i="top"===l?a.tooltiptext:a.bottomTooltiptext;return r.createElement("div",{className:a.tooltip},t,r.createElement("span",{className:i},n))}const i=e=>{let{state:t}=e;return"changed"===t?r.createElement("span",{className:"badge badge--primary badge--rounded h-20"},r.createElement("span",{className:"badge__text"},r.createElement(l,{text:"Changes"},"C"))):"unchanged"===t?r.createElement("span",{className:"badge badge--secondary badge--rounded"},r.createElement("span",{className:"badge__text"},r.createElement(l,{text:"Doesn't change"},"NC"))):1==t?r.createElement("span",{className:"badge badge--success badge--rounded"},r.createElement("span",{className:"badge__text"},r.createElement(l,{text:"Changed to 1"},"1"))):0==t?r.createElement("span",{className:"badge badge--danger badge--rounded"},r.createElement("span",{className:"badge__text"},r.createElement(l,{text:"Changed to 0"},"0"))):r.createElement("span",{className:"badge badge--warning badge--rounded"},r.createElement("span",{className:"badge__text"},"Unknown"))};function s(e){let{carryFlag:t=null,zeroFlag:n=null,signFlag:a=null,overflowFlag:s=null,parityFlag:d=null,auxiliaryCarryFlag:m=null}=e;const c={carryFlag:t,zeroFlag:n,signFlag:a,overflowFlag:s,parityFlag:d,auxiliaryCarryFlag:m};return r.createElement("table",{className:"table table--striped table--responsive flags_table"},r.createElement("thead",null,r.createElement("tr",null,Object.entries(c).map((e=>{let[t,n]=e;return null==n?null:r.createElement("th",null,r.createElement(l,{text:t,toolTipPosition:"bottom"},t[0].toUpperCase()))})))),r.createElement("tbody",null,r.createElement("tr",null,Object.values(c).map((e=>null==e?null:r.createElement("td",null,r.createElement(i,{state:e})))))))}},3520:(e,t,n)=>{n.r(t),n.d(t,{assets:()=>c,contentTitle:()=>d,default:()=>h,frontMatter:()=>s,metadata:()=>m,toc:()=>o});var r=n(7462),a=(n(7294),n(3905)),l=n(3749),i=n(7898);const s={title:"OR",description:"Performs a logical OR of the first operand with the second operand and stores the result in the first operand."},d=void 0,m={unversionedId:"Instructions/OR",id:"Instructions/OR",title:"OR",description:"Performs a logical OR of the first operand with the second operand and stores the result in the first operand.",source:"@site/docs/Instructions/OR.mdx",sourceDirName:"Instructions",slug:"/Instructions/OR",permalink:"/emu_8086/docs/Instructions/OR",draft:!1,editUrl:"https://github.com/Sarath191181208/emu_8086/tree/main/docusaurus/docs/Instructions/OR.mdx",tags:[],version:"current",frontMatter:{title:"OR",description:"Performs a logical OR of the first operand with the second operand and stores the result in the first operand."},sidebar:"tutorialSidebar",previous:{title:"MOV",permalink:"/emu_8086/docs/Instructions/MOV"},next:{title:"SBB",permalink:"/emu_8086/docs/Instructions/SBB"}},c={},o=[{value:"Syntax",id:"syntax",level:2},{value:"Attributes",id:"attributes",level:2},{value:"Example",id:"example",level:2},{value:"Supported Modes",id:"supported-modes",level:2},{value:"Register, Register Addressing",id:"register-register-addressing",level:3},{value:"Register, Immediate Value Addressing",id:"register-immediate-value-addressing",level:3},{value:"Register, Direct Memory Addressing",id:"register-direct-memory-addressing",level:3},{value:"Register, Direct/Indirect Memory Addressing With (or) Without Displacement",id:"register-directindirect-memory-addressing-with-or-without-displacement",level:3},{value:"Direct/Indirect Memory Addressing With (or) Without Displacement , Register Addressing",id:"directindirect-memory-addressing-with-or-without-displacement--register-addressing",level:3},{value:"Direct/Indirect, Immediate Value Addressing",id:"directindirect-immediate-value-addressing",level:3},{value:"Indirect Memory Addressing With (or) Without Displacement, Immediate Value Addressing",id:"indirect-memory-addressing-with-or-without-displacement-immediate-value-addressing",level:3}],u={toc:o},g="wrapper";function h(e){let{components:t,...n}=e;return(0,a.kt)(g,(0,r.Z)({},u,n,{components:t,mdxType:"MDXLayout"}),(0,a.kt)("h2",{id:"syntax"},"Syntax"),(0,a.kt)("pre",null,(0,a.kt)("code",{parentName:"pre",className:"language-asm6502"},"OR operand1, operand2\n")),(0,a.kt)("h2",{id:"attributes"},"Attributes"),(0,a.kt)("table",null,(0,a.kt)("thead",{parentName:"table"},(0,a.kt)("tr",{parentName:"thead"},(0,a.kt)("th",{parentName:"tr",align:"left"},"Attribute"),(0,a.kt)("th",{parentName:"tr",align:"left"},"Description"))),(0,a.kt)("tbody",{parentName:"table"},(0,a.kt)("tr",{parentName:"tbody"},(0,a.kt)("td",{parentName:"tr",align:"left"},(0,a.kt)("strong",{parentName:"td"},"Operands")),(0,a.kt)("td",{parentName:"tr",align:"left"},"The first operand is an general purpose register or a memory location. The second operand is a general-purpose register, memory location, or immediate value.")),(0,a.kt)("tr",{parentName:"tbody"},(0,a.kt)("td",{parentName:"tr",align:"left"},(0,a.kt)("strong",{parentName:"td"},"Operation")),(0,a.kt)("td",{parentName:"tr",align:"left"},(0,a.kt)("inlineCode",{parentName:"td"},"operand1 = operand1 OR operand2"))),(0,a.kt)("tr",{parentName:"tbody"},(0,a.kt)("td",{parentName:"tr",align:"left"},(0,a.kt)("strong",{parentName:"td"},"Size")),(0,a.kt)("td",{parentName:"tr",align:"left"},"2 to 6 bytes.")),(0,a.kt)("tr",{parentName:"tbody"},(0,a.kt)("td",{parentName:"tr",align:"left"},(0,a.kt)("strong",{parentName:"td"},"Action")),(0,a.kt)("td",{parentName:"tr",align:"left"},"Performs a logical OR of the first operand with the second operand and stores the result in the first operand.")),(0,a.kt)("tr",{parentName:"tbody"},(0,a.kt)("td",{parentName:"tr",align:"left"},(0,a.kt)("strong",{parentName:"td"},"Flags")),(0,a.kt)("td",{parentName:"tr",align:"left"},(0,a.kt)(l.Z,{carryFlag:!1,overflowFlag:!1,zeroFlag:"changed",signFlag:"changed",parityFlag:"changed",auxiliaryCarryFlag:"changed",mdxType:"FlagsChangedTable"}))))),(0,a.kt)("h2",{id:"example"},"Example"),(0,a.kt)("pre",null,(0,a.kt)("code",{parentName:"pre",className:"language-asm6502",metastring:"showLineNumbers",showLineNumbers:!0},"MOV AX, 0x0001\nOR AX, 0x0002\n")),(0,a.kt)("blockquote",null,(0,a.kt)("p",{parentName:"blockquote"},"Here the value of ",(0,a.kt)("inlineCode",{parentName:"p"},"AX")," is ",(0,a.kt)("inlineCode",{parentName:"p"},"0x0003")," after the ",(0,a.kt)("inlineCode",{parentName:"p"},"OR")," instruction is executed. ",(0,a.kt)("br",null),"\nThis is because 0x0001 = 0b0000_0000_0000_0001 and 0x0002 = 0b0000_0000_0000_0010. ",(0,a.kt)("br",null),"\nThe ",(0,a.kt)("inlineCode",{parentName:"p"},"OR")," operation is performed on each bit of the operands.",(0,a.kt)("br",null),"\nThe result is 0b0000_0000_0000_0011 which is 0x0003.")),(0,a.kt)("h2",{id:"supported-modes"},"Supported Modes"),(0,a.kt)("h3",{id:"register-register-addressing"},"Register, Register Addressing"),(0,a.kt)(i.VY,{instructionName:"OR",mdxType:"RegisterAddressingMode"}),(0,a.kt)("h3",{id:"register-immediate-value-addressing"},"Register, Immediate Value Addressing"),(0,a.kt)(i.vR,{instructionName:"OR",mdxType:"ImmediateAddressingMode"}),(0,a.kt)("h3",{id:"register-direct-memory-addressing"},"Register, Direct Memory Addressing"),(0,a.kt)(i.PM,{instructionName:"OR",mdxType:"MemoryAddressingMode"}),(0,a.kt)("h3",{id:"register-directindirect-memory-addressing-with-or-without-displacement"},"Register, Direct/Indirect Memory Addressing With (or) Without Displacement"),(0,a.kt)(i.W7,{instructionName:"OR",mdxType:"RegisterAndMemoryAddressing"}),(0,a.kt)("h3",{id:"directindirect-memory-addressing-with-or-without-displacement--register-addressing"},"Direct/Indirect Memory Addressing With (or) Without Displacement , Register Addressing"),(0,a.kt)(i.PP,{instructionName:"OR",mdxType:"MemoryAndRegisterAddressing"}),(0,a.kt)("h3",{id:"directindirect-immediate-value-addressing"},"Direct/Indirect, Immediate Value Addressing"),(0,a.kt)(i.kp,{instructionName:"OR",mdxType:"DirectMemoryAndImmediateAddressing"}),(0,a.kt)("h3",{id:"indirect-memory-addressing-with-or-without-displacement-immediate-value-addressing"},"Indirect Memory Addressing With (or) Without Displacement, Immediate Value Addressing"),(0,a.kt)(i.qT,{instructionName:"OR",mdxType:"IndirectMemoryAndImmediateAddressing"}))}h.isMDXComponent=!0}}]);