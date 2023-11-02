"use strict";(self.webpackChunkdocs=self.webpackChunkdocs||[]).push([[2375],{5537:(t,e,r)=>{r.r(e),r.d(e,{assets:()=>m,contentTitle:()=>o,default:()=>c,frontMatter:()=>i,metadata:()=>s,toc:()=>p});var a=r(7462),n=(r(7294),r(3905));const i={authors:{name:"Vangipuram Srinivasa Sarath Chandra",title:"Tech enthusiast",url:"https://github.com/Sarath191181208",image_url:"https://avatars.githubusercontent.com/u/74459981?v=4"}},o="Support for and instruction compilation",s={permalink:"/emu_8086/blog/2023/10/30/",editUrl:"https://github.com/Sarath191181208/emu_8086/tree/main/docusaurus/blog/2023-10-30.mdx",source:"@site/blog/2023-10-30.mdx",title:"Support for and instruction compilation",description:"The AND instruction is now compiled and working. The AND instruction is a bitwise AND operation. The AND instruction is used to perform a bitwise AND operation between the first source operand (register or memory location) and the second source operand (register or memory location) and store the result in the destination operand (register or memory location). The AND instruction affects the CF and OF flags when the second operand is an immediate value. The SF, ZF, and PF flags are always set according to the result. The state of the AF flag is undefined.",date:"2023-10-30T00:00:00.000Z",formattedDate:"October 30, 2023",tags:[],readingTime:2.11,hasTruncateMarker:!1,authors:[{name:"Vangipuram Srinivasa Sarath Chandra",title:"Tech enthusiast",url:"https://github.com/Sarath191181208",image_url:"https://avatars.githubusercontent.com/u/74459981?v=4",imageURL:"https://avatars.githubusercontent.com/u/74459981?v=4"}],frontMatter:{authors:{name:"Vangipuram Srinivasa Sarath Chandra",title:"Tech enthusiast",url:"https://github.com/Sarath191181208",image_url:"https://avatars.githubusercontent.com/u/74459981?v=4",imageURL:"https://avatars.githubusercontent.com/u/74459981?v=4"}},prevItem:{title:"\ud83d\udee0\ufe0f Refactor and Exec of AND",permalink:"/emu_8086/blog/2023/10/31/"},nextItem:{title:"\ud83d\udee0\ufe0f Refactor: moved common logic into a pattern",permalink:"/emu_8086/blog/2023/10/29/"}},m={authorsImageUrls:[void 0]},p=[],d={toc:p},l="wrapper";function c(t){let{components:e,...r}=t;return(0,n.kt)(l,(0,a.Z)({},d,r,{components:e,mdxType:"MDXLayout"}),(0,n.kt)("p",null,"The ",(0,n.kt)("inlineCode",{parentName:"p"},"AND")," instruction is now compiled and working. The ",(0,n.kt)("inlineCode",{parentName:"p"},"AND")," instruction is a bitwise AND operation. The ",(0,n.kt)("inlineCode",{parentName:"p"},"AND")," instruction is used to perform a bitwise AND operation between the first source operand (register or memory location) and the second source operand (register or memory location) and store the result in the destination operand (register or memory location). The ",(0,n.kt)("inlineCode",{parentName:"p"},"AND")," instruction affects the CF and OF flags when the second operand is an immediate value. The SF, ZF, and PF flags are always set according to the result. The state of the AF flag is undefined.\nThe compilation table of the ",(0,n.kt)("inlineCode",{parentName:"p"},"AND")," instruction is as follows:"),(0,n.kt)("table",null,(0,n.kt)("thead",{parentName:"table"},(0,n.kt)("tr",{parentName:"thead"},(0,n.kt)("th",{parentName:"tr",align:"center"},"Operand"),(0,n.kt)("th",{parentName:"tr",align:"center"},"Opcode"),(0,n.kt)("th",{parentName:"tr",align:"center"},"Size(bytes)"),(0,n.kt)("th",{parentName:"tr",align:"center"},"Description"),(0,n.kt)("th",{parentName:"tr",align:null},"Example instruction"))),(0,n.kt)("tbody",{parentName:"table"},(0,n.kt)("tr",{parentName:"tbody"},(0,n.kt)("td",{parentName:"tr",align:"center"},"reg16, reg16"),(0,n.kt)("td",{parentName:"tr",align:"center"},"0x21"),(0,n.kt)("td",{parentName:"tr",align:"center"},"2"),(0,n.kt)("td",{parentName:"tr",align:"center"},"Performs bitwise AND operation between the two operands and stores the result in the first operand"),(0,n.kt)("td",{parentName:"tr",align:null},"AND AX, BX")),(0,n.kt)("tr",{parentName:"tbody"},(0,n.kt)("td",{parentName:"tr",align:"center"},"reg8, reg8"),(0,n.kt)("td",{parentName:"tr",align:"center"},"0x20"),(0,n.kt)("td",{parentName:"tr",align:"center"},"2"),(0,n.kt)("td",{parentName:"tr",align:"center"},"Performs bitwise AND operation between the two operands and stores the result in the first operand"),(0,n.kt)("td",{parentName:"tr",align:null},"AND AL, BL")),(0,n.kt)("tr",{parentName:"tbody"},(0,n.kt)("td",{parentName:"tr",align:"center"},"reg16, mem16"),(0,n.kt)("td",{parentName:"tr",align:"center"},"0x23"),(0,n.kt)("td",{parentName:"tr",align:"center"},"2"),(0,n.kt)("td",{parentName:"tr",align:"center"},"Performs bitwise AND operation between the two operands and stores the result in the first operand"),(0,n.kt)("td",{parentName:"tr",align:null},"AND AX, ","[BX]")),(0,n.kt)("tr",{parentName:"tbody"},(0,n.kt)("td",{parentName:"tr",align:"center"},"reg8, mem8"),(0,n.kt)("td",{parentName:"tr",align:"center"},"0x22"),(0,n.kt)("td",{parentName:"tr",align:"center"},"2"),(0,n.kt)("td",{parentName:"tr",align:"center"},"Performs bitwise AND operation between the two operands and stores the result in the first operand"),(0,n.kt)("td",{parentName:"tr",align:null},"AND AL, ","[BX]")),(0,n.kt)("tr",{parentName:"tbody"},(0,n.kt)("td",{parentName:"tr",align:"center"},"mem16, reg16"),(0,n.kt)("td",{parentName:"tr",align:"center"},"0x23"),(0,n.kt)("td",{parentName:"tr",align:"center"},"2"),(0,n.kt)("td",{parentName:"tr",align:"center"},"Performs bitwise AND operation between the two operands and stores the result in the first operand"),(0,n.kt)("td",{parentName:"tr",align:null},"AND ","[BX]",", AX")),(0,n.kt)("tr",{parentName:"tbody"},(0,n.kt)("td",{parentName:"tr",align:"center"},"mem8, reg8"),(0,n.kt)("td",{parentName:"tr",align:"center"},"0x22"),(0,n.kt)("td",{parentName:"tr",align:"center"},"2"),(0,n.kt)("td",{parentName:"tr",align:"center"},"Performs bitwise AND operation between the two operands and stores the result in the first operand"),(0,n.kt)("td",{parentName:"tr",align:null},"AND ","[BX]",", AL")),(0,n.kt)("tr",{parentName:"tbody"},(0,n.kt)("td",{parentName:"tr",align:"center"},"reg16, imm16"),(0,n.kt)("td",{parentName:"tr",align:"center"},"0x81 0x20"),(0,n.kt)("td",{parentName:"tr",align:"center"},"3"),(0,n.kt)("td",{parentName:"tr",align:"center"},"Performs bitwise AND operation between the two operands and stores the result in the first operand"),(0,n.kt)("td",{parentName:"tr",align:null},"AND AX, 0x0001")),(0,n.kt)("tr",{parentName:"tbody"},(0,n.kt)("td",{parentName:"tr",align:"center"},"reg8, imm8"),(0,n.kt)("td",{parentName:"tr",align:"center"},"0x80 0x20"),(0,n.kt)("td",{parentName:"tr",align:"center"},"3"),(0,n.kt)("td",{parentName:"tr",align:"center"},"Performs bitwise AND operation between the two operands and stores the result in the first operand"),(0,n.kt)("td",{parentName:"tr",align:null},"AND AL, 0x01")),(0,n.kt)("tr",{parentName:"tbody"},(0,n.kt)("td",{parentName:"tr",align:"center"},"mem16, imm16"),(0,n.kt)("td",{parentName:"tr",align:"center"},"0x81 0x20"),(0,n.kt)("td",{parentName:"tr",align:"center"},"3"),(0,n.kt)("td",{parentName:"tr",align:"center"},"Performs bitwise AND operation between the two operands and stores the result in the first operand"),(0,n.kt)("td",{parentName:"tr",align:null},"AND ","[BX]",", 0x0001")),(0,n.kt)("tr",{parentName:"tbody"},(0,n.kt)("td",{parentName:"tr",align:"center"},"mem8, imm8"),(0,n.kt)("td",{parentName:"tr",align:"center"},"0x80 0x20"),(0,n.kt)("td",{parentName:"tr",align:"center"},"3"),(0,n.kt)("td",{parentName:"tr",align:"center"},"Performs bitwise AND operation between the two operands and stores the result in the first operand"),(0,n.kt)("td",{parentName:"tr",align:null},"AND ","[BX]",", 0x01")))))}c.isMDXComponent=!0}}]);