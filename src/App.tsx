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
  extractFlagsShort,
  extractNonGeneral16bitRegisters,
} from "./types/CPUData/extract";
import {
  getDefaultRegisters,
  getDefaultFlags,
} from "./types/CPUData/getDefaultRegistersAndFlags";
import { langConfiguration, langRules, langTheme } from "./langRules";

type ArrayIndex = number;
type Byte = number;

function App() {
  

  const [showMemoryBottomBar, setIsMemoryShown] = useState(true);
  const [memory, setMemory, prevMemoryRef] = useStateSavePrevious<Map<ArrayIndex, Byte>>(new Map<ArrayIndex, Byte>());

  const [registers, setRegisters, prevRegistersRef] =
    useStateSavePrevious<CPUData>(getDefaultRegisters());
  const compiledBytesRef = useRef<Array<CompiledBytes>>();
  const [flags, setFlags, prevFlagsRef] = useStateSavePrevious<Flags>(
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
        <div className="col-span-1 pr-5">
          <div className="grid-cols-1 gap-4">
            {/* create a grid area */}
            <div className=" flex flex-col ">
              <Table
                key={"reg-table"}
                registers={[
                  registers.ax,
                  registers.bx,
                  registers.cx,
                  registers.dx,
                ]}
                prevRegisters={[
                  prevRegistersRef.current.ax,
                  prevRegistersRef.current.bx,
                  prevRegistersRef.current.cx,
                  prevRegistersRef.current.dx,
                ]}
              />
              <div className="w-min mt-5 flex gap-5">
                <Table16bitRegs
                  nonGeneral16BitRegister={extractNonGeneral16bitRegisters(
                    registers
                  )}
                  prevNonGeneral16BitRegister={extractNonGeneral16bitRegisters(
                    prevRegistersRef.current
                  )}
                  className="w-min"
                />
                <ShowFlags flags={extractFlagsShort(flags)} className="w-min" />
              </div>
            </div>
            <div></div>
          </div>
        </div>
      </div>
    </>
  );
}

function Navbar({
  className = "",
  compileCode,
  nextPressed,
}: {
  className?: string;
  compileCode: () => void;
  nextPressed: () => void;
}) {
  // create a navbar with open file and save file run next and previous buttons
  return (
    <nav className={" " + className}>
      <div className="bg-slate-800 dark:bg-slate-950 flex gap-2">
        <button className="p-2 hover:bg-white/5 transition ease-in-out ">
          Open
        </button>
        <button className="p-2 hover:bg-white/5 transition ease-in-out ">
          Save
        </button>
        <button
          onClick={compileCode}
          className="p-2 hover:bg-white/5 transition ease-in-out "
        >
          Compile
        </button>
        <button
          onClick={nextPressed}
          className="p-2 hover:bg-white/5 transition ease-in-out "
        >
          Next
        </button>
        <button className="p-2 hover:bg-white/5 transition ease-in-out ">
          Previous
        </button>
      </div>
    </nav>
  );
}
function MemoryBottomBar({
  memAddrValueMap,
  prevMemAddrValueMap,
  showMemoryBottomBar,
  setIsMemoryShown,
  className = "",
}: {
  memAddrValueMap: Map<ArrayIndex, Byte>;
  prevMemAddrValueMap: Map<ArrayIndex, Byte>;
  showMemoryBottomBar: boolean;
  setIsMemoryShown: (showMemoryBottomBar: boolean) => void;
  className?: string;
}) {
  const [start, setStart] = useState(0x1000);
  const [inputStr, setInputStr] = useState(
    "0x" + start.toString(16).toUpperCase().padStart(4, "0")
  );
  const [indicesToAnimate, _updateIndicesToAnimate] = useState<number[]>([]);
  const updateIndicesToAnimate = (newVal: number[]) => {
    _updateIndicesToAnimate(newVal);
  };

  const getValOrZero = (index: number): number => {
    const val = memAddrValueMap.get(index);
    if (val === undefined) {
      return 0;
    }
    return val;
  }

  const handleMemoryViewAddressChange = (e: any) => {
    let val: string = e.target.value;
    // if the value is greater than 0xffff then set it to 0xffff
    if (parseInt(val, 16) > 0xffFff) {
      val = "0xfffff";
    }

    // pad it to 4 digits
    // val = val.toUpperCase().padStart(4, "0");

    // put the 0x if doesn't exits
    if (!val.toLowerCase().startsWith("0x")) {
      val = "0x" + val;
    }

    // set the value
    setInputStr(val);
  };

  useEffect(() => {
    // find the indices that are not equal
    const notEqIdxArr: number[] = [];
    memAddrValueMap.forEach((val, idx) => {
      if (val !== prevMemAddrValueMap.get(idx)) {
        notEqIdxArr.push(idx);
      }
    });


    updateIndicesToAnimate(notEqIdxArr);

    let timeoutId = setTimeout(() => {
      // this is to remove the animation class so that it can be added again
      updateIndicesToAnimate([]);
    }, 400);
    return () => clearTimeout(timeoutId);
  }, [memAddrValueMap]);

  return (
    <div className={"absolute w-full " + className}>
      {showMemoryBottomBar && (
        <div
          className={`absolute w-full h-50 pointer-events-auto opacity-100
        left-0 bottom-8 border border-black/20 dark:border-white/20
        transition-all duration-500 ease-in-out bg-slate-800
        `}
        >
          <div className="absolute right-0 top-0">
            <button
              className="pr-2"
              onClick={() => setIsMemoryShown(!showMemoryBottomBar)}
            >
              X
            </button>
          </div>
          <div className="h-full px-5">
            <div className="">
              <button
                className="p-2 hover:bg-white/5 transition ease-in-out "
                onClick={() => {
                  if (start - 0x10 >= 0) {
                    // update start
                    setStart(start - 0x10);
                  }
                }}
              >
                -
              </button>
              <input
                className="bg-slate-800 text-slate-400 dark:text-slate-200 w-20 text-center"
                type="text"
                value={inputStr}
                onChange={handleMemoryViewAddressChange}
                onKeyDown={(e) => {
                  if (e.key === "Enter" && inputStr !== "") {
                    let newInputStr = inputStr.toUpperCase().padStart(4, "0");
                    if (!newInputStr.toLocaleLowerCase().startsWith("0x")) {
                      newInputStr = "0x" + newInputStr;
                    }
                    const newStart = parseInt(newInputStr, 16);
                    setStart(newStart);
                  }
                }}
                // readOnly
              />
              <button
                className="p-2 hover:bg-white/5 transition ease-in-out "
                onClick={() => {
                  if (start + 0x10 <= 0xffff) {
                    // update start
                    setStart(start + 0x10);
                  }
                }}
              >
                +
              </button>
            </div>
            <div
              className={`grid h-full gap-x-3 gap-y-2 gridCols17 gridRows6 text-xs items-center justify-items-center`}
              key={`memory-view-${start}`}
            >
              {Array(16 * 6)
                .fill(0)
                .map((_, i) => (
                  // for every 16 elements create a label
                  <>
                    {i % 16 === 0 && (
                      <div
                        key={`label${start}-${i}`}
                        className="text-slate-400 dark:text-slate-200 text-center font-semibold"
                      >
                        {`0x${(start + i)
                          .toString(16)
                          .toUpperCase()
                          .padStart(4, "0")}`}
                      </div>
                    )}
                    <div
                      // className={`border border-black/10 dark:border-white/10 rounded-md flex items-center justify-center`}
                      key={`${start}-${i}`}
                      className={
                        "text-slate-400 dark:text-slate-200 text-center p-1 " +
                        (indicesToAnimate.includes(start + i)
                          ? `animate-val-change`
                          : "")
                      }
                    >
                      {getValOrZero(start+i).toString(16).toUpperCase().padStart(2, "0")}
                    </div>
                  </>
                ))}
            </div>
          </div>
        </div>
      )}
      <div className="w-full flex absolute bottom-0 bg-slate-800 pl-5 overflow-x-hidden">
        <button
          className="max-w-md text-xs p-2"
          onClick={() => setIsMemoryShown(!showMemoryBottomBar)}
        >
          {showMemoryBottomBar ? "Hide" : "Show"} Memory
        </button>
      </div>
    </div>
  );
}

function Table({
  registers,
  prevRegisters,
  className = "",
}: {
  registers: [number, number, number, number];
  prevRegisters: [number, number, number, number];
  className?: string;
}) {
  const keys = ["ax", "bx", "cx", "dx"];
  const changedValIdxs: boolean[][] = registers.map((val, i) => {
    let highChange = false;
    let lowChange = false;

    if (getHighHex(val) !== getHighHex(prevRegisters[i])) {
      highChange = true;
    }
    if (getLowHex(val) !== getLowHex(prevRegisters[i])) {
      lowChange = true;
    }

    return [highChange, lowChange];
  });

  const [animateKeys, setAnimateKeys] = useState<boolean[][]>([]);

  useEffect(() => {
    setAnimateKeys(changedValIdxs);
    let timeoutId = setTimeout(() => {
      // this is to remove the animation class so that it can be added again
      setAnimateKeys([]);
    }, 400);
    return () => clearTimeout(timeoutId);
  }, [registers]);

  const getShouldAnimateLow = (i: number): boolean => {
    // check if animateKeys is empty
    if (animateKeys.length === 0) {
      return false;
    }
    // check if animateKeys[i] is empty
    if (animateKeys[i].length === 0) {
      return false;
    }
    return animateKeys[i][1];
  };

  const getShouldAnimateHigh = (i: number): boolean => {
    // check if animateKeys is empty
    if (animateKeys.length === 0) {
      return false;
    }
    // check if animateKeys[i] is empty
    if (animateKeys[i].length === 0) {
      return false;
    }
    return animateKeys[i][0];
  };

  return (
    <div
      className={
        "not-prose bg-slate-50 rounded-xl  dark:bg-slate-800/25 max-w-min " +
        className
      }
    >
      <div className="rounded-xl overflow-auto">
        <div className="shadow-sm overflow-hidden mt-2">
          <table className="border-collapse table-auto w-full text-sm">
            <thead>
              <tr>
                <th className="border-b dark:border-slate-600 font-medium px-6 py-2 pt-0 pb-3 text-slate-400 dark:text-slate-200 text-left">
                  Reg
                </th>
                <th className="border-b dark:border-slate-600 font-medium text-center px-6 py-2 pt-0 pb-3 text-slate-400 dark:text-slate-200">
                  H
                </th>
                <th className="border-b dark:border-slate-600 font-medium text-center px-6 py-2 pt-0 pb-3 text-slate-400 dark:text-slate-200">
                  L
                </th>
              </tr>
            </thead>
            <tbody className="bg-slate-800">
              {keys.map((regName, i) => (
                <tr key={regName}>
                  <td className="px-6 py-2 text-slate-400 dark:text-slate-200 text-left">
                    {regName.toUpperCase()}
                  </td>
                  {/* show the text in td but show the values in hex */}
                  <td
                    className={
                      "px-6 py-2 text-slate-400 dark:text-slate-200 text-center " +
                      (getShouldAnimateHigh(i) ? "animate-val-change" : "")
                    }
                  >
                    {getHighHex(registers[i])}
                  </td>
                  <td
                    className={
                      "px-6 py-2 text-slate-400 dark:text-slate-200 text-center " +
                      (getShouldAnimateLow(i) ? "animate-val-change" : "")
                    }
                  >
                    {getLowHex(registers[i])}
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </div>
      <div className="absolute inset-0 pointer-events-none border border-black/5 rounded-xl dark:border-white/5"></div>
    </div>
  );
}

function Table16bitRegs({
  nonGeneral16BitRegister,
  prevNonGeneral16BitRegister,
  className = "",
}: {
  nonGeneral16BitRegister: Registers16BitNotGeneralShort;
  prevNonGeneral16BitRegister: Registers16BitNotGeneralShort;
  className?: string;
}) {
  const changedValKeys = Object.entries(nonGeneral16BitRegister ?? {})
    .map(([key, val]) => {
      if (val !== prevNonGeneral16BitRegister[key]) {
        return key;
      }
      return null;
    })
    .filter((key) => key !== null) as string[];

  const [animateKeys, setAnimateKeys] = useState<string[]>([]);

  useEffect(() => {
    setAnimateKeys(changedValKeys);
    let timeoutId = setTimeout(() => {
      // this is to remove the animation class so that it can be added again
      setAnimateKeys([]);
    }, 400);
    return () => clearTimeout(timeoutId);
  }, [nonGeneral16BitRegister]);

  return (
    <div
      className={
        "not-prose bg-slate-50 rounded-xl  dark:bg-slate-800/25  " + className
      }
    >
      <div className="rounded-xl overflow-auto">
        <div className="shadow-sm overflow-hidden mt-2">
          <table className="border-collapse table-auto w-full text-sm">
            <thead>
              <tr>
                <th className="border-b dark:border-slate-600 font-medium px-4 py-2 pt-0 pb-3 text-slate-400 dark:text-slate-200 text-left">
                  Reg
                </th>
                <th className="border-b dark:border-slate-600 font-medium text-center px-4 py-2 pt-0 pb-3 text-slate-400 dark:text-slate-200">
                  Value
                </th>
              </tr>
            </thead>
            <tbody className="bg-slate-800">
              {Object.entries(nonGeneral16BitRegister ?? {}).map(
                ([regName, value]) => (
                  <tr key={regName}>
                    <td className="px-4 py-2 text-slate-400 dark:text-slate-200 text-left">
                      {regName.toUpperCase()}
                    </td>
                    {/* show the text in td but show the values in hex */}
                    <td
                      className={
                        "px-4 py-2 text-slate-400 dark:text-slate-200 text-center " +
                        (animateKeys.includes(regName)
                          ? "animate-val-change"
                          : "")
                      }
                    >
                      {value.toString(16).toUpperCase().padStart(4, "0")}
                    </td>
                  </tr>
                )
              )}
            </tbody>
          </table>
        </div>
      </div>
      <div className="absolute inset-0 pointer-events-none border border-black/5 rounded-xl dark:border-white/5"></div>
    </div>
  );
}

function ShowFlags({
  flags,
  className = "",
}: {
  flags: FlagsShort;
  className?: string;
}) {
  return (
    <div
      className={"bg-slate-50 rounded-xl  dark:bg-slate-800/25  " + className}
    >
      <div className="rounded-xl overflow-auto">
        <div className="shadow-sm overflow-hidden mt-2">
          <div className="w-full">
            <div className="border-b dark:border-slate-600 font-medium px-4 py-2 pt-0 pb-3 text-slate-400 dark:text-slate-200 text-left">
              Flags
            </div>
            <div className="">
              {Object.entries(flags).map(([flagName, value]) => (
                <div
                  key={flagName}
                  className={`py-2 text-center ${
                    value ? " bg-green-300/30" : "bg-slate-800"
                  }`}
                >
                  {" "}
                  {flagName}{" "}
                </div>
              ))}
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}

export default App;
