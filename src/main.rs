use nefthera::demo;
use nefthera::util::logger::*;
use nefthera::util::parser::*;

fn main() -> Result<(), String> {
    let mut log_opts = LoggerOptions { debug: false };
    let arguments = parse_arguments(&mut log_opts);

    if arguments.demo {
        let player = demo::get_demo_character();
        log("Demo character loaded:", Some(&player.name), &log_opts);
    }

    if arguments.start {
        log("Main loop started by start argument", None, &log_opts);

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
                    print(&err, true)?;
                }
            }
        }

        return Ok(());
    }

    log("Exiting: End of file", None, &log_opts);
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
