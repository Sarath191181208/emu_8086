import { useEffect, useRef, useState } from "react";
import "./App.css";
import "./index.css";
import { Editor } from "@monaco-editor/react";
import { getHighHex, getLowHex } from "./utils";
import { editor } from "monaco-editor";
import { invoke } from "@tauri-apps/api/tauri";
import { useStateSavePrevious } from "./hooks/useStateSavePrevious";
import {
  CPUData,
  Flags,
  FlagsShort,
  Registers16BitNotGeneralShort,
} from "./types/CPUData/CPUData";
import {
  extractCPUData,
  extractFlags,
} from "./types/CPUData/extract";
import {
  getDefaultRegisters,
  getDefaultFlags,
} from "./types/CPUData/getDefaultRegistersAndFlags";
import {
  completionRules,
  langConfiguration,
  langRules,
  langTheme,
} from "./langRules";
import { Navbar } from "./Components/Navbar";
import { RegistersTableView } from "./Components/RegisterView";
import { MemoryBottomBar } from "./Components/MemoryBottombar";

function App() {
  const [showMemoryBottomBar, setIsMemoryShown] = useState(true);
  const [memory, setMemory, prevMemoryRef] = useStateSavePrevious<
    Map<ArrayIndex, Byte>
  >(new Map<ArrayIndex, Byte>());

  const [registers, setRegisters, prevRegistersRef] =
    useStateSavePrevious<CPUData>(getDefaultRegisters());
  const compiledBytesRef = useRef<Array<CompiledBytes>>();
  const [flags, setFlags, _] = useStateSavePrevious<Flags>(
    getDefaultFlags()
  );

  const editorRef = useRef<editor.IStandaloneCodeEditor>();
  const monacoRef = useRef<typeof import("monaco-editor")>();
  const debugDecorationRef = useRef<editor.IEditorDecorationsCollection>();
  const decorateStepNext = (
    editor: editor.IStandaloneCodeEditor,
    decorations: editor.IModelDeltaDecoration[]
  ) => {
    // clear previous decorations
    if (debugDecorationRef.current) {
      debugDecorationRef.current.clear();
    }
    // add new decorations
    let decCollections = editor.createDecorationsCollection(decorations);
    debugDecorationRef.current = decCollections;
  };

  const highlightLine = (lineNumber: number) => {
    const monaco = monacoRef.current;
    const editor = editorRef.current;
    if (!monaco || !editor) {
      return;
    }
    const model = editor.getModel();
    if (!model) {
      return;
    }
    const lineCount = model.getLineCount();
    if (lineNumber >= lineCount) {
      return;
    }

    // editor.setSelection({
    //   startLineNumber: lineNumber + 1,
    //   startColumn: 1,
    //   endLineNumber: lineNumber + 1,
    //   endColumn: 1000,
    // });
    // color the whole line with black color
    const decoration = {
      range: {
        startLineNumber: lineNumber + 1,
        startColumn: 1,
        endLineNumber: lineNumber + 1,
        endColumn: 1,
      },
      options: {
        isWholeLine: true,
        className: "bg-clr",
      },
    };
    decorateStepNext(editor, [decoration]);
  };

  useEffect(() => {
    // caluculate the line from the line ref
    const ins_pointer_offset = registers.instruction_pointer - 0x100;
    const compiledBytes = compiledBytesRef.current;
    if (!compiledBytes) {
      return;
    }
    // sort compiledbytes by line number
    const sortedCompiledBytes = compiledBytes.sort(
      (a, b) => a.line_number - b.line_number
    );
    // accumulate the number of bytes and find the line number
    let line = -1;
    let acc = 0;
    for (let i = 0; i < sortedCompiledBytes.length; i++) {
      const compiledByte = sortedCompiledBytes[i];
      acc += compiledByte.bytes.length;
      if (acc > ins_pointer_offset) {
        line = compiledByte.line_number;
        break;
      }
    }
    if (line === -1) {
      return;
    }
    highlightLine(line);
  }, [registers.instruction_pointer]);

  const setErrorsOnEditor = (e: any) => {
    const errorList = e as CompilationError[];
    const monaco = monacoRef.current;
    const editor = editorRef.current;
    const model = editor?.getModel();
    if (!monaco || !editor || !model) {
      return;
    }

    // the message has | text Error("str") text | in it some we should extract the str from Error()
    const getErrorMessage = (message: string) => {
      return message.replace(/Error\(([^)]+)\)/g, "$1");
    };

    const markers = errorList.map((err) => ({
      startLineNumber: err.line_number + 1,
      startColumn: err.column_number + 1,
      endLineNumber: err.line_number + 1,
      endColumn: err.column_number + err.length + 1,
      message: getErrorMessage(err.message),
      severity: monaco.MarkerSeverity.Error,
    }));
    monaco.editor.setModelMarkers(model, "owner", markers);
  };

  const clearErrorsOnEditor = () => {
    const monaco = monacoRef.current;
    const editor = editorRef.current;
    const model = editor?.getModel();
    if (!monaco || !editor || !model) {
      return;
    }
    monaco.editor.setModelMarkers(model, "owner", []);
  };

  const compileCode = async () => {
    try {
      const result: [CPUData, CompiledBytes[], Array<[number, number]>] =
        await invoke("compile_code", {
          code: editorRef.current?.getValue(),
        });
      const regs: any = result[0];
      compiledBytesRef.current = result[1];
      const memoryChanges = result[2];

      let memClone = new Map<ArrayIndex, Byte>(memory);
      // update memory
      for (let i = 0; i < memoryChanges.length; i++) {
        const [index, value] = memoryChanges[i];
        memClone.set(index, value);
      }
      setMemory(memClone);

      setRegisters(extractCPUData(regs));
      clearErrorsOnEditor();
      setFlags(extractFlags(regs));
    } catch (e) {
      setErrorsOnEditor(e);
    }
  };

  const nextPressed = async () => {
    try {
      const result: [CPUData, Array<[number, number]>] = await invoke("next", {
        code: editorRef.current?.getValue(),
      });
      const regs: any = result[0];
      const cpu = extractCPUData(regs);
      const memoryChanges = result[1];

      let memClone = new Map<ArrayIndex, Byte>(memory);
      // update memory
      for (let i = 0; i < memoryChanges.length; i++) {
        const [index, value] = memoryChanges[i];
        memClone.set(index, value);
      }
      setMemory(memClone);

      setRegisters(cpu);
      setFlags(extractFlags(regs));
      clearErrorsOnEditor();
    } catch (e) {
      // setErrorsOnEditor(e);
      // TODO: handle error
    }
  };

  const tryCompile = async () => {
    try {
      await invoke("try_compile_code", { code: editorRef.current?.getValue() });
      clearErrorsOnEditor();
    } catch (e) {
      setErrorsOnEditor(e);
    }
  };

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
                completionRules
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
