pub mod instruction;
pub mod logger;
pub mod parser;

use std::io::Write;

pub fn print(message: &str, newline: bool) {
    if newline {
        let _ = writeln!(std::io::stdout(), "{message}").map_err(|e| e.to_string());
    } else {
        let _ = write!(std::io::stdout(), "{message}").map_err(|e| e.to_string());
    }
    let _ = std::io::stdout().flush().map_err(|e| e.to_string());
}
