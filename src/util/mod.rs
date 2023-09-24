pub mod instruction;
pub mod logger;
pub mod parser;

use std::io::Write;

pub fn print(message: &str, newline: bool) -> Result<bool, String> {
    if newline {
        writeln!(std::io::stdout(), "{message}").map_err(|e| e.to_string())?;
    } else {
        write!(std::io::stdout(), "{message}").map_err(|e| e.to_string())?;
    }
    std::io::stdout().flush().map_err(|e| e.to_string())?;
    Ok(true)
}
