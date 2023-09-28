import { useState, useEffect } from "react";
import {
  CPUData,
  Flags,
  FlagsShort,
  Registers16BitNotGeneralShort,
} from "../types/CPUData/CPUData";
import {
  extractFlagsShort,
  extractNonGeneral16bitRegisters,
} from "../types/CPUData/extract";
import { getHighHex, getLowHex } from "../utils";

export function RegistersTableView({
  registers,
  prevRegisters,
  flags,
}: {
  registers: CPUData;
  prevRegisters: CPUData;
  flags: Flags;
}) {
  return (
    <div className="col-span-1 pr-5">
      <div className="grid-cols-1 gap-4">
        {/* create a grid area */}
        <div className=" flex flex-col ">
          <Table
            key={"reg-table"}
            registers={[registers.ax, registers.bx, registers.cx, registers.dx]}
            prevRegisters={[
              prevRegisters.ax,
              prevRegisters.bx,
              prevRegisters.cx,
              prevRegisters.dx,
            ]}
          />
          <div className="w-min mt-5 flex gap-5">
            <Table16bitRegs
              nonGeneral16BitRegister={extractNonGeneral16bitRegisters(
                registers
              )}
              prevNonGeneral16BitRegister={extractNonGeneral16bitRegisters(
                prevRegisters
              )}
              className="w-min"
            />
            <ShowFlags flags={extractFlagsShort(flags)} className="w-min" />
          </div>
        </div>
        <div></div>
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
