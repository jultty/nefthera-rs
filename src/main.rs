use std::env;
use std::io;

struct Arguments {
    debug: bool,
    start: bool,
}

struct LoggerOptions {
    debug: bool,
}

fn logger(message: &str, opts: &LoggerOptions) {
    if opts.debug {
        println!("{message}");
    }
}

fn parse_arguments(log_opts: &mut LoggerOptions) -> Arguments {
    let arguments: Vec<String> = env::args().collect();

    let mut return_arguments: Arguments = Arguments {
        debug: false,
        start: false,
    };

    for arg in &arguments {
        if arg == "--debug" {
            return_arguments.debug = true;
            log_opts.debug = true;
        }
        if arg == "--start" {
            return_arguments.start = true;
        }

        let mut log_message = "Read argument ".to_owned();
        log_message.push_str(arg);
        logger(&log_message, log_opts);
    }

    logger("Done reading arguments", log_opts);

    return_arguments
}

fn parse_input(input: String) {
    let log_opts = LoggerOptions { debug: false };

    match input.as_str() {
        "move" => logger("Action triggered: move", &log_opts),
        _other => logger("Action triggered: default", &log_opts),
    }
}

fn main() -> io::Result<()> {
    let mut log_opts = LoggerOptions { debug: false };
    let arguments = parse_arguments(&mut log_opts);

    if arguments.start {
        loop {
            logger("Starting main loop per arguments", &log_opts);

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

    logger("Exiting: End of file", &log_opts);
    Ok(())
}
