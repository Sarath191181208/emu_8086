"use strict";(self.webpackChunkdocs=self.webpackChunkdocs||[]).push([[1487],{7305:(t,e,n)=>{n.r(e),n.d(e,{assets:()=>d,contentTitle:()=>s,default:()=>c,frontMatter:()=>l,metadata:()=>o,toc:()=>p});var a=n(7462),i=(n(7294),n(3905)),r=n(7898);const l={title:"JAE",description:"Jump if (Carry Flag=0)"},s=void 0,o={unversionedId:"Instructions/JAE",id:"Instructions/JAE",title:"JAE",description:"Jump if (Carry Flag=0)",source:"@site/docs/Instructions/JAE.mdx",sourceDirName:"Instructions",slug:"/Instructions/JAE",permalink:"/emu_8086/docs/Instructions/JAE",draft:!1,editUrl:"https://github.com/Sarath191181208/emu_8086/tree/main/docusaurus/docs/Instructions/JAE.mdx",tags:[],version:"current",frontMatter:{title:"JAE",description:"Jump if (Carry Flag=0)"},sidebar:"tutorialSidebar",previous:{title:"JA",permalink:"/emu_8086/docs/Instructions/JA"},next:{title:"JB",permalink:"/emu_8086/docs/Instructions/JB"}},d={},p=[{value:"Syntax",id:"syntax",level:2},{value:"Attributes",id:"attributes",level:2},{value:"Example",id:"example",level:2},{value:"Supported Modes",id:"supported-modes",level:2},{value:"Label/ 8bit constant",id:"label-8bit-constant",level:3}],u={toc:p},m="wrapper";function c(t){let{components:e,...n}=t;return(0,i.kt)(m,(0,a.Z)({},u,n,{components:e,mdxType:"MDXLayout"}),(0,i.kt)("h2",{id:"syntax"},"Syntax"),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre",className:"language-asm6502",metastring:"showLineNumbers",showLineNumbers:!0},"    JAE label\n    JAE 0x02 ; 8bit constant\n")),(0,i.kt)("h2",{id:"attributes"},"Attributes"),(0,i.kt)("table",null,(0,i.kt)("thead",{parentName:"table"},(0,i.kt)("tr",{parentName:"thead"},(0,i.kt)("th",{parentName:"tr",align:null},"Attribute"),(0,i.kt)("th",{parentName:"tr",align:null},"Description"))),(0,i.kt)("tbody",{parentName:"table"},(0,i.kt)("tr",{parentName:"tbody"},(0,i.kt)("td",{parentName:"tr",align:null},(0,i.kt)("strong",{parentName:"td"},"Operands")),(0,i.kt)("td",{parentName:"tr",align:null},(0,i.kt)("inlineCode",{parentName:"td"},"label")," - A label to jump to. ",(0,i.kt)("br",null)," ",(0,i.kt)("inlineCode",{parentName:"td"},"0x02")," - An 8bit constant to jump to.")),(0,i.kt)("tr",{parentName:"tbody"},(0,i.kt)("td",{parentName:"tr",align:null},(0,i.kt)("strong",{parentName:"td"},"Operation")),(0,i.kt)("td",{parentName:"tr",align:null},(0,i.kt)("inlineCode",{parentName:"td"},"JMP if CF = 0"))),(0,i.kt)("tr",{parentName:"tbody"},(0,i.kt)("td",{parentName:"tr",align:null},(0,i.kt)("strong",{parentName:"td"},"Size")),(0,i.kt)("td",{parentName:"tr",align:null},"8bit-ins: 2 bytes, 16bit-ins: 5 bytes")),(0,i.kt)("tr",{parentName:"tbody"},(0,i.kt)("td",{parentName:"tr",align:null},(0,i.kt)("strong",{parentName:"td"},"Action")),(0,i.kt)("td",{parentName:"tr",align:null},"Jump to the specified label if Carry Flag is clear. Generally used in conjunction with the ",(0,i.kt)("inlineCode",{parentName:"td"},"CMP")," instruction.")),(0,i.kt)("tr",{parentName:"tbody"},(0,i.kt)("td",{parentName:"tr",align:null},(0,i.kt)("strong",{parentName:"td"},"Flags")),(0,i.kt)("td",{parentName:"tr",align:null},"No change")))),(0,i.kt)("h2",{id:"example"},"Example"),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre",className:"language-asm6502",metastring:"showLineNumbers",showLineNumbers:!0},"MOV AL, 250\nCMP AL, 5\nJAE label1\n\n;AL is not above 5\nJMP exit\n\nlabel1:\n    ;AL is above 5\nexit:\n   RET\n")),(0,i.kt)("h2",{id:"supported-modes"},"Supported Modes"),(0,i.kt)("h3",{id:"label-8bit-constant"},"Label/ 8bit constant"),(0,i.kt)(r.zJ,{instructionName:"JAE",mdxType:"LabeledInstructionAddressinng"}),(0,i.kt)("admonition",{title:"Info ",type:"info"},(0,i.kt)("p",{parentName:"admonition"},"There isn't a 16bit constant/label mode for this instruction. But you can use the 8bit constant mode and add a ",(0,i.kt)("inlineCode",{parentName:"p"},"JMP")," instruction after it."),(0,i.kt)("p",{parentName:"admonition"},"ex: ",(0,i.kt)("inlineCode",{parentName:"p"},"JAE 0x100")," is equivalent to ",(0,i.kt)("inlineCode",{parentName:"p"},"JNBE 0x03")," + ",(0,i.kt)("inlineCode",{parentName:"p"},"JMP 0x100"),". Here 0x03 is the size of the 16bit jmp instruction ","[0xE9, 0x00, 0x01]",".")))}c.isMDXComponent=!0}}]);