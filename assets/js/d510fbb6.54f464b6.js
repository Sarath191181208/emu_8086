"use strict";(self.webpackChunkdocs=self.webpackChunkdocs||[]).push([[6712],{165:(t,e,a)=>{a.r(e),a.d(e,{assets:()=>d,contentTitle:()=>o,default:()=>c,frontMatter:()=>i,metadata:()=>p,toc:()=>m});var n=a(7462),r=(a(7294),a(3905));const i={authors:{name:"Vangipuram Srinivasa Sarath Chandra",title:"Tech enthusiast",url:"https://github.com/Sarath191181208",image_url:"https://avatars.githubusercontent.com/u/74459981?v=4"}},o="Pop Support",p={permalink:"/emu_8086/blog/2023/10/26/ Added POP support",editUrl:"https://github.com/Sarath191181208/emu_8086/tree/main/docusaurus/blog/2023-10-26 Added POP support.mdx",source:"@site/blog/2023-10-26 Added POP support.mdx",title:"Pop Support",description:"Compilation of the pop instruction",date:"2023-10-26T00:00:00.000Z",formattedDate:"October 26, 2023",tags:[],readingTime:1.04,hasTruncateMarker:!1,authors:[{name:"Vangipuram Srinivasa Sarath Chandra",title:"Tech enthusiast",url:"https://github.com/Sarath191181208",image_url:"https://avatars.githubusercontent.com/u/74459981?v=4",imageURL:"https://avatars.githubusercontent.com/u/74459981?v=4"}],frontMatter:{authors:{name:"Vangipuram Srinivasa Sarath Chandra",title:"Tech enthusiast",url:"https://github.com/Sarath191181208",image_url:"https://avatars.githubusercontent.com/u/74459981?v=4",imageURL:"https://avatars.githubusercontent.com/u/74459981?v=4"}},prevItem:{title:"\ud83d\udee0\ufe0f Refactor",permalink:"/emu_8086/blog/2023/10/27/ Refactor"}},d={authorsImageUrls:[void 0]},m=[{value:"Compilation of the pop instruction",id:"compilation-of-the-pop-instruction",level:2},{value:"Execution of the pop instruction",id:"execution-of-the-pop-instruction",level:2}],l={toc:m},s="wrapper";function c(t){let{components:e,...a}=t;return(0,r.kt)(s,(0,n.Z)({},l,a,{components:e,mdxType:"MDXLayout"}),(0,r.kt)("h2",{id:"compilation-of-the-pop-instruction"},"Compilation of the pop instruction"),(0,r.kt)("table",null,(0,r.kt)("thead",{parentName:"table"},(0,r.kt)("tr",{parentName:"thead"},(0,r.kt)("th",{parentName:"tr",align:"center"},"Operand"),(0,r.kt)("th",{parentName:"tr",align:"center"},"Opcode"),(0,r.kt)("th",{parentName:"tr",align:"center"},"Description"),(0,r.kt)("th",{parentName:"tr",align:"center"},"Example instruction"))),(0,r.kt)("tbody",{parentName:"table"},(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:"center"},"reg16"),(0,r.kt)("td",{parentName:"tr",align:"center"},"0x58+rw"),(0,r.kt)("td",{parentName:"tr",align:"center"},"Pop top of stack into reg16"),(0,r.kt)("td",{parentName:"tr",align:"center"},(0,r.kt)("inlineCode",{parentName:"td"},"pop ax"))),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:"center"},"mem16"),(0,r.kt)("td",{parentName:"tr",align:"center"},"0x8F 0x06 16BIT-addr"),(0,r.kt)("td",{parentName:"tr",align:"center"},"Pop top of stack into mem16"),(0,r.kt)("td",{parentName:"tr",align:"center"},(0,r.kt)("inlineCode",{parentName:"td"},"pop [0x0100]"))),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:"center"},"indexed with no offset"),(0,r.kt)("td",{parentName:"tr",align:"center"},"0x8F 0x00..0x07"),(0,r.kt)("td",{parentName:"tr",align:"center"},"Pop top of stack into index given by regs"),(0,r.kt)("td",{parentName:"tr",align:"center"},(0,r.kt)("inlineCode",{parentName:"td"},"pop [bx+si]"))),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:"center"},"indexed with byte offset"),(0,r.kt)("td",{parentName:"tr",align:"center"},"0x8F 0x40..0x47 16bit-addr"),(0,r.kt)("td",{parentName:"tr",align:"center"},"Pop top of stack into index given by regs + byte offset"),(0,r.kt)("td",{parentName:"tr",align:"center"},(0,r.kt)("inlineCode",{parentName:"td"},"pop [bx+0x01]"))),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:"center"},"indexed with word offset"),(0,r.kt)("td",{parentName:"tr",align:"center"},"0x8F 0x80..0x87 16bit-addr"),(0,r.kt)("td",{parentName:"tr",align:"center"},"Pop top of stack into index given by regs + word offset"),(0,r.kt)("td",{parentName:"tr",align:"center"},(0,r.kt)("inlineCode",{parentName:"td"},"pop [bx+0x0100]"))))),(0,r.kt)("h2",{id:"execution-of-the-pop-instruction"},"Execution of the pop instruction"),(0,r.kt)("p",null,"Made the ",(0,r.kt)("inlineCode",{parentName:"p"},"pop")," instruction execution working.\nAddressing modes of the ",(0,r.kt)("inlineCode",{parentName:"p"},"pop")," instruction implemented are:"),(0,r.kt)("ul",null,(0,r.kt)("li",{parentName:"ul"},"Register addressing mode, ex - ",(0,r.kt)("inlineCode",{parentName:"li"},"pop ax")),(0,r.kt)("li",{parentName:"ul"},"Direct addressing mode, ex - ",(0,r.kt)("inlineCode",{parentName:"li"},"pop [0x1234]")),(0,r.kt)("li",{parentName:"ul"},"Variable addressing mode, ex - ",(0,r.kt)("inlineCode",{parentName:"li"},"pop [var]")),(0,r.kt)("li",{parentName:"ul"},"Indirect addressing mode, ex - ",(0,r.kt)("inlineCode",{parentName:"li"},"pop [bx]")),(0,r.kt)("li",{parentName:"ul"},"Indexed addressing mode, ex - ",(0,r.kt)("inlineCode",{parentName:"li"},"pop [bx+si]"))),(0,r.kt)("p",null,"Algorithm:"),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre"},"operand = SS:[SP] (top of the stack)\nSP = SP + 2\n")))}c.isMDXComponent=!0}}]);