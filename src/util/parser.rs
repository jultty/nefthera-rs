use crate::util::logger::log;
use crate::util::logger::LoggerOptions;
use std::env;

pub fn parse_input(input: &str) -> Result<bool, String> {
    let log_opts = LoggerOptions { debug: false };

    match input {
        "move" => {
            log("Action triggered: move", None, &log_opts);
            print("moving", true)?;
            Ok(false)
        }
        "quit" => {
            log("Action triggered: quit", None, &log_opts);
            print("quitting", true)?;
            Ok(true)
        }
        _ => {
            log("Unknown command:", Some(input), &log_opts);
            Ok(false)
        }
    }
}

// from clap/examples/repl.rs
pub fn readline() -> Result<String, String> {
    print("> ", false)?;

    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .map_err(|e| e.to_string())?;
    Ok(buffer)
}

pub fn parse_arguments(log_opts: &mut LoggerOptions) -> Arguments {
    let arguments: Vec<String> = env::args().collect();

    let mut return_arguments: Arguments = Arguments {
        debug: false,
        start: false,
        demo: false,
    };

    for arg in &arguments {
        match arg.as_str() {
            "--debug" => {
                return_arguments.debug = true;
                log_opts.debug = true;
            }
            "--demo" => {
                return_arguments.demo = true;
                log("Demo data is enabled", None, log_opts);
            }
            "--start" => return_arguments.start = true,
            _ => log("Invalid argument", Some(arg), log_opts),
        }

        log("Read argument", Some(arg), log_opts);
    }

    log("Done reading arguments", None, log_opts);

    return_arguments
}

pub struct Arguments {
    pub debug: bool,
    pub start: bool,
    pub demo: bool,
}
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
