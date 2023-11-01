"use strict";(self.webpackChunkdocs=self.webpackChunkdocs||[]).push([[747],{3905:(e,t,r)=>{r.d(t,{Zo:()=>p,kt:()=>f});var n=r(7294);function a(e,t,r){return t in e?Object.defineProperty(e,t,{value:r,enumerable:!0,configurable:!0,writable:!0}):e[t]=r,e}function o(e,t){var r=Object.keys(e);if(Object.getOwnPropertySymbols){var n=Object.getOwnPropertySymbols(e);t&&(n=n.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),r.push.apply(r,n)}return r}function i(e){for(var t=1;t<arguments.length;t++){var r=null!=arguments[t]?arguments[t]:{};t%2?o(Object(r),!0).forEach((function(t){a(e,t,r[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(r)):o(Object(r)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(r,t))}))}return e}function u(e,t){if(null==e)return{};var r,n,a=function(e,t){if(null==e)return{};var r,n,a={},o=Object.keys(e);for(n=0;n<o.length;n++)r=o[n],t.indexOf(r)>=0||(a[r]=e[r]);return a}(e,t);if(Object.getOwnPropertySymbols){var o=Object.getOwnPropertySymbols(e);for(n=0;n<o.length;n++)r=o[n],t.indexOf(r)>=0||Object.prototype.propertyIsEnumerable.call(e,r)&&(a[r]=e[r])}return a}var s=n.createContext({}),c=function(e){var t=n.useContext(s),r=t;return e&&(r="function"==typeof e?e(t):i(i({},t),e)),r},p=function(e){var t=c(e.components);return n.createElement(s.Provider,{value:t},e.children)},l="mdxType",m={inlineCode:"code",wrapper:function(e){var t=e.children;return n.createElement(n.Fragment,{},t)}},h=n.forwardRef((function(e,t){var r=e.components,a=e.mdxType,o=e.originalType,s=e.parentName,p=u(e,["components","mdxType","originalType","parentName"]),l=c(r),h=a,f=l["".concat(s,".").concat(h)]||l[h]||m[h]||o;return r?n.createElement(f,i(i({ref:t},p),{},{components:r})):n.createElement(f,i({ref:t},p))}));function f(e,t){var r=arguments,a=t&&t.mdxType;if("string"==typeof e||a){var o=r.length,i=new Array(o);i[0]=h;var u={};for(var s in t)hasOwnProperty.call(t,s)&&(u[s]=t[s]);u.originalType=e,u[l]="string"==typeof e?e:a,i[1]=u;for(var c=2;c<o;c++)i[c]=r[c];return n.createElement.apply(null,i)}return n.createElement.apply(null,r)}h.displayName="MDXCreateElement"},2632:(e,t,r)=>{r.r(t),r.d(t,{assets:()=>s,contentTitle:()=>i,default:()=>m,frontMatter:()=>o,metadata:()=>u,toc:()=>c});var n=r(7462),a=(r(7294),r(3905));const o={authors:{name:"Vangipuram Srinivasa Sarath Chandra",title:"Tech enthusiast",url:"https://github.com/Sarath191181208",image_url:"https://avatars.githubusercontent.com/u/74459981?v=4"}},i="Support for execution of OR ins",u={permalink:"/emu_8086/blog/2023/11/2/",editUrl:"https://github.com/Sarath191181208/emu_8086/tree/main/docusaurus/blog/2023-11-2.mdx",source:"@site/blog/2023-11-2.mdx",title:"Support for execution of OR ins",description:"Execution of OR instructions",date:"2023-11-02T00:00:00.000Z",formattedDate:"November 2, 2023",tags:[],readingTime:.185,hasTruncateMarker:!1,authors:[{name:"Vangipuram Srinivasa Sarath Chandra",title:"Tech enthusiast",url:"https://github.com/Sarath191181208",image_url:"https://avatars.githubusercontent.com/u/74459981?v=4",imageURL:"https://avatars.githubusercontent.com/u/74459981?v=4"}],frontMatter:{authors:{name:"Vangipuram Srinivasa Sarath Chandra",title:"Tech enthusiast",url:"https://github.com/Sarath191181208",image_url:"https://avatars.githubusercontent.com/u/74459981?v=4",imageURL:"https://avatars.githubusercontent.com/u/74459981?v=4"}},nextItem:{title:"Support for memory, reg addressing for 2 operand instructions",permalink:"/emu_8086/blog/2023/11/1/"}},s={authorsImageUrls:[void 0]},c=[{value:"Execution of <code>OR</code> instructions",id:"execution-of-or-instructions",level:2}],p={toc:c},l="wrapper";function m(e){let{components:t,...r}=e;return(0,a.kt)(l,(0,n.Z)({},p,r,{components:t,mdxType:"MDXLayout"}),(0,a.kt)("h2",{id:"execution-of-or-instructions"},"Execution of ",(0,a.kt)("inlineCode",{parentName:"h2"},"OR")," instructions"),(0,a.kt)("p",null,"The ",(0,a.kt)("inlineCode",{parentName:"p"},"OR")," instruction performs a bitwise OR operation on the value in register ",(0,a.kt)("inlineCode",{parentName:"p"},"rs1")," and ",(0,a.kt)("inlineCode",{parentName:"p"},"rs2")," and stores the result in register ",(0,a.kt)("inlineCode",{parentName:"p"},"rs1"),". The syntax is:"),(0,a.kt)("pre",null,(0,a.kt)("code",{parentName:"pre",className:"language-asm6502",metastring:"showLineNumbers",showLineNumbers:!0},"OR AX, BX\n")))}m.isMDXComponent=!0}}]);