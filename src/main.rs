use std::env;

struct Arguments {
    debug: bool,
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

    let mut return_arguments: Arguments = Arguments { debug: false };

    for arg in &arguments {
        if arg == "--debug" {
            return_arguments.debug = true;
            log_opts.debug = true;
        }

        let mut log_message = "Read argument ".to_owned();
        log_message.push_str(arg);
        logger(&log_message, log_opts);
    }

    logger("Done reading arguments", log_opts);

    return_arguments
}

fn main() {
    let mut log_opts = LoggerOptions { debug: false };
    let _arguments = parse_arguments(&mut log_opts);

    logger("", &log_opts);
}
