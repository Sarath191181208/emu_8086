import { useRef, useState } from "react";

export function useStateSavePrevious<T>(initialValue: T) {
    const [value, setValueAlt] = useState<T>(initialValue);
    const valueRef = useRef<T>(initialValue);
    const setValue = (newValue: T) => {
        valueRef.current = value;
        setValueAlt(newValue);
    };
    return [value, setValue, valueRef] as const;
}
