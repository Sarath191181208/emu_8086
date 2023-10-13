import { ButtonOnPort } from "../ButtonOnPort";

export function SimpleButtonOnPort0x80({
  className = "",
  readPortValue,
  onClick,
}: {
  onClick: () => void;
  readPortValue: number;
  className?: string;
}) {
    return <div className={className}>

        <ButtonOnPort readPortValue={readPortValue} writeToPortFn={onClick} />
    </div>
}
