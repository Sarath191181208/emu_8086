"use strict";(self.webpackChunkdocs=self.webpackChunkdocs||[]).push([[6574],{4621:(e,t,n)=>{n.r(t),n.d(t,{assets:()=>l,contentTitle:()=>o,default:()=>h,frontMatter:()=>i,metadata:()=>r,toc:()=>d});var a=n(7462),s=(n(7294),n(3905));const i={authors:{name:"Vangipuram Srinivasa Sarath Chandra",title:"Tech enthusiast",url:"https://github.com/Sarath191181208",image_url:"https://avatars.githubusercontent.com/u/74459981?v=4"}},o="\ud83d\udee0\ufe0f Refactor",r={permalink:"/emu_8086/blog/2023/10/27/ Refactor",editUrl:"https://github.com/Sarath191181208/emu_8086/tree/main/docusaurus/blog/2023-10-27 Refactor.mdx",source:"@site/blog/2023-10-27 Refactor.mdx",title:"\ud83d\udee0\ufe0f Refactor",description:"\ud83d\udee0\ufe0f Refactor conditional check of variable type into the evaluate_ins function And added ByteIndexedAddressing in Assembly8086Tokens",date:"2023-10-27T00:00:00.000Z",formattedDate:"October 27, 2023",tags:[],readingTime:2.505,hasTruncateMarker:!1,authors:[{name:"Vangipuram Srinivasa Sarath Chandra",title:"Tech enthusiast",url:"https://github.com/Sarath191181208",image_url:"https://avatars.githubusercontent.com/u/74459981?v=4",imageURL:"https://avatars.githubusercontent.com/u/74459981?v=4"}],frontMatter:{authors:{name:"Vangipuram Srinivasa Sarath Chandra",title:"Tech enthusiast",url:"https://github.com/Sarath191181208",image_url:"https://avatars.githubusercontent.com/u/74459981?v=4",imageURL:"https://avatars.githubusercontent.com/u/74459981?v=4"}},prevItem:{title:"Blog CI + Indiviaual Byte/Word Indexing work",permalink:"/emu_8086/blog/2023/10/28/"},nextItem:{title:"Pop Support",permalink:"/emu_8086/blog/2023/10/26/ Added POP support"}},l={authorsImageUrls:[void 0]},d=[{value:"Refactor conditional check",id:"refactor-conditional-check",level:2},{value:"Removal of parsing chracter in parse fn and moved it into evaluate ins",id:"removal-of-parsing-chracter-in-parse-fn-and-moved-it-into-evaluate-ins",level:2},{value:"Addition of ByteIndexedAddressing in Assembly8086Tokens",id:"addition-of-byteindexedaddressing-in-assembly8086tokens",level:2},{value:"\ud83d\udcd6 DOC:  Setup github actions to automatically deploy docs",id:"-doc--setup-github-actions-to-automatically-deploy-docs",level:2}],u={toc:d},c="wrapper";function h(e){let{components:t,...n}=e;return(0,s.kt)(c,(0,a.Z)({},u,n,{components:t,mdxType:"MDXLayout"}),(0,s.kt)("p",null,"\ud83d\udee0\ufe0f Refactor conditional check of variable type into the evaluate_ins function And added ByteIndexedAddressing in Assembly8086Tokens"),(0,s.kt)("h2",{id:"refactor-conditional-check"},"Refactor conditional check"),(0,s.kt)("ul",null,(0,s.kt)("li",{parentName:"ul"},"Refactored conditional check of variable type into the evaluate_ins function\nThe code base was having this type of conditional checks for checking if the variable type is defined as Word (or) as byte\nThe fowlling is the example of what I am talking about:")),(0,s.kt)("pre",null,(0,s.kt)("code",{parentName:"pre",className:"language-rust",metastring:'title="./src-tauri/src/compiler/parsers/mov.rs" showLineNumbers',title:'"./src-tauri/src/compiler/parsers/mov.rs"',showLineNumbers:!0},"let mov_ins = if is_variable_defined_as_16bit(\n    &variable_abs_offset_map,\n    get_token_as_label(&high_token),\n) {\n    0xC7\n} else {\n    0xC6\n};\n")),(0,s.kt)("p",null,"This is a repetative logic and we can mess up quite easily therefore we have refactored this into the evaluate_ins function the following is the example of the same:"),(0,s.kt)("pre",null,(0,s.kt)("code",{parentName:"pre",className:"language-rust",metastring:'title="./src-tauri/src/compiler/parsers/pattern_extractors/utils.rs 181:5" showLineNumbers',title:'"./src-tauri/src/compiler/parsers/pattern_extractors/utils.rs','181:5"':!0,showLineNumbers:!0},"let var_type = variable_abs_address_map\n    .get(label)\n    .unwrap_or(&(VariableType::Word, 0))\n    .0;\nvariable_type = Some(var_type);\n")),(0,s.kt)("h2",{id:"removal-of-parsing-chracter-in-parse-fn-and-moved-it-into-evaluate-ins"},"Removal of parsing chracter in parse fn and moved it into evaluate ins"),(0,s.kt)("p",null,"The ",(0,s.kt)("inlineCode",{parentName:"p"},"parse_two_arguments_line")," was incharge of handing the substitution of variables and labels into their respective addresses and values. This intrun created a lot of duplicated logic and was getting hard to maintain. Thus, this logic has now been moved into the ",(0,s.kt)("inlineCode",{parentName:"p"},"evaluate_ins")," function which is now incharge of handlig the substitution of variables and labels into their respective addresses and values."),(0,s.kt)("p",null,"This is what the ",(0,s.kt)("inlineCode",{parentName:"p"},"parse_two_arguments_line")," was doing before: "),(0,s.kt)("pre",null,(0,s.kt)("code",{parentName:"pre",className:"language-rust",metastring:"showLineNumbers",showLineNumbers:!0},"match high_token{\n    match low_token{ \n        Assembly8086Tokens::Character(label) => {\n            let addr_bytes_or_num = get_label_address_or_push_into_ref();\n            match addr_bytes_or_num{\n                bytes => AddressingMode::RegisterAndAddress\n                num => AddressingMode::Registers16bitNumber\n            }\n        }\n    }\n}\n\n")),(0,s.kt)("p",null,"This logic has now been converted into the ",(0,s.kt)("inlineCode",{parentName:"p"},"evaluate_ins")," fn where it is already being done."),(0,s.kt)("h2",{id:"addition-of-byteindexedaddressing-in-assembly8086tokens"},"Addition of ByteIndexedAddressing in Assembly8086Tokens"),(0,s.kt)("p",null,"I have recently known that there exists ByteIndexedAddressing in the 8086 processor, it is a mode where you can change/access byte of the memory like when defining the variable as byte. You can also do this in a differnent way therefore to merge all of the uses into a sinlge entity to represent and match easily I have added ByteIndexedAddressing in Assembly8086Tokens. The following is the example of the same:"),(0,s.kt)("p",null,"My discovery: "),(0,s.kt)("pre",null,(0,s.kt)("code",{parentName:"pre",className:"language-asm6502",metastring:"showLineNumbers",showLineNumbers:!0},"MOV b.[BX], 0x0A ; moves 0x0A into the byte of the memory pointed by BX\nMOV w.[BX], 0x0A ; moves 0x00_0A into the word of the memory pointed by BX\n")),(0,s.kt)("p",null,"As this is the case to represent both ",(0,s.kt)("inlineCode",{parentName:"p"},"b.[BX]")," and ",(0,s.kt)("inlineCode",{parentName:"p"},"var db")," I have added ByteIndexedAddressing in Assembly8086Tokens."),(0,s.kt)("h2",{id:"-doc--setup-github-actions-to-automatically-deploy-docs"},"\ud83d\udcd6 DOC:  Setup github actions to automatically deploy docs"),(0,s.kt)("p",null,"From ",(0,s.kt)("a",{parentName:"p",href:"https://docusaurus.io/docs/deployment#deploying-to-github-pages"},"Docusaurus")," docs:\nWe have setup a github actions script that looks like this "),(0,s.kt)("pre",null,(0,s.kt)("code",{parentName:"pre",className:"language-yaml",metastring:"showLineNumbers",showLineNumbers:!0},'name: Build Docs\n\ndefaults:\n  run:\n    working-directory: ./docusaurus\n\non:\n  push:\n    branches: ["main"]\n    paths:\n      - "docusaurus/**"\n  pull_request:\n    branches: ["main"]\n    paths:\n      - "docusaurus/**"\n\npermissions:\n  contents: write\n\njobs:\n  deploy:\n    name: Deploy to GitHub Pages\n    runs-on: ubuntu-latest\n    steps:\n      - uses: actions/checkout@v3\n      - uses: actions/setup-node@v3\n        with:\n          node-version: 18\n          cache: npm \n\n      - name: Install dependencies\n        run: npm ci\n\n      - name: Build\n        run: npm run build\n\n      - name: Deploy to GitHub Pages\n        uses: peaceiris/actions-gh-pages@v3\n        with:\n            github_token: ${{ secrets.GITHUB_TOKEN }}\n            publish_dir: ./build\n            user_name: Sarath19181208[bot]\n            user_email: vssarathc04+gh_bot_emu8086@gmail.com\n')),(0,s.kt)("p",null,"To make automatically deploy docs to github pages on every push to main branch."))}h.isMDXComponent=!0}}]);