import React from "react";
import CodeBlock from "@theme/CodeBlock";

interface AddressingModeProps {
    instructionName: string;
}
    

export function RegisterAddressingMode(props: AddressingModeProps) {
  return (
    <>
      <p>
        This is a mode in which the operand is specified in the instruction
        itself. The operand is either a general-purpose register or a segment
        register.
      </p>
      <p>Example:</p>
      <CodeBlock language="asm6502">
        {props.instructionName} AX, BX <br></br>
        {props.instructionName} BL, CH
      </CodeBlock>
    <hr />
    </>
  );
}

export function ImmediateAddressingMode(props: AddressingModeProps) {
  return (
    <>
      <p>
        This is a mode in which the operand is specified in the instruction
        itself. The operand is a constant.
      </p>
      <p>Example:</p>
      <CodeBlock language="asm6502">
        {props.instructionName} AX, 0FFh <br></br>
        {props.instructionName} BX, 0Bh <br></br>
        {props.instructionName} CL, 0h
      </CodeBlock>
      <hr />
    </>
  );
}

export function MemoryAddressingMode(props: AddressingModeProps) {
  return (
    <>
      <p>
        This is a mode in which the operand is specified in the instruction
        itself. The operand is a memory location.
      </p>
      <p>Example:</p>
      <CodeBlock language="asm6502">
        {props.instructionName} AX, [0x100] <br></br>
        {props.instructionName} AL, b.[0x100] <br />
      </CodeBlock>
      (or) Alternatively you can specify the memory location using the `variable
      assignment`
      <p>Example</p>
      <CodeBlock language="asm6502">
        ORG 100h <br></br>
        .DATA <br></br>
        {`    VAR1 DB 0FFh`} <br></br>
        {`    VAR2 DW 0Bh`} <br></br> <br></br>
        CODE: <br></br>
        {`    ${props.instructionName} AL, Var1`} <br />
        {`    ${props.instructionName} BX, Var2`} <br />
        {`    ${props.instructionName} CL, b.[Var2]`} <br />
        {`    ${props.instructionName} DX, w.[Var1]`} <br />
      </CodeBlock>
      <hr />
    </>
  );
}

export function RegisterIndirectAddressingMode(props: AddressingModeProps) {
  return (
    <>
      <p>
        This is a mode in which the operand is specified in the instruction
        itself. The operand is a memory location whose address is contained in a
        register.
      </p>
      <p>Example:</p>
      <CodeBlock language="asm6502">
        {props.instructionName} AX, [BX] <br></br>
        {props.instructionName} AL, b.[BX] <br />
      </CodeBlock>
      <hr />
    </>
  );
}

export function BasePlusIndexedAddressingMode(props: AddressingModeProps) {
  return (
    <>
      <p>
        This is a mode in which the operand is specified in the instruction
        itself. The operand is a memory location whose address is contained in a
        register and an offset.
      </p>
      <p>Example:</p>
      <CodeBlock language="asm6502">
        {props.instructionName} AX, [BX+SI] <br></br>
        {props.instructionName} AL, b.[BX+SI] <br />
      </CodeBlock>
      <hr />
    </>
  );
}

export function BasePlusIndexPlusDisplacementAddressingMode(
    props: AddressingModeProps
) {
  return (
    <>
      <p>
        This is a mode in which the operand is specified in the instruction
        itself. The operand is a memory location whose address is contained in a
        register and an offset and a displacement.
      </p>
      <p>Example:</p>
      <CodeBlock language="asm6502">
        {props.instructionName} AX, [BX+SI+10h] <br />
        {props.instructionName} AX, [BX+SI+100h] <br />
        {props.instructionName} AL, b.[BX+SI+10h] <br />
        {props.instructionName} AL, b.[BX+SI+100h] <br />
          </CodeBlock>
          <hr />
    </>
  );
}

export function RegisterAndMemoryAddressing(props: AddressingModeProps) {
  return (
    <>
      <p>
        This is a mode in which the operand is specified as a register while the other operand is specified as a memory location.
        The second operand is a memory location whose address is contained in a register (or) an offset. The second operand might also have an displacement.
    </p>
    <p>Example: </p>
      <CodeBlock language="asm6502">
        org 100h <br/>
        data <br />
        {`    VAR DB 0FFh`} <br />
        {`    VAR2 DW 0Bh`} <br />
        code: <br />
        {`    ${props.instructionName} AX, [BX] `}<br />
        {`    ${props.instructionName} DX, [BX+SI] `}<br />
        {`    ${props.instructionName} SP, [BX+SI+10h] `}<br />
        {`    ${props.instructionName} DI, [BX+SI+100h] `}<br />
        {`    ${props.instructionName} BX, Var `}<br />
        {`    ${props.instructionName} BX, w.[Var] `}<br />
        {`    ${props.instructionName} BX, w.[Var2] `}<br />
        </CodeBlock>
    </>
  )
}
