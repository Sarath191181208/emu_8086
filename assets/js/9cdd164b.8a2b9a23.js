"use strict";(self.webpackChunkdocs=self.webpackChunkdocs||[]).push([[1691],{3749:(e,t,a)=>{a.d(t,{Z:()=>i});var n=a(7294);const r={tooltip:"tooltip_OfI_",tooltiptext:"tooltiptext_kMd9",bottomTooltiptext:"bottomTooltiptext_PBXK"};function l(e){let{children:t,text:a,toolTipPosition:l="top"}=e;const s="top"===l?r.tooltiptext:r.bottomTooltiptext;return n.createElement("div",{className:r.tooltip},t,n.createElement("span",{className:s},a))}const s=e=>{let{state:t}=e;return"changed"===t?n.createElement("span",{className:"badge badge--primary badge--rounded h-20"},n.createElement("span",{className:"badge__text"},n.createElement(l,{text:"Changes"},"C"))):"unchanged"===t?n.createElement("span",{className:"badge badge--secondary badge--rounded"},n.createElement("span",{className:"badge__text"},n.createElement(l,{text:"Doesn't change"},"NC"))):1==t?n.createElement("span",{className:"badge badge--success badge--rounded"},n.createElement("span",{className:"badge__text"},n.createElement(l,{text:"Changed to 1"},"1"))):0==t?n.createElement("span",{className:"badge badge--danger badge--rounded"},n.createElement("span",{className:"badge__text"},n.createElement(l,{text:"Changed to 0"},"0"))):n.createElement("span",{className:"badge badge--warning badge--rounded"},n.createElement("span",{className:"badge__text"},"Unknown"))};function i(e){let{carryFlag:t=null,zeroFlag:a=null,signFlag:r=null,overflowFlag:i=null,parityFlag:o=null,auxiliaryCarryFlag:d=null}=e;const m={carryFlag:t,zeroFlag:a,signFlag:r,overflowFlag:i,parityFlag:o,auxiliaryCarryFlag:d};return n.createElement("table",{className:"table table--striped table--responsive flags_table"},n.createElement("thead",null,n.createElement("tr",null,Object.entries(m).map((e=>{let[t,a]=e;return null==a?null:n.createElement("th",null,n.createElement(l,{text:t,toolTipPosition:"bottom"},t[0].toUpperCase()))})))),n.createElement("tbody",null,n.createElement("tr",null,Object.values(m).map((e=>null==e?null:n.createElement("td",null,n.createElement(s,{state:e})))))))}},494:(e,t,a)=>{a.r(t),a.d(t,{assets:()=>m,contentTitle:()=>o,default:()=>g,frontMatter:()=>i,metadata:()=>d,toc:()=>p});var n=a(7462),r=(a(7294),a(3905)),l=a(3749),s=a(7898);const i={title:"DIV",description:"Unsigned divide."},o=void 0,d={unversionedId:"Instructions/DIV",id:"Instructions/DIV",title:"DIV",description:"Unsigned divide.",source:"@site/docs/Instructions/DIV.mdx",sourceDirName:"Instructions",slug:"/Instructions/DIV",permalink:"/emu_8086/docs/Instructions/DIV",draft:!1,editUrl:"https://github.com/Sarath191181208/emu_8086/tree/main/docusaurus/docs/Instructions/DIV.mdx",tags:[],version:"current",frontMatter:{title:"DIV",description:"Unsigned divide."},sidebar:"tutorialSidebar",previous:{title:"CMP",permalink:"/emu_8086/docs/Instructions/CMP"},next:{title:"IDIV",permalink:"/emu_8086/docs/Instructions/IDIV"}},m={},p=[{value:"Syntax",id:"syntax",level:2},{value:"Attributes",id:"attributes",level:2},{value:"Example",id:"example",level:2},{value:"Supported Modes",id:"supported-modes",level:2},{value:"16bit/8bit reg/mem addressing",id:"16bit8bit-regmem-addressing",level:3}],u={toc:p},c="wrapper";function g(e){let{components:t,...a}=e;return(0,r.kt)(c,(0,n.Z)({},u,a,{components:t,mdxType:"MDXLayout"}),(0,r.kt)("h2",{id:"syntax"},"Syntax"),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-asm6502",metastring:"showLineNumbers",showLineNumbers:!0},"    DIV        BL ; 8 bit register\n    DIV        AX ; 16 bit register\n    DIV   [0x100] ; 16bit memory location\n    DIV b.[0x100] ; 8bit memory location\n")),(0,r.kt)("h2",{id:"attributes"},"Attributes"),(0,r.kt)("table",null,(0,r.kt)("thead",{parentName:"table"},(0,r.kt)("tr",{parentName:"thead"},(0,r.kt)("th",{parentName:"tr",align:null},"Attribute"),(0,r.kt)("th",{parentName:"tr",align:null},"Description"))),(0,r.kt)("tbody",{parentName:"table"},(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:null},(0,r.kt)("strong",{parentName:"td"},"Operands")),(0,r.kt)("td",{parentName:"tr",align:null},(0,r.kt)("inlineCode",{parentName:"td"},"register")," - A 16/8 bit register. ",(0,r.kt)("br",null)," ",(0,r.kt)("inlineCode",{parentName:"td"},"memory")," - An 16/8 bit memory location.")),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:null},(0,r.kt)("strong",{parentName:"td"},"Operation")),(0,r.kt)("td",{parentName:"tr",align:null},"If operand is a byte: ",(0,r.kt)("br",null)," ","\u2003"," AL = AX / operand ",(0,r.kt)("br",null)," ","\u2003"," AH = AX % operand -> (remainder) ",(0,r.kt)("br",null)," If operand is a word: ",(0,r.kt)("br",null)," ","\u2003"," AX = (DX AX) / operand ",(0,r.kt)("br",null)," ","\u2003"," DX = (DX AX) % operand -> (remainder)")),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:null},(0,r.kt)("strong",{parentName:"td"},"Size")),(0,r.kt)("td",{parentName:"tr",align:null},"3 bytes, 5 bytes")),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:null},(0,r.kt)("strong",{parentName:"td"},"Action")),(0,r.kt)("td",{parentName:"tr",align:null},"Unsigned division of the operand with the particular register/registers")),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:null},(0,r.kt)("strong",{parentName:"td"},"Flags")),(0,r.kt)("td",{parentName:"tr",align:null},(0,r.kt)(l.Z,{carryFlag:"changed",overflowFlag:"changed",zeroFlag:"changed",signFlag:"changed",parityFlag:"changed",auxiliaryCarryFlag:"changed",mdxType:"FlagsChangedTable"}))))),(0,r.kt)("h2",{id:"example"},"Example"),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-asm6502",metastring:"showLineNumbers",showLineNumbers:!0},"MOV AX, 203   ; AX = 00CBh\nMOV BL, 4\nDIV BL        ; AL = 50 (32h), AH = 3\n")),(0,r.kt)("h2",{id:"supported-modes"},"Supported Modes"),(0,r.kt)("h3",{id:"16bit8bit-regmem-addressing"},"16bit/8bit reg/mem addressing"),(0,r.kt)(s.Rz,{instructionName:"DIV",mdxType:"RegisterAndMemoryAddressingSingleInstruction"}))}g.isMDXComponent=!0}}]);