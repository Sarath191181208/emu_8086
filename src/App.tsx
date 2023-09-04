import { useState } from "react";
import "./App.css";
import "./index.css";
import { Editor } from "@monaco-editor/react";
import { getHighHex, getLowHex } from "./utils";

function App() {
  let [showMemoryBottomBar, setIsMemoryShown] = useState(true);

  return (
    // create a grid view and the editor takes 2 spaces while the other things take 1 space
    <div className="flex gap-4">
      <div className="relative col-span-4 w-full">
        <Editor
          height="100%"
          defaultLanguage="assembly"
          theme="vs-dark"
          options={{ minimap: { enabled: false } }}
          defaultValue="// some comment"
        />
        {/* create a toggle button that creates a white screen when pressed that's on top of editor */}
        <MemoryBottomBar
          showMemoryBottomBar={showMemoryBottomBar}
          setIsMemoryShown={setIsMemoryShown}
        />
      </div>
      <div className="col-span-1 pr-5">
        <div className="grid-cols-1 gap-4">
          {/* create a grid area */}
          <div className=" flex flex-col ">
            <Table />
            <div className="w-min mt-5 flex gap-5">
              <Table16bitRegs className="w-min" />
              <ShowFlags className="w-min" />
            </div>
          </div>
          <div></div>
        </div>
      </div>
    </div>
  );
}

function MemoryBottomBar({
  showMemoryBottomBar,
  setIsMemoryShown,
  className = "",
}: {
  showMemoryBottomBar: boolean;
  setIsMemoryShown: (showMemoryBottomBar: boolean) => void;
  className?: string;
}) {
  const arr = new Array(17 * 6).fill(0);

  return (
    <div className={"absolute w-full" + className}>
      {showMemoryBottomBar && (
        <div
          className={`absolute w-full h-52 pointer-events-auto opacity-100
        left-0 bottom-8 border border-black/5 dark:border-white/5
        transition-all duration-500 ease-in-out
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
          <div className="h-full flex">
            <div
              className={`grid h-full gap-2 p-5 gridCols17 gridRows6 text-xs`}
            >
              {arr.map((_, i) => (
                <div
                  key={i}
                  className="text-slate-400 dark:text-slate-200 text-center"
                >
                  {i.toString(16).toUpperCase().padStart(2, "0")}
                </div>
              ))}
            </div>
          </div>
        </div>
      )}
      <div className="w-full flex absolute bottom-0 bg-black/10 pl-5 overflow-x-hidden">
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

function Table({ className = "" }) {
  const two_col_regs = ["AX", "BX", "CX", "DX"];
  const values = [12, 18, 20, 0x1571];

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
              {two_col_regs.map((reg, i) => (
                <tr key={reg}>
                  <td className="px-6 py-2 text-slate-400 dark:text-slate-200 text-left">
                    {reg}
                  </td>
                  {/* show the text in td but show the values in hex */}
                  <td className="px-6 py-2 text-slate-400 dark:text-slate-200 text-center">
                    {getHighHex(values[i])}
                  </td>
                  <td className="px-6 py-2 text-slate-400 dark:text-slate-200 text-center">
                    {getLowHex(values[i])}
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

function Table16bitRegs({ className = "" }) {
  const regs = ["SP", "BP", "SI", "DI", "IP", "SS", "CS", "DS", "ES"];
  const values = [12, 18, 20, 0x1571, 0x1571, 0x1571, 0x1571, 0x1571, 0x1571];

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
              {regs.map((reg, i) => (
                <tr key={reg}>
                  <td className="px-4 py-2 text-slate-400 dark:text-slate-200 text-left">
                    {reg}
                  </td>
                  {/* show the text in td but show the values in hex */}
                  <td
                    colSpan={2}
                    className="px-4 py-2 text-slate-400 dark:text-slate-200 text-center"
                  >
                    {"0x" +
                      values[i].toString(16).toUpperCase().padStart(4, "0")}
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

function ShowFlags({ className = "" }) {
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
              <div className="bg-slate-800 py-2 text-center"> CF </div>
              <div className="bg-slate-800 py-2 text-center"> PF </div>
              <div className="bg-slate-800 py-2 text-center"> AF </div>
              <div className="bg-slate-800 py-2 text-center"> ZF </div>
              <div className="bg-slate-800 py-2 text-center"> SF </div>
              <div className="bg-slate-800 py-2 text-center"> TF </div>
              <div className="bg-slate-800 py-2 text-center"> IF </div>
              <div className="bg-slate-800 py-2 text-center"> DF </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}

export default App;
