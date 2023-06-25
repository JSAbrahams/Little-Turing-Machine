use std::{env, io};

use turing_machine::display::animation::animate;
use turing_machine::display::cli::print_machine;
use turing_machine::presets::busy_beaver::pick_beaver;
use turing_machine::presets::UniverseMetaData;

pub enum Mode {
    Cli,
    Animation,
}

impl TryFrom<Option<String>> for Mode {
    type Error = String;

    fn try_from(value: Option<String>) -> Result<Self, Self::Error> {
        match value.map(|i| i.trim().to_string()) {
            Some(x) if x == "cli" || x == "c" => Ok(Mode::Cli),
            Some(x) if x == "animation" || x == "a" => Ok(Mode::Animation),
            Some(x) => Err(format!(
                "{x} is not a valid mode, select 'cli/c' or 'animation/a'"
            )),
            None => Err("Select valid mode, 'cli/c' or 'animation/a'".to_owned()),
        }
    }
}

pub fn main() -> Result<(), String> {
    let stdin = io::stdin();

    let args: Vec<String> = env::args().collect();
    let mode = if let Some(arg) = args.get(1) {
        Mode::try_from(Some(arg.to_owned()))
    } else {
        loop {
            let mut cli_input = String::new();
            stdin.read_line(&mut cli_input).unwrap();

            if let Ok(mode) = Mode::try_from(Some(cli_input)) {
                break Ok(mode);
            }
        }
    }?;

    let mut busy_beaver_packed: Option<UniverseMetaData> = None;

    // check command line args first ...
    if let Some(user_input) = args.get(2) {
        let input = user_input.as_str().trim();
        busy_beaver_packed = pick_beaver(input);
    }

    // ... then do interactive stuff
    let stdin = io::stdin();
    let mut user_input = String::new();
    while busy_beaver_packed.is_none() {
        stdin.read_line(&mut user_input).unwrap();
        let input = user_input.as_str().trim();
        busy_beaver_packed = pick_beaver(input);

        user_input.clear();
        println!()
    }

    if let Some(busy_beaver_packed) = busy_beaver_packed {
        match mode {
            Mode::Cli => print_machine(busy_beaver_packed),
            Mode::Animation => animate(busy_beaver_packed),
        };
    }

    Ok(())
}
