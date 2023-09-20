use std::io::Write;

pub struct LoggerOptions {
    pub debug: bool,
}

pub fn logger(message: &str, variable: Option<&str>, opts: &LoggerOptions) {
    if opts.debug {
        let mut output_message = message.to_owned();

        if let Some(s) = variable {
            output_message.push_str(&format!(" {}", s));
        }

        println!("{output_message}")
    }
}

pub fn print(message: &str, newline: bool) -> Result<bool, String> {
    if newline {
        writeln!(std::io::stdout(), "{message}").map_err(|e| e.to_string())?;
    } else {
        write!(std::io::stdout(), "{message}").map_err(|e| e.to_string())?;
    }
    std::io::stdout().flush().map_err(|e| e.to_string())?;
    Ok(true)
}
