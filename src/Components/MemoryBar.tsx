import { useEffect, useState } from "react";

export function MemoryBottombar({
  memAddrValueMap,
  prevMemAddrValueMap,
  memoryIndex,
  className,
}: {
  memAddrValueMap: Map<ArrayIndex, Byte>;
  prevMemAddrValueMap: Map<ArrayIndex, Byte>;
  memoryIndex: ArrayIndex;
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
  };

  const handleMemoryViewAddressChange = (e: any) => {
    let val: string = e.target.value;
    // if the value is greater than 0xffff then set it to 0xffff
    if (parseInt(val, 16) > 0xfffff) {
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

  ("bg-[#fddf47]");

  return (
    <>
      <div className={className}>
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
                    ? `animate-val-change `
                    : " ") +
                  (start + i === memoryIndex
                    ? "bg-[#fddf47] bg-opacity-20 rounded-sm"
                    : " ")
                }
              >
                {getValOrZero(start + i)
                  .toString(16)
                  .toUpperCase()
                  .padStart(2, "0")}
              </div>
            </>
          ))}
      </div>
    </>
  );
}
