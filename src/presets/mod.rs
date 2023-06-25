use std::collections::HashMap;

use crate::{State, Symbol, TransitionFunctionBuilder, Universe};

pub mod busy_beaver;

pub struct UniversePacked {
    pub name: String,
    pub initial_head: usize,
    pub symbols: Vec<Symbol>,
    pub states: Vec<State>,
    pub display_state_as: HashMap<State, String>,
    pub transition_function_buidler: TransitionFunctionBuilder,
    pub universe: Universe,
}
