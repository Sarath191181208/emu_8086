import { invoke } from "@tauri-apps/api";
import { editor } from "monaco-editor";
import { useState, useRef, useEffect } from "react";
import { CPUData, Flags } from "../types/CPUData/CPUData";
import { extractCPUData, extractFlags } from "../types/CPUData/extract";
import {
  getDefaultRegisters,
  getDefaultFlags,
} from "../types/CPUData/getDefaultRegistersAndFlags";
import { useStateSavePrevious } from "./useStateSavePrevious";
import { languages } from "monaco-editor/esm/vs/editor/editor.api";
import {
  CompilationError,
  Suggestion,
  compilationErrorToSuggestions,
  suggestionToCompletionProvider,
} from "../types/compilationError";

export function useApp() {
  const [showMemoryBottomBar, setIsMemoryShown] = useState(true);
  const [memory, setMemory, prevMemoryRef] = useStateSavePrevious<
    Map<ArrayIndex, Byte>
  >(new Map<ArrayIndex, Byte>());

  const [registers, setRegisters, prevRegistersRef] =
    useStateSavePrevious<CPUData>(getDefaultRegisters());
  const compiledBytesRef = useRef<Array<CompiledBytes>>();
  const [flags, setFlags, _] = useStateSavePrevious<Flags>(getDefaultFlags());

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
      await invoke("try_compile_code", {
        code: editorRef.current?.getValue(),
      });
      clearErrorsOnEditor();
    } catch (e) {
      setErrorsOnEditor(e);
      // editorRef.current?.trigger("some", "editor.action.triggerSuggest", {});
    }
  };

  let isReady = false;
  let suggestionsArray = useRef<Suggestion<string>[]>([]);
  let itrRef = 0;
  const getSuggestions = async (lineNumber: number, _: number) => {
    try {
      await invoke("try_compile_code", {
        code: editorRef.current?.getValue(),
      });
      clearErrorsOnEditor();
    } catch (e) {
      if (e === null || e === undefined || isReady === true) {
        return;
      }
      let errors = e as CompilationError[];
      let tempSuggestions = compilationErrorToSuggestions(
        errors,
        lineNumber,
        _
      );
      if (tempSuggestions === null || tempSuggestions.length === 0) {
        return;
      }
      itrRef++;
      suggestionsArray.current = tempSuggestions;
        (isReady = true);
      console.log(suggestionsArray);
      setTimeout(() => {
        console.log("trigger");
        editorRef.current?.trigger("some", "editor.action.triggerSuggest", {});
      }, 100);
    }
  };
  const languageCompletionProvider: languages.CompletionItemProvider = {
    triggerCharacters: [".", " ", ","],
    provideCompletionItems(model, position) {
      let word = model.getWordUntilPosition(position);
      // languages.CompletionList;

      let range = {
        startLineNumber: position.lineNumber,
        endLineNumber: position.lineNumber,
        startColumn: word.startColumn,
        endColumn: word.endColumn,
      };

      return getSuggestions(position.lineNumber, position.column).then(() => {
        if (isReady) {
          let tempSuggestions = suggestionsArray.current;
          suggestionsArray.current = [];
          isReady = false;
          return {
            suggestions: tempSuggestions.map((s) => ({
              label: s.value,
              kind: suggestionToCompletionProvider[s.type],
              insertText: s.value,
              documentation: s.value,
              range,
            })),
          };
        }
        return {
          suggestions: [],
        };
      });
    },
  };

  return {
    registers,
    flags,

    memory,
    showMemoryBottomBar,

    prevRegistersRef,
    prevMemoryRef,
    editorRef,
    monacoRef,

    languageCompletionProvider,

    compileCode,
    nextPressed,
    setIsMemoryShown,
    tryCompile,
  };
}

// function getCompiletion(
//   range: {
//     startLineNumber: number;
//     endLineNumber: number;
//     startColumn: number;
//     endColumn: number;
//   },
//   kind: languages.CompletionItemKind
// ) {
//   return {
//     label: "test",
//     kind,
//     insertText: "test",
//     documentation: "test",
//     range: {
//       startLineNumber: 1,
//       endLineNumber: 1,
//       startColumn: 1,
//       endColumn: 1,
//     },
//   };
// }
