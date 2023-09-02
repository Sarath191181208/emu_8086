pub struct CompilationError {
    line_number: u32,
    column_number: u32,
    length: u32,
    message: String,
}

impl CompilationError {
    pub fn new(line_number: u32, column_number: u32, length: u32, message: &str) -> Self {
        Self {
            line_number,
            column_number,
            length,
            message: message.to_string(),
        }
    }

    pub fn print_compilation_error(&self, source: &str) {
        let line = source.lines().nth(self.line_number as usize).unwrap();
        let line_number = self.line_number + 1;
        let column_number = self.column_number ;
        let length = self.length;
        let message = &self.message;
        println!("Error: {}", message);
        println!(" --> {}:{}:{}", line_number, column_number, length);
        println!("  |");
        println!("{} | {}", line_number, line);
        println!("  | {}{}{}", " ".repeat(column_number as usize), "^".repeat(length as usize), " ".repeat(line.len() - column_number as usize));
    }
}