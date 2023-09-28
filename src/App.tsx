import "./App.css";
import "./index.css";
import { Editor } from "@monaco-editor/react";

import {
  langConfiguration,
  langRules,
  langTheme,
} from "./langRules";
import { Navbar } from "./Components/Navbar";
import { RegistersTableView } from "./Components/RegisterView";
import { MemoryBottomBar } from "./Components/MemoryBottombar";
import { useApp } from "./hooks/useApp";

function App() {
  const {
    registers,
    prevRegistersRef,
    flags,
    memory,
    showMemoryBottomBar,
    prevMemoryRef,
    editorRef,
    monacoRef,

    languageCompletionProvider,

    compileCode,
    nextPressed,
    setIsMemoryShown,
    tryCompile,
  } = useApp();

  return (
    <>
      <Navbar
        compileCode={compileCode}
        nextPressed={nextPressed}
        className="mb-5"
      />
      <div className="flex gap-4 overflow-hidden">
        <div className="relative col-span-4 w-full">
          <Editor
            beforeMount={(monaco) => {
              monaco.editor.defineTheme("assembly-dark", langTheme);
            }}
            onMount={(editor, monaco) => {
              editorRef.current = editor;
              monacoRef.current = monaco;
              monaco.languages.register({ id: "assembly" });
              monaco.languages.setMonarchTokensProvider("assembly", langRules);
              monaco.languages.registerCompletionItemProvider(
                "assembly",
                languageCompletionProvider
              );
              monaco.languages.setLanguageConfiguration(
                "assembly",
                langConfiguration
              );
            }}
            onChange={tryCompile}
            height="100%"
            defaultLanguage="assembly"
            theme="assembly-dark"
            options={{ minimap: { enabled: false } }}
            defaultValue={
              "ORG 100H\n\n .DATA \n Var dw 0x1000 \n code: MOV AX, Var"
            }
          />
          {/* create a toggle button that creates a white screen when pressed that's on top of editor */}
          <MemoryBottomBar
            key="memory-bottom-bar"
            prevMemAddrValueMap={prevMemoryRef.current}
            memAddrValueMap={memory}
            showMemoryBottomBar={showMemoryBottomBar}
            setIsMemoryShown={setIsMemoryShown}
          />
        </div>
        <RegistersTableView
          registers={registers}
          prevRegisters={prevRegistersRef.current}
          flags={flags}
        />
      </div>
    </>
  );
}



export default App;
