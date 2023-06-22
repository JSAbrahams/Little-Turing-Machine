use std::{collections::HashMap, iter::FromIterator};

use turing_machine::{presets::*, State, Symbol, Tape, TransitionFunctionBuilder, Universe};

#[allow(clippy::too_many_arguments)] // for now
fn print_machine(
    name: &str,
    initial_head: usize,
    symbols: &[Symbol],
    states: &[State],
    display_state_as: HashMap<State, String>,
    initial_tape: &[Symbol],
    builder: TransitionFunctionBuilder,
    mut universe: Universe,
) {
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
        "initial tape: {}",
        Tape::from_iter(initial_tape.iter().cloned())
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
    println!("{:>width$} -> $", "HEAD", width = 21 - 4 + initial_head);

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

pub fn main() {
    let (name, initial_head, symbols, states, display_state_as, initial_tape, builder, universe) =
        three_state_busy_beaver();
    print_machine(
        &name,
        initial_head,
        &symbols,
        &states,
        display_state_as,
        &initial_tape,
        builder,
        universe,
    );

    println!("-----------------------------------------------------");
    let (name, initial_head, symbols, states, display_state_as, initial_tape, builder, universe) =
        four_state_busy_beaver();
    print_machine(
        &name,
        initial_head,
        &symbols,
        &states,
        display_state_as,
        &initial_tape,
        builder,
        universe,
    );
}
