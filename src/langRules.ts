import { editor, languages } from "monaco-editor/esm/vs/editor/editor.api";

export const langRules: languages.IMonarchLanguage = {
  defaultToken: "",
  ignoreCase: true,
  keywords: ["mov", "add", "sub"],
  tokenizer: {
    root: [
      [
        /[a-zA-Z_]\w*/,
        {
          cases: {
            "@keywords": "keyword",
            "@default": "identifier",
          },
        },
      ],
      [/[ \t\r\n]+/, "white"],
      [/;.*/, "comment"],
      //   write support for numbers hex numbers
      [/0[xX][0-9a-fA-F]+/, "number.hex"],
      [/[0-9a-fA-F]+[hH]/, "number.hex"],
      [/\d+/, "number"],
      // also if the last character is a h then it is a hex number
    ],
  },
};

export const langTheme: editor.IStandaloneThemeData = {
  base: "vs-dark",
  inherit: true,
  rules: [
    { token: "keyword", foreground: "#569CD6" },
    { token: "identifier", foreground: "#9CDCFE" },
    { token: "comment", foreground: "#6A9955" },
    { token: "number", foreground: "#B5CEA8" },
    { token: "number.hex", foreground: "#B5CEA8" },
    { token: "white", foreground: "#FFFFFF" },
  ],
  colors: {
    "editor.background": "#172033",
    "editor.foreground": "#D4D4D4",
    "editorCursor.foreground": "#AEAFAD",
    "editor.lineHighlightBackground": "#1e293b",
    "editorLineNumber.foreground": "#5A5A5A",
    "editor.selectionBackground": "#264F78",
    "editor.inactiveSelectionBackground": "#3A3D41",
    "editorWhitespace.foreground": "#3B4048",
  },
};
