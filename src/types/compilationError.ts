export interface CompilationError {
  column_number: number;
  length: number;
  line_number: number;
  message: string;
  suggestions: string[];
}

export function compilationErrorToSuggestions(
  compilationErrors: CompilationError[] | null | undefined,
  lineNumber: number,
  _: number
): string[] | null {
  if (!compilationErrors) {
    return null;
  }
  for (let i = 0; i < compilationErrors.length; i++) {
    const err = compilationErrors[i];
    if (err.line_number === lineNumber - 1) {
      if (err.suggestions === undefined || err.suggestions === null) {
        continue;
      }
    }
    let suggestionsArray = err.suggestions;
    for (let i = 0; i < suggestionsArray.length; i++) {
      suggestionsArray[i] = suggestionsArray[i].toString();
    }
    return suggestionsArray;
  }
  return null;
}
