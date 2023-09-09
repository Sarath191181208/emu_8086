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
import { langRules, langTheme } from "./langRules";

function App() {
  const [showMemoryBottomBar, setIsMemoryShown] = useState(true);
  const [memeory, setMemory, prevMemoryRef] = useStateSavePrevious(
    Array(0xffff).fill(0)
  );

  const [registers, setRegisters, prevRegistersRef] =
    useStateSavePrevious<CPUData>(getDefaultRegisters());
  const [flags, setFlags, prevFlagsRef] = useStateSavePrevious<Flags>(
    getDefaultFlags()
  );

  const editorRef = useRef<editor.IStandaloneCodeEditor>();
  const monacoRef = useRef<typeof import("monaco-editor")>();

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
      endColumn: err.column_number + err.length,
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

  const runPressed = async () => {
    try {
      const result: [CPUData, { mem: Array<number> }] = await invoke(
        "compile_code_and_run",
        {
          code: editorRef.current?.getValue(),
        }
      );
      const regs: any = result[0];
      setMemory(result[1].mem);
      setRegisters(extractCPUData(regs));
      clearErrorsOnEditor();
      setFlags(extractFlags(regs));
    } catch (e) {
      setErrorsOnEditor(e);
    }
  };

  const compileCode = async () => {
    try {
      const result: [CPUData, { mem: Array<number> }] = await invoke(
        "compile_code",
        {
          code: editorRef.current?.getValue(),
        }
      );
      const regs: any = result[0];
      setMemory(result[1].mem);
      setRegisters(extractCPUData(regs));
      clearErrorsOnEditor();
      setFlags(extractFlags(regs));
    } catch (e) {
      setErrorsOnEditor(e);
    }
  };

  const nextPressed = async () => {
    try {
      const result: [CPUData, { mem: Array<number> }] = await invoke("next", {
        code: editorRef.current?.getValue(),
      });
      const regs: any = result[0];
      setMemory(result[1].mem);
      setRegisters(extractCPUData(regs));
      clearErrorsOnEditor();
      setFlags(extractFlags(regs));
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
      <Navbar runPressed={runPressed} compileCode={compileCode} nextPressed={nextPressed} className="mb-5" />
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
              // monaco.editor.defineTheme("assembly-dark", langTheme);
            }}
            onChange={tryCompile}
            height="100%"
            defaultLanguage="assembly"
            theme="assembly-dark"
            options={{ minimap: { enabled: false } }}
            defaultValue={
              "MOV AX, BX \nMOV BX, CX\nlabel1: \nSUB CX, AX \n \nMOV AX, 0xAF34 \nMOV BL, 0X12 \nMOV AL, 10\nJMP label1"
            }
          />
          {/* create a toggle button that creates a white screen when pressed that's on top of editor */}
          <MemoryBottomBar
            key="memory-bottom-bar"
            prevArr={prevMemoryRef.current}
            arr={memeory}
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
  runPressed,
  compileCode,
  nextPressed,
}: {
  className?: string;
  runPressed: () => void;
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
          onClick={runPressed}
          className="p-2 hover:bg-white/5 transition ease-in-out "
        >
          Run
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
  arr,
  prevArr,
  showMemoryBottomBar,
  setIsMemoryShown,
  className = "",
}: {
  arr: number[];
  prevArr: number[];
  showMemoryBottomBar: boolean;
  setIsMemoryShown: (showMemoryBottomBar: boolean) => void;
  className?: string;
}) {
  const [start, setStart] = useState(0x0100);
  const [indicesToAnimate, _updateIndicesToAnimate] = useState<number[]>([]);
  const updateIndicesToAnimate = (newVal: number[]) => {
    console.log(newVal);
    _updateIndicesToAnimate(newVal);
  };

  console.log(arr);

  useEffect(() => {
    const notEqIdxArr = arr
      .map((num, i) => (num !== prevArr[i] ? i : -1))
      .filter((num) => num !== -1);
    updateIndicesToAnimate(notEqIdxArr);

    let timeoutId = setTimeout(() => {
      // this is to remove the animation class so that it can be added again
      updateIndicesToAnimate([]);
    }, 3000);
    return () => clearTimeout(timeoutId);
  }, [arr]);

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
            {/* create a number edit field and an increase and dcrease btns */}
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
                value={`0x${start.toString(16).toUpperCase().padStart(4, "0")}`}
                readOnly
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
            >
              {arr.slice(start, start + 16 * 6).map((val, i) => (
                // for every 16 elements create a label
                <>
                  {i % 16 === 0 && (
                    <div
                      key={`label${i}`}
                      className="text-slate-400-dark:text-slate-200 text-center p-1"
                    >
                      {`0x${(start + i)
                        .toString(16)
                        .toUpperCase()
                        .padStart(4, "0")}`}
                    </div>
                  )}
                  <div
                    // className={`border border-black/10 dark:border-white/10 rounded-md flex items-center justify-center`}
                    key={i}
                    className={
                      "text-slate-400 dark:text-slate-200 text-center p-1 " +
                      (indicesToAnimate.includes(start + i)
                        ? `animate-val-change`
                        : "")
                    }
                  >
                    {val.toString(16).toUpperCase().padStart(2, "0")}
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
    }, 3000);
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
    }, 3000);
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
                  className={`bg-slate-800 py-2 text-center ${
                    value ? "bg-green-300/40" : ""
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
