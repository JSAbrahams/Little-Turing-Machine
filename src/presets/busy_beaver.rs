use std::{collections::HashMap, iter::FromIterator};

use crate::universe::function::TransitionFunctionBuilder;
use crate::universe::machine::{Action::*, State, Write};
use crate::universe::{Symbol, Universe};

use super::UniverseMetadata;

pub fn one_state_busy_beaver() -> UniverseMetadata {
    let name = String::from("2-state, 2-symbol busy beaver");
    let initial_head = 0_usize;

    let s0 = Symbol::empty();
    let s1 = Symbol::from(1);

    let s_a = State::from(0);
    let display_state_as: HashMap<State, String> = HashMap::from_iter([(s_a, "A".to_owned())]);

    let mut builder = TransitionFunctionBuilder::default();
    builder.add(s_a, s0, Write::from(s1), R, State::halt());
    builder.add(s_a, s1, Write::from(s1), L, State::halt());

    let transition_function = builder.build();
    let universe = Universe::new(vec![], initial_head, s_a, transition_function);

    UniverseMetadata {
        name,
        head_offset_hint: initial_head,
        symbol_set: vec![s0, s1],
        state_set: vec![s_a],
        display_state_as,
        transition_function_buidler: builder,
        universe,
    }
}

pub fn two_state_busy_beaver() -> UniverseMetadata {
    let name = String::from("2-state, 2-symbol busy beaver");
    let initial_head = 2_usize;

    let s0 = Symbol::empty();
    let s1 = Symbol::from(1);

    let s_a = State::from(0);
    let s_b = State::from(1);

    let display_state_as: HashMap<State, String> =
        HashMap::from_iter([(s_a, "A".to_owned()), (s_b, "B".to_owned())]);

    let mut builder = TransitionFunctionBuilder::default();
    builder.add(s_a, s0, Write::from(s1), R, s_b);
    builder.add(s_a, s1, Write::from(s1), L, s_b);

    builder.add(s_b, s0, Write::from(s1), L, s_a);
    builder.add(s_b, s1, Write::from(s1), R, State::halt());

    let transition_function = builder.build();
    let universe = Universe::new(vec![], initial_head, s_a, transition_function);

    UniverseMetadata {
        name,
        head_offset_hint: initial_head,
        symbol_set: vec![s0, s1],
        state_set: vec![s_a, s_b],
        display_state_as,
        transition_function_buidler: builder,
        universe,
    }
}

pub fn three_state_busy_beaver() -> UniverseMetadata {
    let name = String::from("3-state, 2-symbol busy beaver");
    let initial_head = 1_usize;

    let s0 = Symbol::empty();
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
    builder.add(s_a, s1, Write::from(s1), R, State::halt());

    builder.add(s_b, s0, Write::from(s0), R, s_c);
    builder.add(s_b, s1, Write::from(s1), R, s_b);

    builder.add(s_c, s0, Write::from(s1), L, s_c);
    builder.add(s_c, s1, Write::from(s1), L, s_a);

    let transition_function = builder.build();
    let universe = Universe::new(vec![], initial_head, s_a, transition_function);

    UniverseMetadata {
        name,
        head_offset_hint: initial_head,
        symbol_set: vec![s0, s1],
        state_set: vec![s_a, s_b, s_c],
        display_state_as,
        transition_function_buidler: builder,
        universe,
    }
}

pub fn four_state_busy_beaver() -> UniverseMetadata {
    let name = String::from("4-state, 2-symbol busy beaver");
    let initial_head = 9_usize;

    let s0 = Symbol::empty();
    let s1 = Symbol::from(1);

    let s_a = State::from(0);
    let s_b = State::from(1);
    let s_c = State::from(2);
    let s_d = State::from(3);

    let display_state_as: HashMap<State, String> = HashMap::from_iter([
        (s_a, "A".to_owned()),
        (s_b, "B".to_owned()),
        (s_c, "C".to_owned()),
        (s_d, "D".to_owned()),
    ]);

    let mut builder = TransitionFunctionBuilder::default();
    builder.add(s_a, s0, Write::from(s1), R, s_b);
    builder.add(s_a, s1, Write::from(s1), L, s_b);

    builder.add(s_b, s0, Write::from(s1), L, s_a);
    builder.add(s_b, s1, Write::from(s0), L, s_c);

    builder.add(s_c, s0, Write::from(s1), R, State::halt());
    builder.add(s_c, s1, Write::from(s1), L, s_d);

    builder.add(s_d, s0, Write::from(s1), R, s_d);
    builder.add(s_d, s1, Write::from(s0), R, s_a);

    let transition_function = builder.build();
    let universe = Universe::new(vec![], initial_head, s_a, transition_function);

    UniverseMetadata {
        name,
        head_offset_hint: initial_head,
        symbol_set: vec![s0, s1],
        state_set: vec![s_a, s_b, s_c],
        display_state_as,
        transition_function_buidler: builder,
        universe,
    }
}

pub fn five_state_busy_beaver() -> UniverseMetadata {
    let name = String::from("5-state, 2-symbol busy beaver");
    let initial_head = 300_usize; // eyeball figure

    let s0 = Symbol::empty();
    let s1 = Symbol::from(1);

    let s_a = State::from(0);
    let s_b = State::from(1);
    let s_c = State::from(2);
    let s_d = State::from(3);
    let s_e = State::from(4);

    let display_state_as: HashMap<State, String> = HashMap::from_iter([
        (s_a, "A".to_owned()),
        (s_b, "B".to_owned()),
        (s_c, "C".to_owned()),
        (s_d, "D".to_owned()),
        (s_e, "E".to_owned()),
    ]);

    let mut builder = TransitionFunctionBuilder::default();
    builder.add(s_a, s0, Write::from(s1), R, s_b);
    builder.add(s_a, s1, Write::from(s1), L, s_c);

    builder.add(s_b, s0, Write::from(s1), R, s_c);
    builder.add(s_b, s1, Write::from(s1), R, s_b);

    builder.add(s_c, s0, Write::from(s1), R, s_d);
    builder.add(s_c, s1, Write::from(s0), L, s_e);

    builder.add(s_d, s0, Write::from(s1), L, s_a);
    builder.add(s_d, s1, Write::from(s1), L, s_d);

    builder.add(s_e, s0, Write::from(s1), R, State::halt());
    builder.add(s_e, s1, Write::from(s0), L, s_a);

    let transition_function = builder.build();
    let universe = Universe::new(vec![], initial_head, s_a, transition_function);

    UniverseMetadata {
        name,
        head_offset_hint: initial_head,
        symbol_set: vec![s0, s1],
        state_set: vec![s_a, s_b, s_c],
        display_state_as,
        transition_function_buidler: builder,
        universe,
    }
}

#[cfg(test)]
mod tests {
    use super::{one_state_busy_beaver, three_state_busy_beaver, two_state_busy_beaver};
    use crate::universe::Symbol;

    #[test]
    fn verify_one_state_busy_beaver() {
        let s1 = Symbol::from(1);
        let mut beaver = one_state_busy_beaver();

        macro_rules! verify_symbols {
            ($symbols:expr) => {{
                assert_eq!(beaver.universe.tape.all_symbols(), $symbols.to_vec());
                beaver.universe.tick().unwrap();
            }};
        }

        verify_symbols!([]);
        verify_symbols!([s1]);
    }

    #[test]
    fn verify_two_state_busy_beaver() {
        let (s0, s1) = (Symbol::empty(), Symbol::from(1));
        let mut beaver = two_state_busy_beaver();

        macro_rules! verify_symbols {
            ($symbols:expr) => {{
                assert_eq!(beaver.universe.tape.all_symbols(), $symbols.to_vec());
                beaver.universe.tick().unwrap();
            }};
        }

        verify_symbols!([]);
        verify_symbols!([s0, s0, s1]);
        verify_symbols!([s0, s0, s1, s1]);
        verify_symbols!([s0, s0, s1, s1]);
        verify_symbols!([s0, s1, s1, s1]);
        verify_symbols!([s1, s1, s1, s1]);
    }

    #[test]
    fn verify_three_state_busy_beaver() {
        let (s0, s1) = (Symbol::empty(), Symbol::from(1));
        let mut beaver = three_state_busy_beaver();

        macro_rules! verify_symbols {
            ($symbols:expr) => {{
                assert_eq!(beaver.universe.tape.all_symbols(), $symbols.to_vec());
                beaver.universe.tick().unwrap();
            }};
        }

        verify_symbols!([]);
        verify_symbols!([s0, s1]);
        verify_symbols!([s0, s1, s0]);
        verify_symbols!([s0, s1, s0, s1]);
        verify_symbols!([s0, s1, s1, s1]);
        verify_symbols!([s0, s1, s1, s1]);
        verify_symbols!([s1, s1, s1, s1]);
        verify_symbols!([s1, s1, s1, s1]);
        verify_symbols!([s1, s1, s1, s1]);
        verify_symbols!([s1, s1, s1, s1]);
        verify_symbols!([s1, s1, s1, s1, s0]);
        verify_symbols!([s1, s1, s1, s1, s0, s1]);
        verify_symbols!([s1, s1, s1, s1, s1, s1]);
        verify_symbols!([s1, s1, s1, s1, s1, s1]);
    }
}
