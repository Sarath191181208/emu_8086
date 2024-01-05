"use strict";(self.webpackChunkdocs=self.webpackChunkdocs||[]).push([[1325],{2489:(t,e,n)=>{n.r(e),n.d(e,{assets:()=>l,contentTitle:()=>s,default:()=>k,frontMatter:()=>o,metadata:()=>d,toc:()=>p});var a=n(7462),r=(n(7294),n(3905)),i=n(7898);const o={title:"LES",description:"Loads the value in the effective address into the destination register & Loads ES with the next word."},s=void 0,d={unversionedId:"Instructions/LES",id:"Instructions/LES",title:"LES",description:"Loads the value in the effective address into the destination register & Loads ES with the next word.",source:"@site/docs/Instructions/LES.mdx",sourceDirName:"Instructions",slug:"/Instructions/LES",permalink:"/emu_8086/docs/Instructions/LES",draft:!1,editUrl:"https://github.com/Sarath191181208/emu_8086/tree/main/docusaurus/docs/Instructions/LES.mdx",tags:[],version:"current",frontMatter:{title:"LES",description:"Loads the value in the effective address into the destination register & Loads ES with the next word."},sidebar:"tutorialSidebar",previous:{title:"LEA",permalink:"/emu_8086/docs/Instructions/LEA"},next:{title:"MOV",permalink:"/emu_8086/docs/Instructions/MOV"}},l={},p=[{value:"Syntax",id:"syntax",level:2},{value:"Attributes",id:"attributes",level:2},{value:"Explanation",id:"explanation",level:2},{value:"Supported Modes",id:"supported-modes",level:2},{value:"Register And Memory",id:"register-and-memory",level:3}],m={toc:p},u="wrapper";function k(t){let{components:e,...n}=t;return(0,r.kt)(u,(0,a.Z)({},m,n,{components:e,mdxType:"MDXLayout"}),(0,r.kt)("h2",{id:"syntax"},"Syntax"),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-asm6502"},"LES operand1, operand2\n")),(0,r.kt)("h2",{id:"attributes"},"Attributes"),(0,r.kt)("table",null,(0,r.kt)("thead",{parentName:"table"},(0,r.kt)("tr",{parentName:"thead"},(0,r.kt)("th",{parentName:"tr",align:null},"Attribute"),(0,r.kt)("th",{parentName:"tr",align:null},"Description"))),(0,r.kt)("tbody",{parentName:"table"},(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:null},(0,r.kt)("strong",{parentName:"td"},"Operands")),(0,r.kt)("td",{parentName:"tr",align:null},"The destination operand is a general-purpose register. The source operand is a memory location.")),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:null},(0,r.kt)("strong",{parentName:"td"},"Operation")),(0,r.kt)("td",{parentName:"tr",align:null},(0,r.kt)("inlineCode",{parentName:"td"},"DEST \u2190 SRC; ES \u2190 SRC+2"))),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:null},(0,r.kt)("strong",{parentName:"td"},"Size")),(0,r.kt)("td",{parentName:"tr",align:null},"2 (to) 6 bytes")),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:null},(0,r.kt)("strong",{parentName:"td"},"Action")),(0,r.kt)("td",{parentName:"tr",align:null},"Performs Memory Indirect Read and puts the value in the memory location into the destination register. Loads ES with the next word.")),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:null},(0,r.kt)("strong",{parentName:"td"},"Flags")),(0,r.kt)("td",{parentName:"tr",align:null},"None")))),(0,r.kt)("h2",{id:"explanation"},"Explanation"),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-asm6502",metastring:"showLineNumbers",showLineNumbers:!0},"MOV [BX]     , 0x1234\nMOV [BX+0x02], 0x5678\nLES AX, [BX] ; AX = 0x1234, ES = 0x5678\n")),(0,r.kt)("p",null,"Here the value at the memory location ",(0,r.kt)("inlineCode",{parentName:"p"},"0x1234")," is loaded into the ",(0,r.kt)("inlineCode",{parentName:"p"},"AX")," register and the value at the memory location ",(0,r.kt)("inlineCode",{parentName:"p"},"0x1236")," is loaded into the ",(0,r.kt)("inlineCode",{parentName:"p"},"ES")," register.\ni.e the values of the registers are as follows : "),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre"},"AX = 0x1234\nES = 0x5678\n")),(0,r.kt)("h2",{id:"supported-modes"},"Supported Modes"),(0,r.kt)("h3",{id:"register-and-memory"},"Register And Memory"),(0,r.kt)(i.W7,{instructionName:"LES",mdxType:"RegisterAndMemoryAddressing"}))}k.isMDXComponent=!0}}]);