import { Flags, GeneralPurposeRegisters, PointerRegisters, SegmentRegisters } from "./CPUData";

export function getDefaultGeneralPurposeRegisters(): GeneralPurposeRegisters {
  return {
    ax: 0,
    bx: 0,
    cx: 0,
    dx: 0,
  }
}

export function getDefaultPointerRegisters(): PointerRegisters {
  return {
    base_pointer: 0,
    destination_index: 0,
    instruction_pointer: 0,
    source_index: 0,
    stack_pointer: 0,
  }
}

export function getDefaultSegmentRegisters(): SegmentRegisters {
  return {
    code_segment: 0,
    data_segment: 0,
    extra_segment: 0,
    stack_segment: 0,
  }
}

export function getDefaultRegisters(): GeneralPurposeRegisters &
  PointerRegisters &
  SegmentRegisters {
  return {
    ...getDefaultGeneralPurposeRegisters(),
    ...getDefaultPointerRegisters(),
    ...getDefaultSegmentRegisters(),
    // ...defaultFlags,
  };
}

function getDefaultFlags(): Flags {
  return {
    carry_flag: false,
    direction_flag: false,
    auxiliary_carry_flag: false,
    interrupt_disable_flag: false,
    negative_flag: false,
    overflow_flag: false,
    zero_flag: false,
    pairity_flag: false,
  };
}

export {getDefaultFlags};

