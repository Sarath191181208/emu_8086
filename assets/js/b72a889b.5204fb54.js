"use strict";(self.webpackChunkdocs=self.webpackChunkdocs||[]).push([[939],{3905:(e,t,n)=>{n.d(t,{Zo:()=>d,kt:()=>g});var a=n(7294);function r(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function i(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var a=Object.getOwnPropertySymbols(e);t&&(a=a.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,a)}return n}function o(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?i(Object(n),!0).forEach((function(t){r(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):i(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function l(e,t){if(null==e)return{};var n,a,r=function(e,t){if(null==e)return{};var n,a,r={},i=Object.keys(e);for(a=0;a<i.length;a++)n=i[a],t.indexOf(n)>=0||(r[n]=e[n]);return r}(e,t);if(Object.getOwnPropertySymbols){var i=Object.getOwnPropertySymbols(e);for(a=0;a<i.length;a++)n=i[a],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(r[n]=e[n])}return r}var p=a.createContext({}),s=function(e){var t=a.useContext(p),n=t;return e&&(n="function"==typeof e?e(t):o(o({},t),e)),n},d=function(e){var t=s(e.components);return a.createElement(p.Provider,{value:t},e.children)},m="mdxType",c={inlineCode:"code",wrapper:function(e){var t=e.children;return a.createElement(a.Fragment,{},t)}},u=a.forwardRef((function(e,t){var n=e.components,r=e.mdxType,i=e.originalType,p=e.parentName,d=l(e,["components","mdxType","originalType","parentName"]),m=s(n),u=r,g=m["".concat(p,".").concat(u)]||m[u]||c[u]||i;return n?a.createElement(g,o(o({ref:t},d),{},{components:n})):a.createElement(g,o({ref:t},d))}));function g(e,t){var n=arguments,r=t&&t.mdxType;if("string"==typeof e||r){var i=n.length,o=new Array(i);o[0]=u;var l={};for(var p in t)hasOwnProperty.call(t,p)&&(l[p]=t[p]);l.originalType=e,l[m]="string"==typeof e?e:r,o[1]=l;for(var s=2;s<i;s++)o[s]=n[s];return a.createElement.apply(null,o)}return a.createElement.apply(null,n)}u.displayName="MDXCreateElement"},9854:(e,t,n)=>{n.r(t),n.d(t,{assets:()=>p,contentTitle:()=>o,default:()=>c,frontMatter:()=>i,metadata:()=>l,toc:()=>s});var a=n(7462),r=(n(7294),n(3905));const i={authors:{name:"Vangipuram Srinivasa Sarath Chandra",title:"Tech enthusiast",url:"https://github.com/Sarath191181208",image_url:"https://avatars.githubusercontent.com/u/74459981?v=4"}},o="Blog CI + Indiviaual Byte/Word Indexing work",l={permalink:"/emu_8086/blog/2023/10/28/",editUrl:"https://github.com/Sarath191181208/emu_8086/tree/main/docusaurus/blog/2023-10-28.mdx",source:"@site/blog/2023-10-28.mdx",title:"Blog CI + Indiviaual Byte/Word Indexing work",description:"Blog CI",date:"2023-10-28T00:00:00.000Z",formattedDate:"October 28, 2023",tags:[],readingTime:5.225,hasTruncateMarker:!1,authors:[{name:"Vangipuram Srinivasa Sarath Chandra",title:"Tech enthusiast",url:"https://github.com/Sarath191181208",image_url:"https://avatars.githubusercontent.com/u/74459981?v=4",imageURL:"https://avatars.githubusercontent.com/u/74459981?v=4"}],frontMatter:{authors:{name:"Vangipuram Srinivasa Sarath Chandra",title:"Tech enthusiast",url:"https://github.com/Sarath191181208",image_url:"https://avatars.githubusercontent.com/u/74459981?v=4",imageURL:"https://avatars.githubusercontent.com/u/74459981?v=4"}},prevItem:{title:"\ud83d\udee0\ufe0f Refactor: moved common logic into a pattern",permalink:"/emu_8086/blog/2023/10/29/"},nextItem:{title:"\ud83d\udee0\ufe0f Refactor",permalink:"/emu_8086/blog/2023/10/27/ Refactor"}},p={authorsImageUrls:[void 0]},s=[{value:"Blog CI",id:"blog-ci",level:2},{value:"Individual Byte/Word Indexing",id:"individual-byteword-indexing",level:2},{value:"Compilation of TEST instruction",id:"compilation-of-test-instruction",level:2},{value:"Execution of the test instruction",id:"execution-of-the-test-instruction",level:2},{value:"Testing deprecation of functions",id:"testing-deprecation-of-functions",level:2}],d={toc:s},m="wrapper";function c(e){let{components:t,...n}=e;return(0,r.kt)(m,(0,a.Z)({},d,n,{components:t,mdxType:"MDXLayout"}),(0,r.kt)("h2",{id:"blog-ci"},"Blog CI"),(0,r.kt)("p",null,"Made the ",(0,r.kt)("inlineCode",{parentName:"p"},"docusaurus")," compile in the CI/CD pipeline of github actions and made the ",(0,r.kt)("inlineCode",{parentName:"p"},"docusaurus")," build and deploy to the ",(0,r.kt)("inlineCode",{parentName:"p"},"gh-pages")," branch of the repository. The ",(0,r.kt)("inlineCode",{parentName:"p"},"gh-pages")," branch is used to host the website on github pages.\nThe website is hosted at ",(0,r.kt)("a",{parentName:"p",href:"https://sarath191181208.github.io/"},"https://sarath191181208.github.io/"),"\nIssues faced:"),(0,r.kt)("ol",null,(0,r.kt)("li",{parentName:"ol"},"The docusaurus build was working, but the blog dates were wrong this is due to the fact that we are using DD-MM-YYYY format in the blog posts and the docusaurus is parsing these dates as UTC+5:30(my local timezone) and converting it into UTC+00:00 This resulted in blogs having 1-day offset errors. This can be fixed by using the YYYY-MM-DD format which the docusaurus treats it as UTC+00:00 format thus not messing up the dates."),(0,r.kt)("li",{parentName:"ol"},"The github workflow file was having the publish_dir as ",(0,r.kt)("inlineCode",{parentName:"li"},"./build")," it was actually ",(0,r.kt)("inlineCode",{parentName:"li"},"./docusaurus/build"),", we are using working-directory in the yml but it isn't picked up by the action of actions-gh-pages. This was fixed by changing the publish_dir to ",(0,r.kt)("inlineCode",{parentName:"li"},"./docusaurus/build"))),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-yaml",metastring:"showLineNumbers",showLineNumbers:!0},"defaults:\n  run:\n    working-directory: ./docusaurus\n\n  ########################\n  #### workflow config ###\n  ########################\n\n - name: Deploy to GitHub Pages\n   uses: peaceiris/actions-gh-pages@v3\n   with:\n    github_token: ${{ secrets.GITHUB_TOKEN }}\n# Removed\n    - publish_dir: ./build\n# Added\n    - publish_dir: ./docusaurus/build\n")),(0,r.kt)("h2",{id:"individual-byteword-indexing"},"Individual Byte/Word Indexing"),(0,r.kt)("p",null,"The syntax like using ",(0,r.kt)("inlineCode",{parentName:"p"},"w.[]")," and ",(0,r.kt)("inlineCode",{parentName:"p"},"b.[]")," are now working. We can use of the fowlling syntax to make the fowlling work."),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-asm6502",metastring:"showLineNumbers",showLineNumbers:!0},"\nORG 100h\n.DATA\n  var dw 0x100\nCODE:\n  MOV w.[var], 0x10 ; moves 0x0010 -> [var]\n  MOV b.[var], 0x10 ; moves 0x10   -> [var]\n")),(0,r.kt)("h2",{id:"compilation-of-test-instruction"},"Compilation of TEST instruction"),(0,r.kt)("admonition",{title:"What the differnet texts mean in opcode",type:"tip"},(0,r.kt)("ul",{parentName:"admonition"},(0,r.kt)("li",{parentName:"ul"},(0,r.kt)("p",{parentName:"li"},(0,r.kt)("inlineCode",{parentName:"p"},"[0x00 0x10]")," means that these bytes are derived from memory and address 0x100 is used.")),(0,r.kt)("li",{parentName:"ul"},(0,r.kt)("p",{parentName:"li"},(0,r.kt)("inlineCode",{parentName:"p"},"u8")," means that the byte is derived from the immediate value and has 1byte as size.")),(0,r.kt)("li",{parentName:"ul"},(0,r.kt)("p",{parentName:"li"},(0,r.kt)("inlineCode",{parentName:"p"},"u16")," means that the word is derived from the immediate value and has 2bytes as size.")),(0,r.kt)("li",{parentName:"ul"},(0,r.kt)("p",{parentName:"li"},(0,r.kt)("inlineCode",{parentName:"p"},"i8")," means that the byte is derived from the immediate value is signed and has 1byte size.")),(0,r.kt)("li",{parentName:"ul"},(0,r.kt)("p",{parentName:"li"},(0,r.kt)("inlineCode",{parentName:"p"},"i16")," means that the word is derived from the immediate value is signed and has 2byte size.")),(0,r.kt)("li",{parentName:"ul"},(0,r.kt)("p",{parentName:"li"},(0,r.kt)("inlineCode",{parentName:"p"},"0x00..0xFF")," means that these bytes are derived from the instructions/registers.")),(0,r.kt)("li",{parentName:"ul"},(0,r.kt)("p",{parentName:"li"},(0,r.kt)("inlineCode",{parentName:"p"},"reg16")," means that the register is 16-bit. Ex: ",(0,r.kt)("inlineCode",{parentName:"p"},"AX"),", ",(0,r.kt)("inlineCode",{parentName:"p"},"BX"),", ",(0,r.kt)("inlineCode",{parentName:"p"},"CX"),", ",(0,r.kt)("inlineCode",{parentName:"p"},"DX"),", ",(0,r.kt)("inlineCode",{parentName:"p"},"SP"),", ",(0,r.kt)("inlineCode",{parentName:"p"},"BP"),", ",(0,r.kt)("inlineCode",{parentName:"p"},"SI"),", ",(0,r.kt)("inlineCode",{parentName:"p"},"DI"))),(0,r.kt)("li",{parentName:"ul"},(0,r.kt)("p",{parentName:"li"},(0,r.kt)("inlineCode",{parentName:"p"},"reg8")," means that the register is 8-bit. Ex: ",(0,r.kt)("inlineCode",{parentName:"p"},"AL"),", ",(0,r.kt)("inlineCode",{parentName:"p"},"BL"),", ",(0,r.kt)("inlineCode",{parentName:"p"},"CL"),", ",(0,r.kt)("inlineCode",{parentName:"p"},"DL"),", ",(0,r.kt)("inlineCode",{parentName:"p"},"AH"),", ",(0,r.kt)("inlineCode",{parentName:"p"},"BH"),", ",(0,r.kt)("inlineCode",{parentName:"p"},"CH"),", ",(0,r.kt)("inlineCode",{parentName:"p"},"DH"))),(0,r.kt)("li",{parentName:"ul"},(0,r.kt)("p",{parentName:"li"},(0,r.kt)("inlineCode",{parentName:"p"},"var16")," means that the variable is 16-bit. Ex: ",(0,r.kt)("inlineCode",{parentName:"p"},"var dw 0x100")," or ",(0,r.kt)("inlineCode",{parentName:"p"},"w.[0x100]"))),(0,r.kt)("li",{parentName:"ul"},(0,r.kt)("p",{parentName:"li"},(0,r.kt)("inlineCode",{parentName:"p"},"var8")," means that the variable is 8-bit. Ex: ",(0,r.kt)("inlineCode",{parentName:"p"},"var db 0x100")," or ",(0,r.kt)("inlineCode",{parentName:"p"},"b.[0x100]"))),(0,r.kt)("li",{parentName:"ul"},(0,r.kt)("p",{parentName:"li"},(0,r.kt)("inlineCode",{parentName:"p"},"idx16")," means that the index is 16-bit. Ex: ",(0,r.kt)("inlineCode",{parentName:"p"},"w.[BX]")," or ",(0,r.kt)("inlineCode",{parentName:"p"},"w.[SI]")," or ",(0,r.kt)("inlineCode",{parentName:"p"},"w.[DI]"))),(0,r.kt)("li",{parentName:"ul"},(0,r.kt)("p",{parentName:"li"},(0,r.kt)("inlineCode",{parentName:"p"},"idx8")," means that the index is 8-bit. Ex: ",(0,r.kt)("inlineCode",{parentName:"p"},"b.[BX]")," or ",(0,r.kt)("inlineCode",{parentName:"p"},"b.[SI]")," or ",(0,r.kt)("inlineCode",{parentName:"p"},"b.[DI]"))))),(0,r.kt)("p",null,"The ",(0,r.kt)("inlineCode",{parentName:"p"},"TEST")," instruction is now compiled and working. The ",(0,r.kt)("inlineCode",{parentName:"p"},"TEST")," instruction is used to perform bitwise AND operation on the operands and set the flags accordingly. The ",(0,r.kt)("inlineCode",{parentName:"p"},"TEST")," instruction is compiled into the following code."),(0,r.kt)("table",null,(0,r.kt)("thead",{parentName:"table"},(0,r.kt)("tr",{parentName:"thead"},(0,r.kt)("th",{parentName:"tr",align:"center"},"Operand"),(0,r.kt)("th",{parentName:"tr",align:"center"},"Opcode"),(0,r.kt)("th",{parentName:"tr",align:"center"},"Size(bytes)"),(0,r.kt)("th",{parentName:"tr",align:"center"},"Description"),(0,r.kt)("th",{parentName:"tr",align:null},"Example instruction"))),(0,r.kt)("tbody",{parentName:"table"},(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:"center"},"reg16, reg16"),(0,r.kt)("td",{parentName:"tr",align:"center"},"0x85 0xC0..0xFF"),(0,r.kt)("td",{parentName:"tr",align:"center"},"2"),(0,r.kt)("td",{parentName:"tr",align:"center"},"Performs bitwise AND operation on the operands and set the flags accordingly"),(0,r.kt)("td",{parentName:"tr",align:null},(0,r.kt)("inlineCode",{parentName:"td"},"TEST AX, BX"))),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:"center"},"reg8, reg8"),(0,r.kt)("td",{parentName:"tr",align:"center"},"0x84 0xC0..0xFF"),(0,r.kt)("td",{parentName:"tr",align:"center"},"2"),(0,r.kt)("td",{parentName:"tr",align:"center"},"Performs bitwise AND operation on the operands and set the flags accordingly"),(0,r.kt)("td",{parentName:"tr",align:null},(0,r.kt)("inlineCode",{parentName:"td"},"TEST AL, BL"))),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:"center"},"AX, imm16"),(0,r.kt)("td",{parentName:"tr",align:"center"},"0xA9 0xC0..0xFF"),(0,r.kt)("td",{parentName:"tr",align:"center"},"3"),(0,r.kt)("td",{parentName:"tr",align:"center"},"Performs bitwise AND operation on the operands and set the flags accordingly"),(0,r.kt)("td",{parentName:"tr",align:null},(0,r.kt)("inlineCode",{parentName:"td"},"TEST AX, 0x10"))),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:"center"},"AL, imm8"),(0,r.kt)("td",{parentName:"tr",align:"center"},"0xA8 0xC0..0xFF"),(0,r.kt)("td",{parentName:"tr",align:"center"},"2"),(0,r.kt)("td",{parentName:"tr",align:"center"},"Performs bitwise AND operation on the operands and set the flags accordingly"),(0,r.kt)("td",{parentName:"tr",align:null},(0,r.kt)("inlineCode",{parentName:"td"},"TEST AL, 0x10"))),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:"center"},"reg16, imm16/imm8"),(0,r.kt)("td",{parentName:"tr",align:"center"},"0xF7 0xC0..0xFF u16"),(0,r.kt)("td",{parentName:"tr",align:"center"},"4"),(0,r.kt)("td",{parentName:"tr",align:"center"},"Performs bitwise AND operation on the operands and set the flags accordingly"),(0,r.kt)("td",{parentName:"tr",align:null},(0,r.kt)("inlineCode",{parentName:"td"},"TEST AX, 0x10"))),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:"center"},"reg8, imm8"),(0,r.kt)("td",{parentName:"tr",align:"center"},"0xF6 0xC0..0xFF u8"),(0,r.kt)("td",{parentName:"tr",align:"center"},"3"),(0,r.kt)("td",{parentName:"tr",align:"center"},"Performs bitwise AND operation on the operands and set the flags accordingly"),(0,r.kt)("td",{parentName:"tr",align:null},(0,r.kt)("inlineCode",{parentName:"td"},"TEST AL, 0x10"))),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:"center"},"var16/reg16 , reg16/var16"),(0,r.kt)("td",{parentName:"tr",align:"center"},"0x85 0x06..0x36 ","[0x02 0x01]"),(0,r.kt)("td",{parentName:"tr",align:"center"},"4"),(0,r.kt)("td",{parentName:"tr",align:"center"},"Performs bitwise AND operation on the operands and set the flags accordingly"),(0,r.kt)("td",{parentName:"tr",align:null},(0,r.kt)("inlineCode",{parentName:"td"},"TEST [0x100], AX"))),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:"center"},"var8/reg8 , reg8/var8"),(0,r.kt)("td",{parentName:"tr",align:"center"},"0x84 0x06..0x36 ","[0x02 0x01]"),(0,r.kt)("td",{parentName:"tr",align:"center"},"4"),(0,r.kt)("td",{parentName:"tr",align:"center"},"Performs bitwise AND operation on the operands and set the flags accordingly"),(0,r.kt)("td",{parentName:"tr",align:null},(0,r.kt)("inlineCode",{parentName:"td"},"TEST [0x100], AL"))),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:"center"},"var16, imm16/imm8"),(0,r.kt)("td",{parentName:"tr",align:"center"},"0xF7 0x06 ","[0x00 0x10]"," u16"),(0,r.kt)("td",{parentName:"tr",align:"center"},"6"),(0,r.kt)("td",{parentName:"tr",align:"center"},"Performs bitwise AND operation on the operands and set the flags accordingly"),(0,r.kt)("td",{parentName:"tr",align:null},(0,r.kt)("inlineCode",{parentName:"td"},"TEST [0x100], 0x10"))),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:"center"},"var8, imm8"),(0,r.kt)("td",{parentName:"tr",align:"center"},"0xF6 0x06 ","[0x00 0x10]"," u8"),(0,r.kt)("td",{parentName:"tr",align:"center"},"5"),(0,r.kt)("td",{parentName:"tr",align:"center"},"Performs bitwise AND operation on the operands and set the flags accordingly"),(0,r.kt)("td",{parentName:"tr",align:null},(0,r.kt)("inlineCode",{parentName:"td"},"TEST [0x100], 0x10"))),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:"center"},"reg16, idx16"),(0,r.kt)("td",{parentName:"tr",align:"center"},"0x85 0x00..0x30"),(0,r.kt)("td",{parentName:"tr",align:"center"},"2"),(0,r.kt)("td",{parentName:"tr",align:"center"},"Performs bitwise AND operation on the operands and set the flags accordingly"),(0,r.kt)("td",{parentName:"tr",align:null},(0,r.kt)("inlineCode",{parentName:"td"},"TEST AX, [BX]"))),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:"center"},"reg16, idx16+imm8"),(0,r.kt)("td",{parentName:"tr",align:"center"},"0x85 0x40..0x70 i8"),(0,r.kt)("td",{parentName:"tr",align:"center"},"3"),(0,r.kt)("td",{parentName:"tr",align:"center"},"Performs bitwise AND operation on the operands and set the flags accordingly"),(0,r.kt)("td",{parentName:"tr",align:null},(0,r.kt)("inlineCode",{parentName:"td"},"TEST AX, [BX+0x10]"))),(0,r.kt)("tr",{parentName:"tbody"},(0,r.kt)("td",{parentName:"tr",align:"center"},"reg16, idx16+imm16"),(0,r.kt)("td",{parentName:"tr",align:"center"},"0x85 0x80..0xB0 i16"),(0,r.kt)("td",{parentName:"tr",align:"center"},"4"),(0,r.kt)("td",{parentName:"tr",align:"center"},"Performs bitwise AND operation on the operands and set the flags accordingly"),(0,r.kt)("td",{parentName:"tr",align:null},(0,r.kt)("inlineCode",{parentName:"td"},"TEST AX, [BX+0x1000]"))))),(0,r.kt)("h2",{id:"execution-of-the-test-instruction"},"Execution of the test instruction"),(0,r.kt)("p",null,"Made the ",(0,r.kt)("inlineCode",{parentName:"p"},"TEST")," instruction execute and set the flags accordingly. The result isn't stored anywhere, only the flags are chaged.\nAlgorithm"),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust",metastring:"showLineNumbers",showLineNumbers:!0},"\n// For 16-bit operands\nlet res = op1 & op2;\ncarry_flag = false;\noverflow_flag = false;\nzero_flag = res == 0;\nnegative_flag = res & 0x8000 != 0;\npairity_flag = res.count_ones() % 2 == 0;\n\n// For 8-bit operands\nlet res = op1 & op2;\ncarry_flag = false;\noverflow_flag = false;\nzero_flag = res == 0;\nnegative_flag = res & 0x80 != 0;\npairity_flag = res.count_ones() % 2 == 0;\n\n")),(0,r.kt)("h2",{id:"testing-deprecation-of-functions"},"Testing deprecation of functions"),(0,r.kt)("p",null,"Some of the old functions which don't use best practices are deprecated and the new functions are used.\nThe functions which are deprecated are:"),(0,r.kt)("ul",null,(0,r.kt)("li",{parentName:"ul"},(0,r.kt)("inlineCode",{parentName:"li"},"generate_test")," macro"),(0,r.kt)("li",{parentName:"ul"},(0,r.kt)("inlineCode",{parentName:"li"},"generate_test_with_cycles")," macro "),(0,r.kt)("li",{parentName:"ul"},(0,r.kt)("inlineCode",{parentName:"li"},"compile_and_test_str")," function")),(0,r.kt)("p",null,"In favour of these convoluted function a new api is created for testing. It is both easy and simple to use. The new api is as follows:"),(0,r.kt)("pre",null,(0,r.kt)("code",{parentName:"pre",className:"language-rust",metastring:"showLineNumbers",showLineNumbers:!0},'let code = "MOV AX, BX"\nlet num_cycles = 1;\nrun_code(code, num_cycles)\n')))}c.isMDXComponent=!0}}]);