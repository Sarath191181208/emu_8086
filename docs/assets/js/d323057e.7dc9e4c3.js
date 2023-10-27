"use strict";(self.webpackChunkdocs=self.webpackChunkdocs||[]).push([[934],{3905:(t,e,n)=>{n.d(e,{Zo:()=>d,kt:()=>g});var r=n(7294);function a(t,e,n){return e in t?Object.defineProperty(t,e,{value:n,enumerable:!0,configurable:!0,writable:!0}):t[e]=n,t}function o(t,e){var n=Object.keys(t);if(Object.getOwnPropertySymbols){var r=Object.getOwnPropertySymbols(t);e&&(r=r.filter((function(e){return Object.getOwnPropertyDescriptor(t,e).enumerable}))),n.push.apply(n,r)}return n}function i(t){for(var e=1;e<arguments.length;e++){var n=null!=arguments[e]?arguments[e]:{};e%2?o(Object(n),!0).forEach((function(e){a(t,e,n[e])})):Object.getOwnPropertyDescriptors?Object.defineProperties(t,Object.getOwnPropertyDescriptors(n)):o(Object(n)).forEach((function(e){Object.defineProperty(t,e,Object.getOwnPropertyDescriptor(n,e))}))}return t}function p(t,e){if(null==t)return{};var n,r,a=function(t,e){if(null==t)return{};var n,r,a={},o=Object.keys(t);for(r=0;r<o.length;r++)n=o[r],e.indexOf(n)>=0||(a[n]=t[n]);return a}(t,e);if(Object.getOwnPropertySymbols){var o=Object.getOwnPropertySymbols(t);for(r=0;r<o.length;r++)n=o[r],e.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(t,n)&&(a[n]=t[n])}return a}var l=r.createContext({}),c=function(t){var e=r.useContext(l),n=e;return t&&(n="function"==typeof t?t(e):i(i({},e),t)),n},d=function(t){var e=c(t.components);return r.createElement(l.Provider,{value:e},t.children)},m="mdxType",s={inlineCode:"code",wrapper:function(t){var e=t.children;return r.createElement(r.Fragment,{},e)}},u=r.forwardRef((function(t,e){var n=t.components,a=t.mdxType,o=t.originalType,l=t.parentName,d=p(t,["components","mdxType","originalType","parentName"]),m=c(n),u=a,g=m["".concat(l,".").concat(u)]||m[u]||s[u]||o;return n?r.createElement(g,i(i({ref:e},d),{},{components:n})):r.createElement(g,i({ref:e},d))}));function g(t,e){var n=arguments,a=e&&e.mdxType;if("string"==typeof t||a){var o=n.length,i=new Array(o);i[0]=u;var p={};for(var l in e)hasOwnProperty.call(e,l)&&(p[l]=e[l]);p.originalType=t,p[m]="string"==typeof t?t:a,i[1]=p;for(var c=2;c<o;c++)i[c]=n[c];return r.createElement.apply(null,i)}return r.createElement.apply(null,n)}u.displayName="MDXCreateElement"},2037:(t,e,n)=>{n.r(e),n.d(e,{assets:()=>l,contentTitle:()=>i,default:()=>s,frontMatter:()=>o,metadata:()=>p,toc:()=>c});var r=n(7462),a=(n(7294),n(3905));const o={authors:{name:"Vangipuram Srinivasa Sarath Chandra",title:"Tech enthusiast",url:"https://github.com/Sarath191181208",image_url:"https://avatars.githubusercontent.com/u/74459981?v=4"}},i="Pop Support",p={permalink:"/emu_8086/blog/26-10-2023 Added POP support",editUrl:"https://github.com/facebook/docusaurus/tree/main/packages/create-docusaurus/templates/shared/blog/26-10-2023 Added POP support.mdx",source:"@site/blog/26-10-2023 Added POP support.mdx",title:"Pop Support",description:"Compilation of the pop instruction",date:"2023-10-26T05:36:19.000Z",formattedDate:"October 26, 2023",tags:[],readingTime:1.04,hasTruncateMarker:!1,authors:[{name:"Vangipuram Srinivasa Sarath Chandra",title:"Tech enthusiast",url:"https://github.com/Sarath191181208",image_url:"https://avatars.githubusercontent.com/u/74459981?v=4",imageURL:"https://avatars.githubusercontent.com/u/74459981?v=4"}],frontMatter:{authors:{name:"Vangipuram Srinivasa Sarath Chandra",title:"Tech enthusiast",url:"https://github.com/Sarath191181208",image_url:"https://avatars.githubusercontent.com/u/74459981?v=4",imageURL:"https://avatars.githubusercontent.com/u/74459981?v=4"}},prevItem:{title:"\ud83d\udee0\ufe0f Refactor",permalink:"/emu_8086/blog/27-10-2023 Refactor"}},l={authorsImageUrls:[void 0]},c=[{value:"Compilation of the pop instruction",id:"compilation-of-the-pop-instruction",level:2},{value:"Execution of the pop instruction",id:"execution-of-the-pop-instruction",level:2}],d={toc:c},m="wrapper";function s(t){let{components:e,...n}=t;return(0,a.kt)(m,(0,r.Z)({},d,n,{components:e,mdxType:"MDXLayout"}),(0,a.kt)("h2",{id:"compilation-of-the-pop-instruction"},"Compilation of the pop instruction"),(0,a.kt)("table",null,(0,a.kt)("thead",{parentName:"table"},(0,a.kt)("tr",{parentName:"thead"},(0,a.kt)("th",{parentName:"tr",align:"center"},"Operand"),(0,a.kt)("th",{parentName:"tr",align:"center"},"Opcode"),(0,a.kt)("th",{parentName:"tr",align:"center"},"Description"),(0,a.kt)("th",{parentName:"tr",align:"center"},"Example instruction"))),(0,a.kt)("tbody",{parentName:"table"},(0,a.kt)("tr",{parentName:"tbody"},(0,a.kt)("td",{parentName:"tr",align:"center"},"reg16"),(0,a.kt)("td",{parentName:"tr",align:"center"},"0x58+rw"),(0,a.kt)("td",{parentName:"tr",align:"center"},"Pop top of stack into reg16"),(0,a.kt)("td",{parentName:"tr",align:"center"},(0,a.kt)("inlineCode",{parentName:"td"},"pop ax"))),(0,a.kt)("tr",{parentName:"tbody"},(0,a.kt)("td",{parentName:"tr",align:"center"},"mem16"),(0,a.kt)("td",{parentName:"tr",align:"center"},"0x8F 0x06 16BIT-addr"),(0,a.kt)("td",{parentName:"tr",align:"center"},"Pop top of stack into mem16"),(0,a.kt)("td",{parentName:"tr",align:"center"},(0,a.kt)("inlineCode",{parentName:"td"},"pop [0x0100]"))),(0,a.kt)("tr",{parentName:"tbody"},(0,a.kt)("td",{parentName:"tr",align:"center"},"indexed with no offset"),(0,a.kt)("td",{parentName:"tr",align:"center"},"0x8F 0x00..0x07"),(0,a.kt)("td",{parentName:"tr",align:"center"},"Pop top of stack into index given by regs"),(0,a.kt)("td",{parentName:"tr",align:"center"},(0,a.kt)("inlineCode",{parentName:"td"},"pop [bx+si]"))),(0,a.kt)("tr",{parentName:"tbody"},(0,a.kt)("td",{parentName:"tr",align:"center"},"indexed with byte offset"),(0,a.kt)("td",{parentName:"tr",align:"center"},"0x8F 0x40..0x47 16bit-addr"),(0,a.kt)("td",{parentName:"tr",align:"center"},"Pop top of stack into index given by regs + byte offset"),(0,a.kt)("td",{parentName:"tr",align:"center"},(0,a.kt)("inlineCode",{parentName:"td"},"pop [bx+0x01]"))),(0,a.kt)("tr",{parentName:"tbody"},(0,a.kt)("td",{parentName:"tr",align:"center"},"indexed with word offset"),(0,a.kt)("td",{parentName:"tr",align:"center"},"0x8F 0x80..0x87 16bit-addr"),(0,a.kt)("td",{parentName:"tr",align:"center"},"Pop top of stack into index given by regs + word offset"),(0,a.kt)("td",{parentName:"tr",align:"center"},(0,a.kt)("inlineCode",{parentName:"td"},"pop [bx+0x0100]"))))),(0,a.kt)("h2",{id:"execution-of-the-pop-instruction"},"Execution of the pop instruction"),(0,a.kt)("p",null,"Made the ",(0,a.kt)("inlineCode",{parentName:"p"},"pop")," instruction execution working.\nAddressing modes of the ",(0,a.kt)("inlineCode",{parentName:"p"},"pop")," instruction implemented are:"),(0,a.kt)("ul",null,(0,a.kt)("li",{parentName:"ul"},"Register addressing mode, ex - ",(0,a.kt)("inlineCode",{parentName:"li"},"pop ax")),(0,a.kt)("li",{parentName:"ul"},"Direct addressing mode, ex - ",(0,a.kt)("inlineCode",{parentName:"li"},"pop [0x1234]")),(0,a.kt)("li",{parentName:"ul"},"Variable addressing mode, ex - ",(0,a.kt)("inlineCode",{parentName:"li"},"pop [var]")),(0,a.kt)("li",{parentName:"ul"},"Indirect addressing mode, ex - ",(0,a.kt)("inlineCode",{parentName:"li"},"pop [bx]")),(0,a.kt)("li",{parentName:"ul"},"Indexed addressing mode, ex - ",(0,a.kt)("inlineCode",{parentName:"li"},"pop [bx+si]"))),(0,a.kt)("p",null,"Algorithm:"),(0,a.kt)("pre",null,(0,a.kt)("code",{parentName:"pre"},"operand = SS:[SP] (top of the stack)\nSP = SP + 2\n")))}s.isMDXComponent=!0}}]);