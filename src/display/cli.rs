use crate::presets::UniverseMetaData;

pub fn print_machine(busy_beaver_packed: UniverseMetaData) {
    let (name, initial_head) = (busy_beaver_packed.name, busy_beaver_packed.head_offset_hint);
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

    macro_rules! print_state {
        ($state:expr) => {
            match $state {
                x if x.is_halted() => x.to_string(),
                _ => display_state_as[$state].to_owned(),
            }
        };
    }

    for (input, output) in builder.added() {
        let (cur_s, scanned_s) = (
            display_state_as[&input.current_state()].to_owned(),
            input.scanned_symbol(),
        );

        let next_s = print_state!(&output.next_state());
        let (print_s, move_h) = (output.print_symbol(), output.move_head());
        println!("  ({cur_s}, {scanned_s}) -> ({print_s}, {move_h}, {next_s})");
    }

    println!("\ncomputation");
    println!("sequence :: instr :: tape");
    println!(
        "{:>width$} -> $ (start position)",
        "HEAD",
        width = 21 - 4 + initial_head
    );

    let mut sequence = 0;
    while !universe.machine.state.is_halted() {
        let state = print_state!(&universe.machine.state);
        println!("{sequence:8} :: {state:^5} :: {}", universe.tape);

        universe.tick().unwrap();
        sequence += 1;
    }

    let state = print_state!(&universe.machine.state);
    println!("{sequence:8} :: {state:^5} :: {}", universe.tape);
}
