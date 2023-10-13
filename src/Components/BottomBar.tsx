import { BottomBarStates } from "../App";

// build the bottom bar component from a dict

export function BottomBar({
  stateToComponentMap,
  setBottomBarState,
  bottomBarState,
  className = "",
}: {
    stateToComponentMap: {
    [key in BottomBarStates]: JSX.Element;
  };
  setBottomBarState: (bottomBarState: BottomBarStates) => void;
  bottomBarState: BottomBarStates;
  className?: string;
}) {
  // if (bottomBarState == "Collapsed") {
  //   return <></>;
  // }

  const cancelButton = (
    <div className="absolute right-0 top-0">
      <button className="pr-2" onClick={() => setBottomBarState("Collapsed")}>
        X
      </button>
    </div>
  );

  const containerClass = `absolute w-full h-52 pointer-events-auto opacity-100
        left-0 bottom-8 border border-black/20 dark:border-white/20
        transition-all duration-500 ease-in-out bg-slate-800
        `;
    
  const bottomComponent = (
    <div className="h-full px-5 overflow-y-hidden">
      {
        stateToComponentMap[bottomBarState] ?? <></>
      }
    </div>
  );
  
  return (
    <div className={"absolute w-full " + className}>
      { (bottomBarState !== "Collapsed") && <div className={containerClass}>
        {cancelButton}
        {bottomComponent}
      </div>
      }
      {/* Navigation bar of the Bottom Bar */}
      <div className="w-full flex absolute bottom-0 bg-slate-800 pl-5 overflow-x-hidden">
        {
          Object.keys(stateToComponentMap).map((state) => {
            return (
              <button
                className="max-w-md text-xs p-2"
                onClick={() => {
                  setBottomBarState(
                    bottomBarState === state ? "Collapsed" : state as BottomBarStates
                  );
                }}
              >
                {bottomBarState == state ? "Hide" : "Show"} {state}
              </button>
            );
          })
        }
      </div>
    </div>
  );
}
