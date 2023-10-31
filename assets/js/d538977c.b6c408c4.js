"use strict";(self.webpackChunkdocs=self.webpackChunkdocs||[]).push([[838],{3905:(t,e,r)=>{r.d(e,{Zo:()=>m,kt:()=>g});var n=r(7294);function a(t,e,r){return e in t?Object.defineProperty(t,e,{value:r,enumerable:!0,configurable:!0,writable:!0}):t[e]=r,t}function i(t,e){var r=Object.keys(t);if(Object.getOwnPropertySymbols){var n=Object.getOwnPropertySymbols(t);e&&(n=n.filter((function(e){return Object.getOwnPropertyDescriptor(t,e).enumerable}))),r.push.apply(r,n)}return r}function o(t){for(var e=1;e<arguments.length;e++){var r=null!=arguments[e]?arguments[e]:{};e%2?i(Object(r),!0).forEach((function(e){a(t,e,r[e])})):Object.getOwnPropertyDescriptors?Object.defineProperties(t,Object.getOwnPropertyDescriptors(r)):i(Object(r)).forEach((function(e){Object.defineProperty(t,e,Object.getOwnPropertyDescriptor(r,e))}))}return t}function s(t,e){if(null==t)return{};var r,n,a=function(t,e){if(null==t)return{};var r,n,a={},i=Object.keys(t);for(n=0;n<i.length;n++)r=i[n],e.indexOf(r)>=0||(a[r]=t[r]);return a}(t,e);if(Object.getOwnPropertySymbols){var i=Object.getOwnPropertySymbols(t);for(n=0;n<i.length;n++)r=i[n],e.indexOf(r)>=0||Object.prototype.propertyIsEnumerable.call(t,r)&&(a[r]=t[r])}return a}var p=n.createContext({}),l=function(t){var e=n.useContext(p),r=e;return t&&(r="function"==typeof t?t(e):o(o({},e),t)),r},m=function(t){var e=l(t.components);return n.createElement(p.Provider,{value:e},t.children)},c="mdxType",d={inlineCode:"code",wrapper:function(t){var e=t.children;return n.createElement(n.Fragment,{},e)}},u=n.forwardRef((function(t,e){var r=t.components,a=t.mdxType,i=t.originalType,p=t.parentName,m=s(t,["components","mdxType","originalType","parentName"]),c=l(r),u=a,g=c["".concat(p,".").concat(u)]||c[u]||d[u]||i;return r?n.createElement(g,o(o({ref:e},m),{},{components:r})):n.createElement(g,o({ref:e},m))}));function g(t,e){var r=arguments,a=e&&e.mdxType;if("string"==typeof t||a){var i=r.length,o=new Array(i);o[0]=u;var s={};for(var p in e)hasOwnProperty.call(e,p)&&(s[p]=e[p]);s.originalType=t,s[c]="string"==typeof t?t:a,o[1]=s;for(var l=2;l<i;l++)o[l]=r[l];return n.createElement.apply(null,o)}return n.createElement.apply(null,r)}u.displayName="MDXCreateElement"},9144:(t,e,r)=>{r.r(e),r.d(e,{assets:()=>p,contentTitle:()=>o,default:()=>d,frontMatter:()=>i,metadata:()=>s,toc:()=>l});var n=r(7462),a=(r(7294),r(3905));const i={authors:{name:"Vangipuram Srinivasa Sarath Chandra",title:"Tech enthusiast",url:"https://github.com/Sarath191181208",image_url:"https://avatars.githubusercontent.com/u/74459981?v=4"}},o="Support for and instruction compilation",s={permalink:"/emu_8086/blog/2023/10/30/",editUrl:"https://github.com/Sarath191181208/emu_8086/tree/main/docusaurus/blog/2023-10-30.mdx",source:"@site/blog/2023-10-30.mdx",title:"Support for and instruction compilation",description:"The AND instruction is now compiled and working. The AND instruction is a bitwise AND operation. The AND instruction is used to perform a bitwise AND operation between the first source operand (register or memory location) and the second source operand (register or memory location) and store the result in the destination operand (register or memory location). The AND instruction affects the CF and OF flags when the second operand is an immediate value. The SF, ZF, and PF flags are always set according to the result. The state of the AF flag is undefined.",date:"2023-10-30T00:00:00.000Z",formattedDate:"October 30, 2023",tags:[],readingTime:2.11,hasTruncateMarker:!1,authors:[{name:"Vangipuram Srinivasa Sarath Chandra",title:"Tech enthusiast",url:"https://github.com/Sarath191181208",image_url:"https://avatars.githubusercontent.com/u/74459981?v=4",imageURL:"https://avatars.githubusercontent.com/u/74459981?v=4"}],frontMatter:{authors:{name:"Vangipuram Srinivasa Sarath Chandra",title:"Tech enthusiast",url:"https://github.com/Sarath191181208",image_url:"https://avatars.githubusercontent.com/u/74459981?v=4",imageURL:"https://avatars.githubusercontent.com/u/74459981?v=4"}},prevItem:{title:"\ud83d\udee0\ufe0f Refactor of common parsing while exec of reg as first ins",permalink:"/emu_8086/blog/2023/10/31/"},nextItem:{title:"\ud83d\udee0\ufe0f Refactor: moved common logic into a pattern",permalink:"/emu_8086/blog/2023/10/29/"}},p={authorsImageUrls:[void 0]},l=[],m={toc:l},c="wrapper";function d(t){let{components:e,...r}=t;return(0,a.kt)(c,(0,n.Z)({},m,r,{components:e,mdxType:"MDXLayout"}),(0,a.kt)("p",null,"The ",(0,a.kt)("inlineCode",{parentName:"p"},"AND")," instruction is now compiled and working. The ",(0,a.kt)("inlineCode",{parentName:"p"},"AND")," instruction is a bitwise AND operation. The ",(0,a.kt)("inlineCode",{parentName:"p"},"AND")," instruction is used to perform a bitwise AND operation between the first source operand (register or memory location) and the second source operand (register or memory location) and store the result in the destination operand (register or memory location). The ",(0,a.kt)("inlineCode",{parentName:"p"},"AND")," instruction affects the CF and OF flags when the second operand is an immediate value. The SF, ZF, and PF flags are always set according to the result. The state of the AF flag is undefined.\nThe compilation table of the ",(0,a.kt)("inlineCode",{parentName:"p"},"AND")," instruction is as follows:"),(0,a.kt)("table",null,(0,a.kt)("thead",{parentName:"table"},(0,a.kt)("tr",{parentName:"thead"},(0,a.kt)("th",{parentName:"tr",align:"center"},"Operand"),(0,a.kt)("th",{parentName:"tr",align:"center"},"Opcode"),(0,a.kt)("th",{parentName:"tr",align:"center"},"Size(bytes)"),(0,a.kt)("th",{parentName:"tr",align:"center"},"Description"),(0,a.kt)("th",{parentName:"tr",align:null},"Example instruction"))),(0,a.kt)("tbody",{parentName:"table"},(0,a.kt)("tr",{parentName:"tbody"},(0,a.kt)("td",{parentName:"tr",align:"center"},"reg16, reg16"),(0,a.kt)("td",{parentName:"tr",align:"center"},"0x21"),(0,a.kt)("td",{parentName:"tr",align:"center"},"2"),(0,a.kt)("td",{parentName:"tr",align:"center"},"Performs bitwise AND operation between the two operands and stores the result in the first operand"),(0,a.kt)("td",{parentName:"tr",align:null},"AND AX, BX")),(0,a.kt)("tr",{parentName:"tbody"},(0,a.kt)("td",{parentName:"tr",align:"center"},"reg8, reg8"),(0,a.kt)("td",{parentName:"tr",align:"center"},"0x20"),(0,a.kt)("td",{parentName:"tr",align:"center"},"2"),(0,a.kt)("td",{parentName:"tr",align:"center"},"Performs bitwise AND operation between the two operands and stores the result in the first operand"),(0,a.kt)("td",{parentName:"tr",align:null},"AND AL, BL")),(0,a.kt)("tr",{parentName:"tbody"},(0,a.kt)("td",{parentName:"tr",align:"center"},"reg16, mem16"),(0,a.kt)("td",{parentName:"tr",align:"center"},"0x23"),(0,a.kt)("td",{parentName:"tr",align:"center"},"2"),(0,a.kt)("td",{parentName:"tr",align:"center"},"Performs bitwise AND operation between the two operands and stores the result in the first operand"),(0,a.kt)("td",{parentName:"tr",align:null},"AND AX, ","[BX]")),(0,a.kt)("tr",{parentName:"tbody"},(0,a.kt)("td",{parentName:"tr",align:"center"},"reg8, mem8"),(0,a.kt)("td",{parentName:"tr",align:"center"},"0x22"),(0,a.kt)("td",{parentName:"tr",align:"center"},"2"),(0,a.kt)("td",{parentName:"tr",align:"center"},"Performs bitwise AND operation between the two operands and stores the result in the first operand"),(0,a.kt)("td",{parentName:"tr",align:null},"AND AL, ","[BX]")),(0,a.kt)("tr",{parentName:"tbody"},(0,a.kt)("td",{parentName:"tr",align:"center"},"mem16, reg16"),(0,a.kt)("td",{parentName:"tr",align:"center"},"0x23"),(0,a.kt)("td",{parentName:"tr",align:"center"},"2"),(0,a.kt)("td",{parentName:"tr",align:"center"},"Performs bitwise AND operation between the two operands and stores the result in the first operand"),(0,a.kt)("td",{parentName:"tr",align:null},"AND ","[BX]",", AX")),(0,a.kt)("tr",{parentName:"tbody"},(0,a.kt)("td",{parentName:"tr",align:"center"},"mem8, reg8"),(0,a.kt)("td",{parentName:"tr",align:"center"},"0x22"),(0,a.kt)("td",{parentName:"tr",align:"center"},"2"),(0,a.kt)("td",{parentName:"tr",align:"center"},"Performs bitwise AND operation between the two operands and stores the result in the first operand"),(0,a.kt)("td",{parentName:"tr",align:null},"AND ","[BX]",", AL")),(0,a.kt)("tr",{parentName:"tbody"},(0,a.kt)("td",{parentName:"tr",align:"center"},"reg16, imm16"),(0,a.kt)("td",{parentName:"tr",align:"center"},"0x81 0x20"),(0,a.kt)("td",{parentName:"tr",align:"center"},"3"),(0,a.kt)("td",{parentName:"tr",align:"center"},"Performs bitwise AND operation between the two operands and stores the result in the first operand"),(0,a.kt)("td",{parentName:"tr",align:null},"AND AX, 0x0001")),(0,a.kt)("tr",{parentName:"tbody"},(0,a.kt)("td",{parentName:"tr",align:"center"},"reg8, imm8"),(0,a.kt)("td",{parentName:"tr",align:"center"},"0x80 0x20"),(0,a.kt)("td",{parentName:"tr",align:"center"},"3"),(0,a.kt)("td",{parentName:"tr",align:"center"},"Performs bitwise AND operation between the two operands and stores the result in the first operand"),(0,a.kt)("td",{parentName:"tr",align:null},"AND AL, 0x01")),(0,a.kt)("tr",{parentName:"tbody"},(0,a.kt)("td",{parentName:"tr",align:"center"},"mem16, imm16"),(0,a.kt)("td",{parentName:"tr",align:"center"},"0x81 0x20"),(0,a.kt)("td",{parentName:"tr",align:"center"},"3"),(0,a.kt)("td",{parentName:"tr",align:"center"},"Performs bitwise AND operation between the two operands and stores the result in the first operand"),(0,a.kt)("td",{parentName:"tr",align:null},"AND ","[BX]",", 0x0001")),(0,a.kt)("tr",{parentName:"tbody"},(0,a.kt)("td",{parentName:"tr",align:"center"},"mem8, imm8"),(0,a.kt)("td",{parentName:"tr",align:"center"},"0x80 0x20"),(0,a.kt)("td",{parentName:"tr",align:"center"},"3"),(0,a.kt)("td",{parentName:"tr",align:"center"},"Performs bitwise AND operation between the two operands and stores the result in the first operand"),(0,a.kt)("td",{parentName:"tr",align:null},"AND ","[BX]",", 0x01")))))}d.isMDXComponent=!0}}]);