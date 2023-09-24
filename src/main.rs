use nefthera::base::character::Character;
use nefthera::base::space::passage::*;
use nefthera::demo;
use nefthera::lore::locations::passages;
use nefthera::util::{logger::*, parser::*, print};

fn main() -> Result<(), String> {
    let mut player: Character;
    let world: PassageMap;
    let mut log_opts = LoggerOptions { debug: false };
    let arguments = parse_arguments(&mut log_opts);

    if arguments.demo {
        player = demo::get_demo_character();
        // TODO make this a struct with several maps and other needed data
        world = demo::get_passage_map();
        log("Demo character loaded:", Some(&player.name), &log_opts);
    } else {
        player = Character::new_blank();
        world = passages::populate();
        log("Blank character loaded:", Some(&player.name), &log_opts);
    }

    if arguments.start {
        log("Main loop started by start argument", None, &log_opts);

        loop {
            let line = readline()?;
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            match parse_input(line, &world) {
                Ok(instruction) => match instruction.kind.as_str() {
                    "go" => {
                        player.go(instruction);
                    }
                    "sense" => {
                        player.sense(instruction);
                    }
                    "enter_passage" => player.enter_passage(instruction),
                    "quit" => break,
                    _ => {}
                },
                Err(err) => {
                    print(&err, true);
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
