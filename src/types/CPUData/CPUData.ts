interface GeneralPurposeRegisters {
  [key: string]: number;
  ax: 0;
  bx: 0;
  cx: 0;
  dx: 0;
}

interface PointerRegisters {
  [key: string]: number;
  base_pointer: number;
  destination_index: number;
  instruction_pointer: number;
  source_index: number;
  stack_pointer: number;
}

interface SegmentRegisters {
  [key: string]: number;
  code_segment: number;
  data_segment: number;
  extra_segment: number;
  stack_segment: number;
}

interface Flags {
  [key: string]: boolean;
  carry_flag: boolean;
  direction_flag: boolean;
  auxiliary_carry_flag: boolean;
  interrupt_disable_flag: boolean;
  negative_flag: boolean;
  overflow_flag: boolean;
  zero_flag: boolean;
  pairity_flag: boolean;
}

interface Registers16BitNotGeneralShort {
  [key: string]: number;
  sp: number;
  bp: number;
  si: number;
  di: number;
  ip: number;
  cs: number;
  ds: number;
  es: number;
  ss: number;
}

interface FlagsShort {
  [key: string]: boolean;
  CF: boolean;
  PF: boolean;
  AF: boolean;
  ZF: boolean;
  SF: boolean;
  IF: boolean;
  DF: boolean;
}

interface Ports{
  // [key: string]: number[];
  ports: number[],
}

type Registers16BitNotGeneral = PointerRegisters & SegmentRegisters;
type CPUData = GeneralPurposeRegisters & PointerRegisters & SegmentRegisters & Ports;

export function findKeysOfNotMatchingregisters16BitNotGeneral(
  registers: Registers16BitNotGeneral,
  defaultRegisters: Registers16BitNotGeneral
): string[] {
  const keys = Object.keys(registers);
  const keysOfNotMatchingRegisters = keys.filter(
    (key) => registers[key] !== defaultRegisters[key]
  );
  return keysOfNotMatchingRegisters;
}

export type {
  CPUData,
  Ports,
  GeneralPurposeRegisters,
  PointerRegisters,
  SegmentRegisters,
  Flags,
  Registers16BitNotGeneral,
  Registers16BitNotGeneralShort,
  FlagsShort,
};
