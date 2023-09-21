pub struct LoggerOptions {
    pub debug: bool,
}

pub fn log(message: &str, variable: Option<&str>, opts: &LoggerOptions) {
    if opts.debug {
        let mut output_message = message.to_owned();

        if let Some(s) = variable {
            output_message.push_str(&format!(" {}", s));
        }

        println!("{output_message}")
    }
}
