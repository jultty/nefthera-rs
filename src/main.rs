use std::env;
use std::io::Write;

struct Arguments {
    debug: bool,
    start: bool,
}

struct LoggerOptions {
    debug: bool,
}

fn logger(message: &str, variable: Option<&str>, opts: &LoggerOptions) {
    if opts.debug {
        let mut output_message = message.to_owned();

        if let Some(s) = variable {
            output_message.push_str(&format!(" {}", s));
        }

        println!("{output_message}")
    }
}

fn parse_arguments(log_opts: &mut LoggerOptions) -> Arguments {
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

fn parse_input(input: &str) -> Result<bool, String> {
    let log_opts = LoggerOptions { debug: false };

    match input {
        "move" => { 
            logger("Action triggered: move", None, &log_opts);
            print("moving")?;
            Ok(true)
        },
        _ => { 
            logger("Action triggered: default", None, &log_opts);
            print("unknown command")?;
            Ok(true)
        },
    }
}

fn print(message: &str) -> Result<bool, String> {
    write!(std::io::stdout(), "{message}").map_err(|e| e.to_string())?;
    std::io::stdout().flush().map_err(|e| e.to_string())?;
    Ok(true)
}

// from clap/examples/repl.rs
fn readline() -> Result<String, String> {

    print("> ")?;
    
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .map_err(|e| e.to_string())?;
    Ok(buffer)
}

fn main() -> Result<(), String> {
    let mut log_opts = LoggerOptions { debug: false };
    let arguments = parse_arguments(&mut log_opts);

    if arguments.start {

        logger("Main loop started by start argument", None, &log_opts);
        loop {

            let line = readline()?;
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            match parse_input(line) {
                Ok(quit) => {
                    if quit {
                        break;
                    }
                }
                Err(err) => {
                    write!(std::io::stdout(), "{err}").map_err(|e| e.to_string())?;
                    std::io::stdout().flush().map_err(|e| e.to_string())?;
                }
            }
        }

        return Ok(())
    }

    logger("Exiting: End of file", None, &log_opts);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main_returns_ok() {
        let result = main().expect("Main did not return Ok");
        assert_eq!(result, ());
    }
}
