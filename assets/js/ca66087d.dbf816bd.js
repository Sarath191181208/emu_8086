"use strict";(self.webpackChunkdocs=self.webpackChunkdocs||[]).push([[987],{8486:(t,e,n)=>{n.r(e),n.d(e,{assets:()=>d,contentTitle:()=>o,default:()=>N,frontMatter:()=>l,metadata:()=>s,toc:()=>u});var a=n(7462),i=(n(7294),n(3905)),r=n(7898);const l={title:"JNL",description:"Jump if Sign Flag is equal to Overflow Flag."},o=void 0,s={unversionedId:"Instructions/JNL",id:"Instructions/JNL",title:"JNL",description:"Jump if Sign Flag is equal to Overflow Flag.",source:"@site/docs/Instructions/JNL.mdx",sourceDirName:"Instructions",slug:"/Instructions/JNL",permalink:"/emu_8086/docs/Instructions/JNL",draft:!1,editUrl:"https://github.com/Sarath191181208/emu_8086/tree/main/docusaurus/docs/Instructions/JNL.mdx",tags:[],version:"current",frontMatter:{title:"JNL",description:"Jump if Sign Flag is equal to Overflow Flag."},sidebar:"tutorialSidebar",previous:{title:"JNGE",permalink:"/emu_8086/docs/Instructions/JNGE"},next:{title:"JNLE",permalink:"/emu_8086/docs/Instructions/JNLE"}},d={},u=[{value:"Syntax",id:"syntax",level:2},{value:"Attributes",id:"attributes",level:2},{value:"Example",id:"example",level:2},{value:"Supported Modes",id:"supported-modes",level:2},{value:"Label/ 8bit constant",id:"label-8bit-constant",level:3}],p={toc:u},m="wrapper";function N(t){let{components:e,...n}=t;return(0,i.kt)(m,(0,a.Z)({},p,n,{components:e,mdxType:"MDXLayout"}),(0,i.kt)("h2",{id:"syntax"},"Syntax"),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre",className:"language-asm6502",metastring:"showLineNumbers",showLineNumbers:!0},"    JNL label\n    JNL 0x02 ; 8bit constant\n")),(0,i.kt)("h2",{id:"attributes"},"Attributes"),(0,i.kt)("table",null,(0,i.kt)("thead",{parentName:"table"},(0,i.kt)("tr",{parentName:"thead"},(0,i.kt)("th",{parentName:"tr",align:null},"Attribute"),(0,i.kt)("th",{parentName:"tr",align:null},"Description"))),(0,i.kt)("tbody",{parentName:"table"},(0,i.kt)("tr",{parentName:"tbody"},(0,i.kt)("td",{parentName:"tr",align:null},(0,i.kt)("strong",{parentName:"td"},"Operands")),(0,i.kt)("td",{parentName:"tr",align:null},(0,i.kt)("inlineCode",{parentName:"td"},"label")," - A label to jump to. ",(0,i.kt)("br",null)," ",(0,i.kt)("inlineCode",{parentName:"td"},"0x02")," - An 8bit constant to jump to.")),(0,i.kt)("tr",{parentName:"tbody"},(0,i.kt)("td",{parentName:"tr",align:null},(0,i.kt)("strong",{parentName:"td"},"Operation")),(0,i.kt)("td",{parentName:"tr",align:null},(0,i.kt)("inlineCode",{parentName:"td"},"JMP if SF = OF"))),(0,i.kt)("tr",{parentName:"tbody"},(0,i.kt)("td",{parentName:"tr",align:null},(0,i.kt)("strong",{parentName:"td"},"Size")),(0,i.kt)("td",{parentName:"tr",align:null},"8bit-ins: 2 bytes, 16bit-ins: 5 bytes")),(0,i.kt)("tr",{parentName:"tbody"},(0,i.kt)("td",{parentName:"tr",align:null},(0,i.kt)("strong",{parentName:"td"},"Action")),(0,i.kt)("td",{parentName:"tr",align:null},"Jump to the specified label if Sign Flag and the Overflow Flags are equal. Generally used in conjunction with the ",(0,i.kt)("inlineCode",{parentName:"td"},"CMP")," instruction.")),(0,i.kt)("tr",{parentName:"tbody"},(0,i.kt)("td",{parentName:"tr",align:null},(0,i.kt)("strong",{parentName:"td"},"Flags")),(0,i.kt)("td",{parentName:"tr",align:null},"No change")))),(0,i.kt)("h2",{id:"example"},"Example"),(0,i.kt)("pre",null,(0,i.kt)("code",{parentName:"pre",className:"language-asm6502",metastring:"showLineNumbers",showLineNumbers:!0},"MOV AL, 250\nCMP AL, 5\nJNL label1\n\n;AL is not above 5\nJMP exit\n\nlabel1:\n    ;AL is above 5\nexit:\n   RET\n")),(0,i.kt)("h2",{id:"supported-modes"},"Supported Modes"),(0,i.kt)("h3",{id:"label-8bit-constant"},"Label/ 8bit constant"),(0,i.kt)(r.zJ,{instructionName:"JNL",mdxType:"LabeledInstructionAddressinng"}),(0,i.kt)("admonition",{title:"Info",type:"info"},(0,i.kt)("p",{parentName:"admonition"},"There isn't a 16bit constant/label mode for this instruction. But you can use the 8bit constant mode and add a ",(0,i.kt)("inlineCode",{parentName:"p"},"JMP")," instruction after it."),(0,i.kt)("p",{parentName:"admonition"},"ex: ",(0,i.kt)("inlineCode",{parentName:"p"},"JNL 0x100")," is equivalent to ",(0,i.kt)("inlineCode",{parentName:"p"},"JL 0x03")," + ",(0,i.kt)("inlineCode",{parentName:"p"},"JMP 0x100"),". Here 0x03 is the size of the 16bit jmp instruction ","[0xE9, 0x00, 0x01]",".")))}N.isMDXComponent=!0}}]);