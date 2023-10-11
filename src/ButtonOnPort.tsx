export function ButtonOnPort({
  readPortValue,
  writeToPortFn,
}: {
  readPortValue: number;
  writeToPortFn: (value: number) => void;
}) {
  return (
    <button
      // usue tailwind to sytle the button and make sure the button is grey when off and red when on
      className={`text-white font-medium rounded-full text-sm px-5 py-2.5 text-center mr-2 mb-2 ${
        readPortValue == 0 ? "bg-gray-700" : "bg-red-700"
      } }`}
      onClick={() => {
        const value = readPortValue == 0 ? 1 : 0;
        writeToPortFn(value);
      }}
    >
      {readPortValue}
    </button>
  );
}
