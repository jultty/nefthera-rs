use std::env;
use std::io;

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

fn parse_input(input: String) {
    let log_opts = LoggerOptions { debug: false };

    match input.as_str() {
        "move" => logger("Action triggered: move", None, &log_opts),
        _ => logger("Action triggered: default", None, &log_opts),
    }
}

fn main() -> io::Result<()> {
    let mut log_opts = LoggerOptions { debug: false };
    let arguments = parse_arguments(&mut log_opts);

    if arguments.start {
        loop {
            logger("Main loop started by start argument", None, &log_opts);

            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(_) => {
                    if input == "exit" {
                        break;
                    } else {
                        parse_input(input);
                    }
                }
                Err(error) => println!("Error: {error}"),
            }
        }
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
