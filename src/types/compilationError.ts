import { languages } from "monaco-editor/esm/vs/editor/editor.api";
let completionTypes = languages.CompletionItemKind;
export interface Suggestion<T> {
  value: T;
  type:
    | "instruction"
    | "register16bit"
    | "register8bit"
    | "define"
    | "variable16bit"
    | "variable8bit"
    | "label"
    | "constant16bit"
    | "constant8bit";
}

export let suggestionToCompletionProvider = {
  instruction: completionTypes.Keyword,
  register16bit: completionTypes.Field,
  register8bit: completionTypes.Field,
  define: completionTypes.Keyword,
  variable16bit: completionTypes.Variable,
  variable8bit: completionTypes.Variable,
  label: completionTypes.Reference,
  constant16bit: completionTypes.Constant,
  constant8bit: completionTypes.Constant,
};

export interface CompilationError {
  column_number: number;
  length: number;
  line_number: number;
  message: string;
  suggestions: Suggestion<any>[] | null;
}

export function compilationErrorToSuggestions(
  compilationErrors: CompilationError[] | null | undefined,
  lineNumber: number,
  _: number
): Suggestion<string>[] | null {
  if (!compilationErrors) {
    return null;
  }
  for (let i = 0; i < compilationErrors.length; i++) {
    const err = compilationErrors[i];
    if (err.line_number !== lineNumber - 1) {
      continue;
    }
    if (err.suggestions === undefined || err.suggestions === null) {
      continue;
    }
    let suggestionsArray: Suggestion<string>[] = [];
    for (let i = 0; i < err.suggestions.length; i++) {
      const suggestion = err.suggestions[i];
      suggestionsArray.push({
        value: suggestion.value.toString(),
        type: suggestion.type,
      });
    }
    return suggestionsArray;
  }
  return null;
}
