export function Navbar({
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
