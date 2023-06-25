use std::{env, io};

use turing_machine::presets::busy_beaver::pick_beaver;
use turing_machine::presets::UniversePacked;
use turing_machine::util::print_machine;

pub fn main() {
    let mut user_input = String::new();

    let mut busy_beaver_packed: Option<UniversePacked> = None;

    // check command line args first ...
    let args: Vec<String> = env::args().collect();
    if let Some(user_input) = args.get(1) {
        let input = user_input.as_str().trim();
        busy_beaver_packed = pick_beaver(input);
    }

    // ... then do interactive stuff
    let stdin = io::stdin();
    while busy_beaver_packed.is_none() {
        stdin.read_line(&mut user_input).unwrap();
        let input = user_input.as_str().trim();
        busy_beaver_packed = pick_beaver(input);

        user_input.clear();
        println!()
    }

    if let Some(busy_beaver_packed) = busy_beaver_packed {
        print_machine(busy_beaver_packed);
    }
}
