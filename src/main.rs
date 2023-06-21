use std::{collections::HashMap, iter::FromIterator};

use turing_machine::Action::*;
use turing_machine::{State, Symbol, TransitionFunctionBuilder, Universe, Write};

pub fn main() {
    let tape_len = 10_usize;
    let initial_head = 4_usize;

    let s0 = Symbol::from(0);
    let s1 = Symbol::from(1);

    let s_a = State::from(0);
    let s_b = State::from(1);
    let s_c = State::from(2);

    let display_state_as: HashMap<State, String> = HashMap::from_iter([
        (s_a, "A".to_owned()),
        (s_b, "B".to_owned()),
        (s_c, "C".to_owned()),
    ]);

    let mut builder = TransitionFunctionBuilder::default();
    builder.add(s_a, s0, Write::from(s1), R, s_b);
    builder.add(s_a, s1, Write::from(s1), L, s_c);
    builder.add(s_b, s0, Write::from(s1), L, s_a);
    builder.add(s_b, s1, Write::from(s1), R, s_b);
    builder.add(s_c, s0, Write::from(s1), L, s_b);
    builder.add(s_c, s1, Write::from(s1), N, State::halt());

    let transition_function = builder.build();
    let mut universe = Universe::new(vec![s0; tape_len], initial_head, s_a, transition_function);

    println!("machine");
    println!("symbols: {s0}, {s1}");
    println!(
        "states: {}, {}, {}",
        display_state_as[&s_a], display_state_as[&s_b], display_state_as[&s_c]
    );
    println!("transition function:");
    println!("  (current state, scanned symbol) -> (print symbol, move tape, next state)");
    for (input, output) in builder.added() {
        let (cur_s, scanned_s) = (
            display_state_as[&input.current_state()].to_owned(),
            input.scanned_symbol(),
        );

        let next_s = match output.next_state() {
            x if x.is_halted() => x.to_string(),
            _ => display_state_as[&output.next_state()].to_owned(),
        };
        let (print_s, move_t) = (output.print_symbol(), output.move_tape());

        println!("  ({cur_s}, {scanned_s}) -> ({print_s}, {move_t}, {next_s})");
    }

    println!("\ncomputation");
    println!("sequence :: instruction :: tape");
    println!("{:>width$} -> $", "HEAD", width = 27 - 4 + initial_head);

    let mut sequence = 0;
    while !universe.machine.state.is_halted() {
        let state = display_state_as[&universe.machine.state].to_string();
        let tape = universe
            .tape
            .read_to(tape_len)
            .iter()
            .map(|i| format!("{i}"))
            .collect::<String>();
        println!("{sequence:8} :: {state:^11} :: {tape}");

        universe.tick().unwrap();
        sequence += 1;
    }
}
