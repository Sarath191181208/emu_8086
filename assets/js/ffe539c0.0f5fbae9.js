"use strict";(self.webpackChunkdocs=self.webpackChunkdocs||[]).push([[2370],{7498:(t,e,n)=>{n.r(e),n.d(e,{assets:()=>d,contentTitle:()=>s,default:()=>N,frontMatter:()=>l,metadata:()=>o,toc:()=>p});var a=n(7462),i=(n(7294),n(3905)),r=n(7898);const l={title:"JNA",description:"Jump if Carry Flag is 1 (or) Zero Flag is 1."},s=void 0,o={unversionedId:"Instructions/JNA",id:"Instructions/JNA",title:"JNA",description:"Jump if Carry Flag is 1 (or) Zero Flag is 1.",source:"@site/docs/Instructions/JNA.mdx",sourceDirName:"Instructions",slug:"/Instructions/JNA",permalink:"/emu_8086/docs/Instructions/JNA",draft:!1,editUrl:"https://github.com/Sarath191181208/emu_8086/tree/main/docusaurus/docs/Instructions/JNA.mdx",tags:[],version:"current",frontMatter:{title:"JNA",description:"Jump if Carry Flag is 1 (or) Zero Flag is 1."},sidebar:"tutorialSidebar",previous:{title:"JLE",permalink:"/emu_8086/docs/Instructions/JLE"},next:{title:"JNAE",permalink:"/emu_8086/docs/Instructions/JNAE"}},d={},p=[{value:"Syntax",id:"syntax",level:2},{value:"Attributes",id:"attributes",level:2},{value:"Example",id:"example",level:2},{value:"Supported Modes",id:"supported-modes",level:2},{value:"Label/ 8bit constant",id:"label-8bit-constant",level:3}],u={toc:p},m="wrapper";function N(t){let{components:e,...n}=t;return(0,i.kt)(m,(0,a.Z)({},u,n,{components:e,mdxType:"MDXLayout"}),(0,i.kt)("h2",{id:"syntax"},"Syntax"),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre",className:"language-asm6502",metastring:"showLineNumbers",showLineNumbers:!0},"    JNA label\n    JNA 0x02 ; 8bit constant\n")),(0,i.kt)("h2",{id:"attributes"},"Attributes"),(0,i.kt)("table",null,(0,i.kt)("thead",{parentName:"table"},(0,i.kt)("tr",{parentName:"thead"},(0,i.kt)("th",{parentName:"tr",align:null},"Attribute"),(0,i.kt)("th",{parentName:"tr",align:null},"Description"))),(0,i.kt)("tbody",{parentName:"table"},(0,i.kt)("tr",{parentName:"tbody"},(0,i.kt)("td",{parentName:"tr",align:null},(0,i.kt)("strong",{parentName:"td"},"Operands")),(0,i.kt)("td",{parentName:"tr",align:null},(0,i.kt)("inlineCode",{parentName:"td"},"label")," - A label to jump to. ",(0,i.kt)("br",null)," ",(0,i.kt)("inlineCode",{parentName:"td"},"0x02")," - An 8bit constant to jump to.")),(0,i.kt)("tr",{parentName:"tbody"},(0,i.kt)("td",{parentName:"tr",align:null},(0,i.kt)("strong",{parentName:"td"},"Operation")),(0,i.kt)("td",{parentName:"tr",align:null},(0,i.kt)("inlineCode",{parentName:"td"},"JMP if CF = 1 (or) ZF = 1"))),(0,i.kt)("tr",{parentName:"tbody"},(0,i.kt)("td",{parentName:"tr",align:null},(0,i.kt)("strong",{parentName:"td"},"Size")),(0,i.kt)("td",{parentName:"tr",align:null},"8bit-ins: 2 bytes, 16bit-ins: 5 bytes")),(0,i.kt)("tr",{parentName:"tbody"},(0,i.kt)("td",{parentName:"tr",align:null},(0,i.kt)("strong",{parentName:"td"},"Action")),(0,i.kt)("td",{parentName:"tr",align:null},"Jump to the specified label if Carry Flag is set (or) Zero Flag is set. Generally used in conjunction with the ",(0,i.kt)("inlineCode",{parentName:"td"},"CMP")," instruction.")),(0,i.kt)("tr",{parentName:"tbody"},(0,i.kt)("td",{parentName:"tr",align:null},(0,i.kt)("strong",{parentName:"td"},"Flags")),(0,i.kt)("td",{parentName:"tr",align:null},"No change")))),(0,i.kt)("h2",{id:"example"},"Example"),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre",className:"language-asm6502",metastring:"showLineNumbers",showLineNumbers:!0},"MOV AL, 250\nCMP AL, 5\nJNA label1\n\n;AL is not above 5\nJMP exit\n\nlabel1:\n    ;AL is above 5\nexit:\n   RET\n")),(0,i.kt)("h2",{id:"supported-modes"},"Supported Modes"),(0,i.kt)("h3",{id:"label-8bit-constant"},"Label/ 8bit constant"),(0,i.kt)(r.zJ,{instructionName:"JNA",mdxType:"LabeledInstructionAddressinng"}),(0,i.kt)("admonition",{title:"Info",type:"info"},(0,i.kt)("p",{parentName:"admonition"},"There isn't a 16bit constant/label mode for this instruction. But you can use the 8bit constant mode and add a ",(0,i.kt)("inlineCode",{parentName:"p"},"JMP")," instruction after it."),(0,i.kt)("p",{parentName:"admonition"},"ex: ",(0,i.kt)("inlineCode",{parentName:"p"},"JNA 0x100")," is equivalent to ",(0,i.kt)("inlineCode",{parentName:"p"},"JNBE 0x03")," + ",(0,i.kt)("inlineCode",{parentName:"p"},"JMP 0x100"),". Here 0x03 is the size of the 16bit jmp instruction ","[0xE9, 0x00, 0x01]",".")))}N.isMDXComponent=!0}}]);