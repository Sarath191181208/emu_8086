"use strict";(self.webpackChunkdocs=self.webpackChunkdocs||[]).push([[938],{3905:(e,t,r)=>{r.d(t,{Zo:()=>u,kt:()=>g});var n=r(7294);function a(e,t,r){return t in e?Object.defineProperty(e,t,{value:r,enumerable:!0,configurable:!0,writable:!0}):e[t]=r,e}function o(e,t){var r=Object.keys(e);if(Object.getOwnPropertySymbols){var n=Object.getOwnPropertySymbols(e);t&&(n=n.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),r.push.apply(r,n)}return r}function i(e){for(var t=1;t<arguments.length;t++){var r=null!=arguments[t]?arguments[t]:{};t%2?o(Object(r),!0).forEach((function(t){a(e,t,r[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(r)):o(Object(r)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(r,t))}))}return e}function s(e,t){if(null==e)return{};var r,n,a=function(e,t){if(null==e)return{};var r,n,a={},o=Object.keys(e);for(n=0;n<o.length;n++)r=o[n],t.indexOf(r)>=0||(a[r]=e[r]);return a}(e,t);if(Object.getOwnPropertySymbols){var o=Object.getOwnPropertySymbols(e);for(n=0;n<o.length;n++)r=o[n],t.indexOf(r)>=0||Object.prototype.propertyIsEnumerable.call(e,r)&&(a[r]=e[r])}return a}var l=n.createContext({}),c=function(e){var t=n.useContext(l),r=t;return e&&(r="function"==typeof e?e(t):i(i({},t),e)),r},u=function(e){var t=c(e.components);return n.createElement(l.Provider,{value:t},e.children)},f="mdxType",p={inlineCode:"code",wrapper:function(e){var t=e.children;return n.createElement(n.Fragment,{},t)}},m=n.forwardRef((function(e,t){var r=e.components,a=e.mdxType,o=e.originalType,l=e.parentName,u=s(e,["components","mdxType","originalType","parentName"]),f=c(r),m=a,g=f["".concat(l,".").concat(m)]||f[m]||p[m]||o;return r?n.createElement(g,i(i({ref:t},u),{},{components:r})):n.createElement(g,i({ref:t},u))}));function g(e,t){var r=arguments,a=t&&t.mdxType;if("string"==typeof e||a){var o=r.length,i=new Array(o);i[0]=m;var s={};for(var l in t)hasOwnProperty.call(t,l)&&(s[l]=t[l]);s.originalType=e,s[f]="string"==typeof e?e:a,i[1]=s;for(var c=2;c<o;c++)i[c]=r[c];return n.createElement.apply(null,i)}return n.createElement.apply(null,r)}m.displayName="MDXCreateElement"},5507:(e,t,r)=>{r.r(t),r.d(t,{assets:()=>l,contentTitle:()=>i,default:()=>p,frontMatter:()=>o,metadata:()=>s,toc:()=>c});var n=r(7462),a=(r(7294),r(3905));const o={authors:{name:"Vangipuram Srinivasa Sarath Chandra",title:"Tech enthusiast",url:"https://github.com/Sarath191181208",image_url:"https://avatars.githubusercontent.com/u/74459981?v=4"}},i="\ud83d\udee0\ufe0f Refactor and Exec of AND",s={permalink:"/emu_8086/blog/2023/10/31/",editUrl:"https://github.com/Sarath191181208/emu_8086/tree/main/docusaurus/blog/2023-10-31.mdx",source:"@site/blog/2023-10-31.mdx",title:"\ud83d\udee0\ufe0f Refactor and Exec of AND",description:"\ud83d\udee0\ufe0f Refactor of common parsing while exec of reg as first ins",date:"2023-10-31T00:00:00.000Z",formattedDate:"October 31, 2023",tags:[],readingTime:1.215,hasTruncateMarker:!1,authors:[{name:"Vangipuram Srinivasa Sarath Chandra",title:"Tech enthusiast",url:"https://github.com/Sarath191181208",image_url:"https://avatars.githubusercontent.com/u/74459981?v=4",imageURL:"https://avatars.githubusercontent.com/u/74459981?v=4"}],frontMatter:{authors:{name:"Vangipuram Srinivasa Sarath Chandra",title:"Tech enthusiast",url:"https://github.com/Sarath191181208",image_url:"https://avatars.githubusercontent.com/u/74459981?v=4",imageURL:"https://avatars.githubusercontent.com/u/74459981?v=4"}},nextItem:{title:"Support for and instruction compilation",permalink:"/emu_8086/blog/2023/10/30/"}},l={authorsImageUrls:[void 0]},c=[{value:"\ud83d\udee0\ufe0f Refactor of common parsing while exec of reg as first ins",id:"\ufe0f-refactor-of-common-parsing-while-exec-of-reg-as-first-ins",level:2},{value:"Execution of <code>AND</code> instruction",id:"execution-of-and-instruction",level:2}],u={toc:c},f="wrapper";function p(e){let{components:t,...r}=e;return(0,a.kt)(f,(0,n.Z)({},u,r,{components:t,mdxType:"MDXLayout"}),(0,a.kt)("h2",{id:"\ufe0f-refactor-of-common-parsing-while-exec-of-reg-as-first-ins"},"\ud83d\udee0\ufe0f Refactor of common parsing while exec of reg as first ins"),(0,a.kt)("p",null,"The logic of register as first instruction is always similar for example\n",(0,a.kt)("inlineCode",{parentName:"p"},"TEST AX, BX")," -> bytes ","[ 0x85, 0xD8 ]",", ",(0,a.kt)("inlineCode",{parentName:"p"},"ADD AX, BX")," -> bytes ","[ 0x01, 0xD8 ]"," and so on.\nwhile exec 0xD8 is always the register for instructions. Therefore, the logic of parsing the register is common for all the instructions. So, I have refactored the code to make it more readable and maintainable.\nThe new functions are "),(0,a.kt)("ul",null,(0,a.kt)("li",{parentName:"ul"},(0,a.kt)("inlineCode",{parentName:"li"},"consume_bytes_and_parse_8bit_reg_as_first_arg_double_ins")),(0,a.kt)("li",{parentName:"ul"},(0,a.kt)("inlineCode",{parentName:"li"},"consume_bytes_and_parse_16bit_reg_as_first_arg_double_ins"))),(0,a.kt)("p",null,"The functions are used in the fowlling ways "),(0,a.kt)("pre",null,(0,a.kt)("code",{parentName:"pre",className:"language-rust",metastring:"showLineNumbers",showLineNumbers:!0},"// For 16bit registers\nself.consume_bytes_and_parse_16bit_reg_as_first_arg_double_ins(\n    mem,\n    &|cpu: &mut CPU, val1: u16, val2: u16| -> Option<u16> {\n        let res = val1 & val2;\n        cpu.set_test_ins_flags_from_16bit_res(res);\n        None\n    },\n)\n\n// For 8bit registers\nlet exec_fn = &|cpu: &mut CPU, val1: u8, val2: u8| -> Option<u8> {\n    let res = val1 & val2;\n    cpu.set_test_ins_flags_from_8bit_res(res);\n    None\n};\nself.consume_bytes_and_parse_8bit_reg_as_first_arg_double_ins(mem, exec_fn);\n")),(0,a.kt)("h2",{id:"execution-of-and-instruction"},"Execution of ",(0,a.kt)("inlineCode",{parentName:"h2"},"AND")," instruction"),(0,a.kt)("p",null,"The ",(0,a.kt)("inlineCode",{parentName:"p"},"AND")," instruction is executed in the following way"),(0,a.kt)("pre",null,(0,a.kt)("code",{parentName:"pre",className:"language-rust",metastring:"showLineNumbers",showLineNumbers:!0},"// For 16bit registers\nlet res = op1 & op2;\ncarry_flag = false;\noverflow_flag = false;\nzero_flag = res == 0;\nnegative_flag = res & 0x8000 != 0;\npairity_flag = (res & 0xFF).count_ones() % 2 == 0;\n\n// For 8bit registers\nlet res = op1 & op2;\ncarry_flag = false;\noverflow_flag = false;\nzero_flag = res == 0;\nnegative_flag = res & 0x80 != 0;\npairity_flag = res.count_ones() % 2 == 0;\n")))}p.isMDXComponent=!0}}]);