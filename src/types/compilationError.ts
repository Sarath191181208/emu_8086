interface CompilationError {
  column_number: number;
  length: number;
  line_number: number;
  message: string;
  suggestions: string[];
}
