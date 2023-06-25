use std::collections::HashMap;

use crate::{State, Symbol, TransitionFunctionBuilder, Universe};

pub mod busy_beaver;

/// Universe with some metadata for more elegant views.
pub struct UniverseMetaData {
    pub name: String,
    pub head_offset_hint: usize,
    pub symbol_set: Vec<Symbol>,
    pub state_set: Vec<State>,
    pub display_state_as: HashMap<State, String>,
    pub transition_function_buidler: TransitionFunctionBuilder,
    pub universe: Universe,
}
