import {
  CPUData,
  Flags,
  FlagsShort,
  GeneralPurposeRegisters,
  PointerRegisters,
  Registers16BitNotGeneralShort,
  SegmentRegisters,
} from "./CPUData";
import {
  getDefaultRegisters,
  getDefaultFlags,
  getDefaultGeneralPurposeRegisters,
  getDefaultPointerRegisters,
  getDefaultSegmentRegisters,
} from "./getDefaultRegistersAndFlags";

export function extractCPUData(data: CPUData & Flags): CPUData {
  const keys = Object.keys(data);
  const cpuData: CPUData = getDefaultRegisters();
  keys.forEach((key) => {
    cpuData[key] = data[key];
  });
  return cpuData;
}

export function extractFlags(data: CPUData & Flags): Flags {
  const keys = Object.keys(data);
  const flags: Flags = getDefaultFlags();
  keys.forEach((key) => {
    flags[key] = data[key];
  });
  return flags;
}

export function extractGeneralPurposeRegisters(
  data: CPUData
): GeneralPurposeRegisters {
  const keys = Object.keys(data);
  const generalPurposeRegisters: GeneralPurposeRegisters =
    getDefaultGeneralPurposeRegisters();
  keys.forEach((key) => {
    generalPurposeRegisters[key] = data[key];
  });
  return generalPurposeRegisters;
}

export function extractPointerRegisters(data: CPUData): PointerRegisters {
  const keys = Object.keys(data);
  const pointerRegisters: PointerRegisters = getDefaultPointerRegisters();
  keys.forEach((key) => {
    pointerRegisters[key] = data[key];
  });
  return pointerRegisters;
}

export function extractSegmentRegisters(data: CPUData): SegmentRegisters {
  const keys = Object.keys(data);
  const segmentRegisters: SegmentRegisters = getDefaultSegmentRegisters();
  keys.forEach((key) => {
    segmentRegisters[key] = data[key];
  });
  return segmentRegisters;
}

export function extractNonGeneral16bitRegisters(
  data: CPUData
): Registers16BitNotGeneralShort {
  return {
    sp: data.stack_pointer,
    bp: data.base_pointer,
    si: data.source_index,
    di: data.destination_index,
    ip: data.instruction_pointer,
    cs: data.code_segment,
    ds: data.data_segment,
    es: data.extra_segment,
    ss: data.stack_segment,
  };
}

export function extractFlagsShort(data: Flags): FlagsShort {
  return {
    CF: data.carry_flag,
    PF: data.pairity_flag,
    AF: data.auxiliary_carry_flag,
    ZF: data.zero_flag,
    SF: data.negative_flag,
    OF: data.overflow_flag,
    IF: data.interrupt_disable_flag,
    DF: data.direction_flag,
  };
}
