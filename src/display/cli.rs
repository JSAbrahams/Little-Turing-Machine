use crate::{display::display_state, presets::UniverseMetadata};

pub fn print_machine(busy_beaver_packed: UniverseMetadata) {
    let name = busy_beaver_packed.name;
    let (symbols, states) = (busy_beaver_packed.symbol_set, busy_beaver_packed.state_set);
    let display_state_as = busy_beaver_packed.display_state_as;
    let mut universe = busy_beaver_packed.universe;
    let builder = busy_beaver_packed.transition_function_buidler;

    println!("machine: {name}");
    println!(
        "symbols: {}",
        symbols.iter().map(|s| format!("{s} ")).collect::<String>()
    );
    println!(
        "states: {}",
        states
            .iter()
            .map(|s| format!("{} ", display_state_as[s]))
            .collect::<String>()
    );

    println!(
        "initial state: {}",
        display_state_as[&universe.machine.state]
    );

    println!("transition function:");
    println!("  (current state, scanned symbol) -> (print symbol, move tape, next state)");

    for (input, output) in builder.added() {
        let (cur_s, scanned_s) = (display_state(input.state, &display_state_as), input.symbol);

        let next_s = display_state(output.state, &display_state_as);
        let (print_s, move_h) = (output.write, output.action);
        println!("  ({cur_s}, {scanned_s}) -> ({print_s}, {move_h}, {next_s})");
    }

    println!("\ncomputation");
    println!("sequence :: instr :: HEAD :: tape");

    let mut sequence = 0;
    while !universe.machine.state.is_halted() {
        let state = display_state(universe.machine.state, &display_state_as);
        println!(
            "{sequence:8} :: {state:^5} :: {:^4} :: {}",
            universe.pos, universe.tape
        );

        universe.tick().unwrap();
        sequence += 1;
    }

    let state = display_state(universe.machine.state, &display_state_as);
    println!(
        "{sequence:8} :: {state:^5} :: {:^4} :: {}",
        universe.pos, universe.tape
    );
}
