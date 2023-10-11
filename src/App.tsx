import "./App.css";
import "./index.css";
import { Editor } from "@monaco-editor/react";

import { langConfiguration, langRules, langTheme } from "./langRules";
import { Navbar } from "./Components/Navbar";
import { RegistersTableView } from "./Components/RegisterView";
import { BottomBar } from "./Components/MemoryBottombar";
import { useApp } from "./hooks/useApp";
import { useState } from "react";
import { ButtonOnPort } from "./ButtonOnPort";

export type BottomBarStates = "Memory" | "Collapsed" | "Display";

function App() {
  const [bottomBarState, setBottomBarState] =
    useState<BottomBarStates>("Memory");

  const {
    registers,
    prevRegistersRef,
    flags,
    memory,
    prevMemoryRef,
    editorRef,
    monacoRef,

    wirteString,

    languageCompletionProvider,
    langDefinitionProvider,

    compileCode,
    nextPressed,
    tryCompile,
  } = useApp();

  return (
    <>
      <Navbar
        compileCode={compileCode}
        nextPressed={nextPressed}
        className="mb-5"
      />
      <ButtonOnPort
        readPortValue={registers.ports[0x80]}
        writeToPortFn={function (_: number): void {
          throw new Error("Function not implemented.");
        }}
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

              monaco.languages.registerDefinitionProvider(
                "assembly",
                langDefinitionProvider
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
          <BottomBar
            key="memory-bottom-bar"
            memoryIndex={
              registers.instruction_pointer + registers.code_segment * 0x10
            }
            writeString={wirteString}
            prevMemAddrValueMap={prevMemoryRef.current}
            memAddrValueMap={memory}
            bottomBarState={bottomBarState}
            setBottomBarState={setBottomBarState}
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
