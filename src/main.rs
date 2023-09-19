use std::env;

const DEBUG: bool = true;

fn logger(message: &str) {
    if DEBUG {
        println!("{message}");
    }
}

fn parse_arguments() {
    logger("Reading arguments");
    let args: Vec<String> = env::args().collect();
    dbg!(args);
}

fn main() {
    parse_arguments();
}
