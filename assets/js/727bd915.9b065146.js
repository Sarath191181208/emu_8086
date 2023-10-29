"use strict";(self.webpackChunkdocs=self.webpackChunkdocs||[]).push([[325],{3849:e=>{e.exports=JSON.parse('{"blogPosts":[{"id":"/2023/10/29/","metadata":{"permalink":"/emu_8086/blog/2023/10/29/","editUrl":"https://github.com/Sarath191181208/emu_8086/tree/main/docusaurus/blog/2023-10-29.mdx","source":"@site/blog/2023-10-29.mdx","title":"\ud83d\udee0\ufe0f Refactor: moved common logic into a pattern","description":"For instruction like TEST, AND, MOV, ADD ... etc. All of these follow a single pattern for their compliation step example","date":"2023-10-29T00:00:00.000Z","formattedDate":"October 29, 2023","tags":[],"readingTime":0.595,"hasTruncateMarker":false,"authors":[{"name":"Vangipuram Srinivasa Sarath Chandra","title":"Tech enthusiast","url":"https://github.com/Sarath191181208","image_url":"https://avatars.githubusercontent.com/u/74459981?v=4","imageURL":"https://avatars.githubusercontent.com/u/74459981?v=4"}],"frontMatter":{"authors":{"name":"Vangipuram Srinivasa Sarath Chandra","title":"Tech enthusiast","url":"https://github.com/Sarath191181208","image_url":"https://avatars.githubusercontent.com/u/74459981?v=4","imageURL":"https://avatars.githubusercontent.com/u/74459981?v=4"}},"nextItem":{"title":"Blog CI + Indiviaual Byte/Word Indexing work","permalink":"/emu_8086/blog/2023/10/28/"}},"content":"For instruction like `TEST`, `AND`, `MOV`, `ADD` ... etc. All of these follow a single pattern for their compliation step example \\n\\n```asm6502\\nMOV AX, BX ; 0x8B 0xC3 \\nTEST AX, BX ; 0x85 0xC3 \\n\\nMOV AX, [BX] ; 0x8B 0x07\\nTEST AX, [BX] ; 0x85 0x07\\n\\nMOV AX, [BX+SI] ; 0x8B 0x00\\nTEST AX, [BX+SI] ; 0x85 0x00\\n\\nMOV AX, [BX+SI+0x1234] ; 0x8B 0x84 0x34 0x12\\nTEST AX, [BX+SI+0x1234] ; 0x85 0x84 0x34 0x12\\n```\\n\\nHere 0x8B is the root ins (or) in other words MOV and 0xC3 represents the registers. \\nAll of these instructions fowllow this pattern therefore the logic is sent into a sperate file called `src-tauri\\\\src\\\\compiler\\\\parsers\\\\pattern_extractors\\\\compile_first_ins_reg_pattern.rs` and the functions are as follows \\n- `parse_8bitreg_first_addr_mode`\\n- `parse_16bitreg_first_addr_mode`"},{"id":"/2023/10/28/","metadata":{"permalink":"/emu_8086/blog/2023/10/28/","editUrl":"https://github.com/Sarath191181208/emu_8086/tree/main/docusaurus/blog/2023-10-28.mdx","source":"@site/blog/2023-10-28.mdx","title":"Blog CI + Indiviaual Byte/Word Indexing work","description":"Blog CI","date":"2023-10-28T00:00:00.000Z","formattedDate":"October 28, 2023","tags":[],"readingTime":5.205,"hasTruncateMarker":false,"authors":[{"name":"Vangipuram Srinivasa Sarath Chandra","title":"Tech enthusiast","url":"https://github.com/Sarath191181208","image_url":"https://avatars.githubusercontent.com/u/74459981?v=4","imageURL":"https://avatars.githubusercontent.com/u/74459981?v=4"}],"frontMatter":{"authors":{"name":"Vangipuram Srinivasa Sarath Chandra","title":"Tech enthusiast","url":"https://github.com/Sarath191181208","image_url":"https://avatars.githubusercontent.com/u/74459981?v=4","imageURL":"https://avatars.githubusercontent.com/u/74459981?v=4"}},"prevItem":{"title":"\ud83d\udee0\ufe0f Refactor: moved common logic into a pattern","permalink":"/emu_8086/blog/2023/10/29/"},"nextItem":{"title":"\ud83d\udee0\ufe0f Refactor","permalink":"/emu_8086/blog/2023/10/27/ Refactor"}},"content":"## Blog CI\\n\\nMade the `docusaurus` compile in the CI/CD pipeline of github actions and made the `docusaurus` build and deploy to the `gh-pages` branch of the repository. The `gh-pages` branch is used to host the website on github pages.\\nThe website is hosted at [https://sarath191181208.github.io/](https://sarath191181208.github.io/)\\nIssues faced:\\n\\n1. The docusaurus build was working, but the blog dates were wrong this is due to the fact that we are using DD-MM-YYYY format in the blog posts and the docusaurus is parsing these dates as UTC+5:30(my local timezone) and converting it into UTC+00:00 This resulted in blogs having 1-day offset errors. This can be fixed by using the YYYY-MM-DD format which the docusaurus treats it as UTC+00:00 format thus not messing up the dates.\\n2. The github workflow file was having the publish_dir as `./build` it was actually `./docusaurus/build`, we are using working-directory in the yml but it isn\'t picked up by the action of actions-gh-pages. This was fixed by changing the publish_dir to `./docusaurus/build`\\n\\n```yaml\\ndefaults:\\n  run:\\n    working-directory: ./docusaurus\\n\\n  ########################\\n  #### workflow config ###\\n  ########################\\n\\n - name: Deploy to GitHub Pages\\n   uses: peaceiris/actions-gh-pages@v3\\n   with:\\n    github_token: ${{ secrets.GITHUB_TOKEN }}\\n# Removed\\n    - publish_dir: ./build\\n# Added\\n    - publish_dir: ./docusaurus/build\\n```\\n\\n## Individual Byte/Word Indexing\\n\\nThe syntax like using `w.[]` and `b.[]` are now working. We can use of the fowlling syntax to make the fowlling work.\\n\\n```asm6502\\n\\nORG 100h\\n.DATA\\n  var dw 0x100\\nCODE:\\n  MOV w.[var], 0x10 ; moves 0x0010 -> [var]\\n  MOV b.[var], 0x10 ; moves 0x10   -> [var]\\n```\\n\\n## Compilation of TEST instruction\\n\\n:::tip What the differnet texts mean in opcode\\n\\n- `[0x00 0x10]` means that these bytes are derived from memory and address 0x100 is used.\\n- `u8` means that the byte is derived from the immediate value and has 1byte as size.\\n- `u16` means that the word is derived from the immediate value and has 2bytes as size.\\n- `i8` means that the byte is derived from the immediate value is signed and has 1byte size.\\n- `i16` means that the word is derived from the immediate value is signed and has 2byte size.\\n- `0x00..0xFF` means that these bytes are derived from the instructions/registers.\\n\\n- `reg16` means that the register is 16-bit. Ex: `AX`, `BX`, `CX`, `DX`, `SP`, `BP`, `SI`, `DI`\\n- `reg8` means that the register is 8-bit. Ex: `AL`, `BL`, `CL`, `DL`, `AH`, `BH`, `CH`, `DH`\\n- `var16` means that the variable is 16-bit. Ex: `var dw 0x100` or `w.[0x100]`\\n- `var8` means that the variable is 8-bit. Ex: `var db 0x100` or `b.[0x100]`\\n- `idx16` means that the index is 16-bit. Ex: `w.[BX]` or `w.[SI]` or `w.[DI]`\\n- `idx8` means that the index is 8-bit. Ex: `b.[BX]` or `b.[SI]` or `b.[DI]`\\n\\n:::\\n\\nThe `TEST` instruction is now compiled and working. The `TEST` instruction is used to perform bitwise AND operation on the operands and set the flags accordingly. The `TEST` instruction is compiled into the following code.\\n\\n|          Operand          |           Opcode            | Size(bytes) |                                 Description                                  | Example instruction    |\\n| :-----------------------: | :-------------------------: | :---------: | :--------------------------------------------------------------------------: | ---------------------- |\\n|       reg16, reg16        |       0x85 0xC0..0xFF       |      2      | Performs bitwise AND operation on the operands and set the flags accordingly | `TEST AX, BX`          |\\n|        reg8, reg8         |       0x84 0xC0..0xFF       |      2      | Performs bitwise AND operation on the operands and set the flags accordingly | `TEST AL, BL`          |\\n|         AX, imm16         |       0xA9 0xC0..0xFF       |      3      | Performs bitwise AND operation on the operands and set the flags accordingly | `TEST AX, 0x10`        |\\n|         AL, imm8          |       0xA8 0xC0..0xFF       |      2      | Performs bitwise AND operation on the operands and set the flags accordingly | `TEST AL, 0x10`        |\\n|     reg16, imm16/imm8     |     0xF7 0xC0..0xFF u16     |      4      | Performs bitwise AND operation on the operands and set the flags accordingly | `TEST AX, 0x10`        |\\n|        reg8, imm8         |     0xF6 0xC0..0xFF u8      |      3      | Performs bitwise AND operation on the operands and set the flags accordingly | `TEST AL, 0x10`        |\\n| var16/reg16 , reg16/var16 | 0x85 0x06..0x36 [0x02 0x01] |      4      | Performs bitwise AND operation on the operands and set the flags accordingly | `TEST [0x100], AX`     |\\n|   var8/reg8 , reg8/var8   | 0x84 0x06..0x36 [0x02 0x01] |      4      | Performs bitwise AND operation on the operands and set the flags accordingly | `TEST [0x100], AL`     |\\n|     var16, imm16/imm8     |  0xF7 0x06 [0x00 0x10] u16  |      6      | Performs bitwise AND operation on the operands and set the flags accordingly | `TEST [0x100], 0x10`   |\\n|        var8, imm8         |  0xF6 0x06 [0x00 0x10] u8   |      5      | Performs bitwise AND operation on the operands and set the flags accordingly | `TEST [0x100], 0x10`   |\\n|       reg16, idx16        |       0x85 0x00..0x30       |      2      | Performs bitwise AND operation on the operands and set the flags accordingly | `TEST AX, [BX]`        |\\n|     reg16, idx16+imm8     |     0x85 0x40..0x70 i8      |      3      | Performs bitwise AND operation on the operands and set the flags accordingly | `TEST AX, [BX+0x10]`   |\\n|    reg16, idx16+imm16     |     0x85 0x80..0xB0 i16     |      4      | Performs bitwise AND operation on the operands and set the flags accordingly | `TEST AX, [BX+0x1000]` |\\n\\n## Execution of the test instruction\\n\\nMade the `TEST` instruction execute and set the flags accordingly. The result isn\'t stored anywhere, only the flags are chaged.\\nAlgorithm\\n\\n```rust\\n\\n// For 16-bit operands\\nlet res = op1 & op2;\\ncarry_flag = false;\\noverflow_flag = false;\\nzero_flag = res == 0;\\nnegative_flag = res & 0x8000 != 0;\\npairity_flag = res.count_ones() % 2 == 0;\\n\\n// For 8-bit operands\\nlet res = op1 & op2;\\ncarry_flag = false;\\noverflow_flag = false;\\nzero_flag = res == 0;\\nnegative_flag = res & 0x80 != 0;\\npairity_flag = res.count_ones() % 2 == 0;\\n\\n```\\n\\n## Testing deprecation of functions \\nSome of the old functions which don\'t use best practices are deprecated and the new functions are used. \\nThe functions which are deprecated are:\\n- `generate_test` macro\\n- `generate_test_with_cycles` macro \\n- `compile_and_test_str` function\\n\\nIn favour of these convoluted function a new api is created for testing. It is both easy and simple to use. The new api is as follows:\\n```rust\\nlet code = \\"MOV AX, BX\\"\\nlet num_cycles = 1;\\nrun_code(code, num_cycles)\\n```"},{"id":"/2023/10/27/ Refactor","metadata":{"permalink":"/emu_8086/blog/2023/10/27/ Refactor","editUrl":"https://github.com/Sarath191181208/emu_8086/tree/main/docusaurus/blog/2023-10-27 Refactor.mdx","source":"@site/blog/2023-10-27 Refactor.mdx","title":"\ud83d\udee0\ufe0f Refactor","description":"\ud83d\udee0\ufe0f Refactor conditional check of variable type into the evaluate_ins function And added ByteIndexedAddressing in Assembly8086Tokens","date":"2023-10-27T00:00:00.000Z","formattedDate":"October 27, 2023","tags":[],"readingTime":2.49,"hasTruncateMarker":false,"authors":[{"name":"Vangipuram Srinivasa Sarath Chandra","title":"Tech enthusiast","url":"https://github.com/Sarath191181208","image_url":"https://avatars.githubusercontent.com/u/74459981?v=4","imageURL":"https://avatars.githubusercontent.com/u/74459981?v=4"}],"frontMatter":{"authors":{"name":"Vangipuram Srinivasa Sarath Chandra","title":"Tech enthusiast","url":"https://github.com/Sarath191181208","image_url":"https://avatars.githubusercontent.com/u/74459981?v=4","imageURL":"https://avatars.githubusercontent.com/u/74459981?v=4"}},"prevItem":{"title":"Blog CI + Indiviaual Byte/Word Indexing work","permalink":"/emu_8086/blog/2023/10/28/"},"nextItem":{"title":"Pop Support","permalink":"/emu_8086/blog/2023/10/26/ Added POP support"}},"content":"\ud83d\udee0\ufe0f Refactor conditional check of variable type into the evaluate_ins function And added ByteIndexedAddressing in Assembly8086Tokens\\n\\n## Refactor conditional check\\n\\n- Refactored conditional check of variable type into the evaluate_ins function\\n  The code base was having this type of conditional checks for checking if the variable type is defined as Word (or) as byte\\n  The fowlling is the example of what I am talking about:\\n\\n```rust\\n// ./src-tauri/src/compiler/parsers/mov.rs\\nlet mov_ins = if is_variable_defined_as_16bit(\\n    &variable_abs_offset_map,\\n    get_token_as_label(&high_token),\\n) {\\n    0xC7\\n} else {\\n    0xC6\\n};\\n```\\n\\nThis is a repetative logic and we can mess up quite easily therefore we have refactored this into the evaluate_ins function the following is the example of the same:\\n\\n```rust\\n// ./src-tauri/src/compiler/parsers/pattern_extractors/utils.rs 181:5\\nlet var_type = variable_abs_address_map\\n    .get(label)\\n    .unwrap_or(&(VariableType::Word, 0))\\n    .0;\\nvariable_type = Some(var_type);\\n```\\n\\n## Removal of parsing chracter in parse fn and moved it into evaluate ins\\n\\nThe `parse_two_arguments_line` was incharge of handing the substitution of variables and labels into their respective addresses and values. This intrun created a lot of duplicated logic and was getting hard to maintain. Thus, this logic has now been moved into the `evaluate_ins` function which is now incharge of handlig the substitution of variables and labels into their respective addresses and values.\\n\\nThis is what the `parse_two_arguments_line` was doing before: \\n```rust \\nmatch high_token{\\n    match low_token{ \\n        Assembly8086Tokens::Character(label) => {\\n            let addr_bytes_or_num = get_label_address_or_push_into_ref();\\n            match addr_bytes_or_num{\\n                bytes => AddressingMode::RegisterAndAddress\\n                num => AddressingMode::Registers16bitNumber\\n            }\\n        }\\n    }\\n}\\n\\n```\\n\\nThis logic has now been converted into the `evaluate_ins` fn where it is already being done.\\n\\n## Addition of ByteIndexedAddressing in Assembly8086Tokens\\n\\nI have recently known that there exists ByteIndexedAddressing in the 8086 processor, it is a mode where you can change/access byte of the memory like when defining the variable as byte. You can also do this in a differnent way therefore to merge all of the uses into a sinlge entity to represent and match easily I have added ByteIndexedAddressing in Assembly8086Tokens. The following is the example of the same:\\n\\nMy discovery: \\n```asm6502\\nMOV b.[BX], 0x0A ; moves 0x0A into the byte of the memory pointed by BX\\nMOV w.[BX], 0x0A ; moves 0x00_0A into the word of the memory pointed by BX\\n```\\n\\nAs this is the case to represent both `b.[BX]` and `var db` I have added ByteIndexedAddressing in Assembly8086Tokens.\\n\\n## \ud83d\udcd6 DOC:  Setup github actions to automatically deploy docs \\n\\nFrom [Docusaurus](https://docusaurus.io/docs/deployment#deploying-to-github-pages) docs: \\nWe have setup a github actions script that looks like this \\n```yaml \\nname: Build Docs\\n\\ndefaults:\\n  run:\\n    working-directory: ./docusaurus\\n\\non:\\n  push:\\n    branches: [\\"main\\"]\\n    paths:\\n      - \\"docusaurus/**\\"\\n  pull_request:\\n    branches: [\\"main\\"]\\n    paths:\\n      - \\"docusaurus/**\\"\\n\\npermissions:\\n  contents: write\\n\\njobs:\\n  deploy:\\n    name: Deploy to GitHub Pages\\n    runs-on: ubuntu-latest\\n    steps:\\n      - uses: actions/checkout@v3\\n      - uses: actions/setup-node@v3\\n        with:\\n          node-version: 18\\n          cache: npm \\n\\n      - name: Install dependencies\\n        run: npm ci\\n\\n      - name: Build\\n        run: npm run build\\n\\n      - name: Deploy to GitHub Pages\\n        uses: peaceiris/actions-gh-pages@v3\\n        with:\\n            github_token: ${{ secrets.GITHUB_TOKEN }}\\n            publish_dir: ./build\\n            user_name: Sarath19181208[bot]\\n            user_email: vssarathc04+gh_bot_emu8086@gmail.com\\n``` \\nTo make automatically deploy docs to github pages on every push to main branch."},{"id":"/2023/10/26/ Added POP support","metadata":{"permalink":"/emu_8086/blog/2023/10/26/ Added POP support","editUrl":"https://github.com/Sarath191181208/emu_8086/tree/main/docusaurus/blog/2023-10-26 Added POP support.mdx","source":"@site/blog/2023-10-26 Added POP support.mdx","title":"Pop Support","description":"Compilation of the pop instruction","date":"2023-10-26T00:00:00.000Z","formattedDate":"October 26, 2023","tags":[],"readingTime":1.04,"hasTruncateMarker":false,"authors":[{"name":"Vangipuram Srinivasa Sarath Chandra","title":"Tech enthusiast","url":"https://github.com/Sarath191181208","image_url":"https://avatars.githubusercontent.com/u/74459981?v=4","imageURL":"https://avatars.githubusercontent.com/u/74459981?v=4"}],"frontMatter":{"authors":{"name":"Vangipuram Srinivasa Sarath Chandra","title":"Tech enthusiast","url":"https://github.com/Sarath191181208","image_url":"https://avatars.githubusercontent.com/u/74459981?v=4","imageURL":"https://avatars.githubusercontent.com/u/74459981?v=4"}},"prevItem":{"title":"\ud83d\udee0\ufe0f Refactor","permalink":"/emu_8086/blog/2023/10/27/ Refactor"}},"content":"## Compilation of the pop instruction\\n\\n|         Operand          |          Opcode          |                       Description                       | Example instruction |\\n| :----------------------: | :----------------------: | :-----------------------------------------------------: | :-----------------: |\\n|          reg16           |          0x58+rw           |               Pop top of stack into reg16               |       ` pop ax `        |\\n|          mem16           |    0x8F 0x06 16BIT-addr    |               Pop top of stack into mem16               |    `pop [0x0100]`     |\\n|  indexed with no offset  |      0x8F 0x00..0x07       |        Pop top of stack into index given by regs        |     `pop [bx+si]`     |\\n| indexed with byte offset | 0x8F 0x40..0x47 16bit-addr | Pop top of stack into index given by regs + byte offset |  `pop [bx+0x01]`   |\\n| indexed with word offset | 0x8F 0x80..0x87 16bit-addr | Pop top of stack into index given by regs + word offset | `pop [bx+0x0100]`  |\\n\\n## Execution of the pop instruction\\n\\nMade the `pop` instruction execution working.\\nAddressing modes of the `pop` instruction implemented are:\\n\\n- Register addressing mode, ex - `pop ax`\\n- Direct addressing mode, ex - `pop [0x1234]`\\n- Variable addressing mode, ex - `pop [var]`\\n- Indirect addressing mode, ex - `pop [bx]`\\n- Indexed addressing mode, ex - `pop [bx+si]`\\n\\nAlgorithm:\\n```\\noperand = SS:[SP] (top of the stack)\\nSP = SP + 2\\n```"}]}')}}]);