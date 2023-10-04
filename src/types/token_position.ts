interface TokenPosition {
  column_number: number;
  length: number;
  line_number: number;
}

type DefintionTokenPosition = TokenPosition;
type ReferenceTokenPosition = TokenPosition;
export type Definitions = Array<[DefintionTokenPosition, ReferenceTokenPosition]>;

export function find_matching_reference_positions(
  definitions: Definitions,
  reference: ReferenceTokenPosition
): DefintionTokenPosition | null {
  for (let i = 0; i < definitions.length; i++) {
    const definition = definitions[i];
    const def = definition[0];
    const ref = definition[1];
    if (
      ref.line_number === reference.line_number &&
      ref.column_number === reference.column_number 
    ) {
      return def;
    }
  }
  return null;
}
