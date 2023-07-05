use std::{collections::HashMap, iter::FromIterator};

use crate::universe::function::TransitionFunctionBuilder;
use crate::universe::machine::{Action::*, State, Write};
use crate::universe::{Symbol, Universe};

use super::UniverseMetadata;

pub fn counter_binary() -> UniverseMetadata {
    let name = String::from("binary counting");
    let initial_head = 0_usize;

    let se = Symbol::empty();
    let s0 = Symbol::from(0);
    let s1 = Symbol::from(1);

    let s_w = State::from(1);
    let s_r = State::from(2);
    let display_state_as: HashMap<State, String> =
        HashMap::from_iter([(s_w, "write".to_owned()), (s_r, "return".to_owned())]);

    let mut builder = TransitionFunctionBuilder::default();

    builder.add(s_w, se, Write::from(s1), R, s_r);
    builder.add(s_w, s0, Write::from(s1), R, s_r);
    builder.add(s_w, s1, Write::from(s0), L, s_w);

    builder.add(s_r, se, Write::None, L, s_w);
    builder.add(s_r, s0, Write::None, R, s_r);
    builder.add(s_r, s1, Write::None, R, s_r);

    let transition_function = builder.build();
    let universe = Universe::new(vec![], initial_head, s_r, transition_function);

    UniverseMetadata {
        name,
        head_offset_hint: initial_head,
        symbol_set: vec![s0, s1],
        state_set: vec![s_w, s_r],
        display_state_as,
        transition_function_buidler: builder,
        universe,
    }
}
