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
        This is a mode in which the operand is specified as a register while the
        other operand is specified as a memory location. The second operand is a
        memory location whose address is contained in a register (or) an offset.
        The second operand might also have an displacement.
      </p>
      <p>Example: </p>
      <CodeBlock language="asm6502" showLineNumbers={true}>
        org 100h <br />
        data <br />
        {`    VAR DB 0FFh`} <br />
        {`    VAR2 DW 0Bh`} <br />
        code: <br />
        {`    ${props.instructionName} BX, Var `}
        <br />
        {`    ${props.instructionName} BX, w.[Var] `}
        <br />
        {`    ${props.instructionName} BX, w.[Var2] `}
        <br /> <br />
        {`    ${props.instructionName} AX, [BX] `}
        <br />
        {`    ${props.instructionName} DX, [BX+SI] `}
        <br />
        {`    ${props.instructionName} SP, [BX+SI+10h] `}
        <br />
        {`    ${props.instructionName} DI, [BX+SI+100h] `}
        <br />
      </CodeBlock>
    </>
  );
}

interface AddressingModeTableProps {
  instructionName: string;

  reg_16bit_and_anything_ins: number;
  reg_8bit_and_anything_ins: number;
  indexed_addressing_and_anyting_ins: number;
  addr_and_8bit_reg: number;

  al_and_num_ins: number | null;
  ax_and_num_ins: number | null;

  reg16bit_and_16bit_num: number;
  reg16bit_and_8bit_num: number;
  reg8bit_and_num: number;
  reg_num_sub_ins: number;

  addr16bit_and_16bit_num: number;
  addr16bit_and_8bit_num: number;
  addr8bit_and_num: number;
  addr_num_sub_ins: number;
}

export function GenerateCompilationTable(props: AddressingModeTableProps) {
  return (
    <table>
      <thead>
        <tr>
          <th>Operand</th>
          <th>Opcode</th>
          <th>Size(bytes)</th>
          <th>Example instruction</th>
        </tr>
      </thead>
      <tbody>
        <tr>
          <td>reg16, indirect mem</td>
          <td>{props.reg_16bit_and_anything_ins} 0x00..=0x3F</td>
          <td>2</td>
          <td>{`${props.instructionName} AX, [BX]`}</td>
        </tr>
        <tr>
          <td>reg16, direct mem</td>
          <td>
            {props.reg_16bit_and_anything_ins} {"0x06 | reg_idx << 3"}
          </td>
          <td>4</td>
          <td>{`${props.instructionName} DX, [0x100]`}</td>
        </tr>
        <tr>
          <td>reg16, indirect mem with 8bit offset</td>
          <td>{props.reg_16bit_and_anything_ins} 0x40..=0x7F 0x00..=0xFF</td>
          <td>3</td>
          <td>{`${props.instructionName} AX, [BX+0x10]`}</td>
        </tr>
        <tr>
          <td>reg16, indirect mem with 16bit offset</td>
          <td>{props.reg_16bit_and_anything_ins} 0x80..=0xBF 0x00..=0xFF</td>
          <td>4</td>
          <td>{`${props.instructionName} AX, [BX+0x100]`}</td>
        </tr>
        <tr>
          <td>reg16, reg16</td>
          <td>{props.reg_16bit_and_anything_ins} 0xC0..=0xFF</td>
          <td>2</td>
          <td>{`${props.instructionName} AX, BX`}</td>
        </tr>
        <tr>
          <td>reg8, indirect mem</td>
          <td>{props.reg_8bit_and_anything_ins} 0x00..=0x3F</td>
          <td>2</td>
          <td>{`${props.instructionName} AL, [BX]`}</td>
        </tr>
        <tr>
          <td>reg8, direct mem</td>
          <td>
            {props.reg_8bit_and_anything_ins} {"0x06 | reg_idx << 3"}
          </td>
          <td>4</td>
          <td>{`${props.instructionName} CL, [0x100]`}</td>
        </tr>
        <tr>
          <td>reg8, indirect mem with 8bit offset</td>
          <td>{props.reg_8bit_and_anything_ins} 0x40..=0x7F 0x00..=0xFF</td>
          <td>3</td>
          <td>{`${props.instructionName} AL, [BX+0x10]`}</td>
        </tr>
        <tr>
          <td>reg8, indirect mem with 16bit offset</td>
          <td>{props.reg_8bit_and_anything_ins} 0x80..=0xBF 0x00..=0xFF</td>
          <td>4</td>
          <td>{`${props.instructionName} AL, [BX+0x100]`}</td>
        </tr>
        <tr>
          <td>reg8, reg8</td>
          <td>{props.reg_8bit_and_anything_ins} 0xC0..=0xFF</td>
          <td>2</td>
          <td>{`${props.instructionName} AL, BL`}</td>
        </tr>
        <tr>
          <td>indirect mem, reg 16</td>
          <td>{props.indexed_addressing_and_anyting_ins} 0x00..=0x3F</td>
          <td>2</td>
          <td>{`${props.instructionName} [BX+SI], AX`}</td>
        </tr>
        <tr>
          <td>direct mem, reg 16</td>
          <td>
            {props.indexed_addressing_and_anyting_ins} {"0x06 | reg_idx << 3"}
          </td>
          <td>4</td>
          <td>{`${props.instructionName} [0x100], BP`}</td>
        </tr>
        <tr>
          <td>indirect mem with 8bit offset, reg 16</td>
          <td>
            {props.indexed_addressing_and_anyting_ins} 0x40..=0x7F 0x00..=0xFF
          </td>
          <td>3</td>
          <td>{`${props.instructionName} [BX+SI+0x10], CX`}</td>
        </tr>
        <tr>
          <td>indirect mem with 16bit offset, reg 16</td>
          <td>
            {props.indexed_addressing_and_anyting_ins} 0x80..=0xBF 0x00..=0xFF
          </td>
          <td>4</td>
          <td>{`${props.instructionName} [BX+SI+0x100], DX`}</td>
        </tr>

        <tr>
          <td>direct mem, reg 8</td>
          <td>
            {props.addr_and_8bit_reg} {"0x06 | reg_idx << 3"}
          </td>
          <td>4</td>
          <td>{`${props.instructionName} [0x100], AL`}</td>
        </tr>
        <tr>
          <td>AL, num</td>
          <td>{props.al_and_num_ins} 0x00..=0xFF 0x00..=0xFF</td>
          <td>2</td>
          <td>{`${props.instructionName} AL, 0x10`}</td>
        </tr>
        <tr>
          <td>AX, num</td>
          <td>{props.ax_and_num_ins} 0x00..=0xFF 0x00..=0xFF</td>
          <td>3</td>
          <td>{`${props.instructionName} AX, 0x100`}</td>
        </tr>
        <tr>
          <td>reg16, num16</td>
          <td>
            {props.reg16bit_and_16bit_num} {props.reg_num_sub_ins}+reg_idx
            0x00..=0xFF 0x00..=0xFF
          </td>
          <td>3</td>
          <td>{`${props.instructionName} DX, 0x100`}</td>
        </tr>
        <tr>
          <td>reg16, num8</td>
          <td>
            {props.reg16bit_and_8bit_num} {props.reg_num_sub_ins}+reg_idx
            0x00..=0xFF
          </td>
          <td>2</td>
          <td>{`${props.instructionName} CX, 0x10`}</td>
        </tr>
        <tr>
          <td>reg8, num</td>
          <td>
            {props.reg8bit_and_num} {props.reg_num_sub_ins}+reg_idx 0x00..=0xFF
          </td>
          <td>2</td>
          <td>{`${props.instructionName} AL, 0x10`}</td>
        </tr>
        <tr>
          <td>direct address, num16</td>
          <td>
            {props.addr16bit_and_16bit_num} {props.addr_num_sub_ins}+reg_idx{" "}
            [0x00..=0xFF 0x00..=0xFF] 0x00..=0xFF 0x00..=0xFF
          </td>
          <td>6</td>
          <td>{`${props.instructionName} [0x100], 0x100`}</td>
        </tr>
        <tr>
          <td>direct address, num8</td>
          <td>
            {props.addr16bit_and_16bit_num} {props.addr_num_sub_ins}+reg_idx{" "}
            [0x00..=0xFF 0x00..=0xFF] 0x00..=0xFF{" "}
          </td>
          <td>5</td>
          <td>{`${props.instructionName} [0x100], 0x10`}</td>
        </tr>
        <tr>
          <td>addr8, num</td>
          <td>
            {props.addr8bit_and_num} {props.addr_num_sub_ins}+reg_idx
            [0x00..=0xFF 0x00..=0xFF] 0x00..=0xFF
          </td>
          <td>2</td>
          <td>{`${props.instructionName} b.[0x100], 0x10`}</td>
        </tr>
      </tbody>
    </table>
  );
}
