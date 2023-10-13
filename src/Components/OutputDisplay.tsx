export function OutputDisplay({
  className = "",
  field,
}: {
  className?: string;
  field: string;
}) {
  return (
    <div
      className={"bg-slate-800 text-slate-400 h-full w-full flex flex-col " + className}
    >
      {/* show a Output headding */}
      <div className="text-center text-lg font-semibold p-2">Output:</div>
      {/* show the output */}
      <div className="flex-1 p-2 overflow-y-auto">
        <div className="text-sm">{field}</div>
      </div>
    </div>
  );
}
