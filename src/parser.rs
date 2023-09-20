use crate::utils::*;
use std::env;

pub struct Arguments {
    pub debug: bool,
    pub start: bool,
}

pub fn parse_arguments(log_opts: &mut LoggerOptions) -> Arguments {
    let arguments: Vec<String> = env::args().collect();

    let mut return_arguments: Arguments = Arguments {
        debug: false,
        start: false,
    };

    for arg in &arguments {
        match arg.as_str() {
            "--debug" => {
                return_arguments.debug = true;
                log_opts.debug = true;
            }
            "--start" => return_arguments.start = true,
            _ => logger("Invalid argument", Some(arg), log_opts),
        }

        logger("Read argument", Some(arg), log_opts);
    }

    logger("Done reading arguments", None, log_opts);

    return_arguments
}

pub fn parse_input(input: &str) -> Result<bool, String> {
    let log_opts = LoggerOptions { debug: false };

    match input {
        "move" => {
            logger("Action triggered: move", None, &log_opts);
            print("moving", true)?;
            Ok(false)
        }
        "quit" => {
            logger("Action triggered: quit", None, &log_opts);
            print("quitting", true)?;
            Ok(true)
        }
        _ => {
            logger("Action triggered: default", None, &log_opts);
            print("unknown command", true)?;
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
