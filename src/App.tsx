import "./App.css";
import "./index.css";
import { Editor } from "@monaco-editor/react";

import { langConfiguration, langRules, langTheme } from "./langRules";
import { Navbar } from "./Components/Navbar";
import { RegistersTableView } from "./Components/RegisterView";
import { BottomBar } from "./Components/BottomBar";
import { useApp } from "./hooks/useApp";
import { useState } from "react";
import { MemoryBottombar } from "./Components/MemoryBar";
import { OutputDisplay } from "./Components/OutputDisplay";
import { SimpleButtonOnPort0x80 } from "./Components/ButtonOnPort";

export type BottomBarStates =
  | "Memory"
  | "Collapsed"
  | "Display"
  | "SimpleButtonOnPort0x80";

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
    setPort,
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
            bottomBarState={bottomBarState}
            setBottomBarState={setBottomBarState}
            stateToComponentMap={{
              Memory: (
                <MemoryBottombar
                  memAddrValueMap={memory}
                  prevMemAddrValueMap={prevMemoryRef.current}
                  memoryIndex={
                    registers.instruction_pointer +
                    registers.code_segment * 0x10
                  }
                />
              ),
              Display: <OutputDisplay field={wirteString} />,
              Collapsed: <></>,
              SimpleButtonOnPort0x80: (
                <SimpleButtonOnPort0x80
                  onClick={() => {
                    // check the value and flip it
                    if (registers.ports[0x80] === 0) {
                      setPort(0x80, [1]);
                    } else {
                      setPort(0x80, [0]);
                    }
                  }}
                  readPortValue={registers.ports[0x80]}
                />
              ),
            }}
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
