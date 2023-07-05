use std::collections::HashMap;

use crate::universe::function::TransitionFunctionBuilder;
use crate::universe::machine::State;
use crate::universe::{Symbol, Universe};

use self::busy_beaver::{
    five_state_busy_beaver, four_state_busy_beaver, three_state_busy_beaver, two_state_busy_beaver,
};
use self::counter::counter_binary;

pub mod busy_beaver;
pub mod counter;

/// Universe with some metadata for more elegant views.
#[derive(Debug, Default)]
pub struct UniverseMetadata {
    pub name: String,
    pub head_offset_hint: usize,
    pub symbol_set: Vec<Symbol>,
    pub state_set: Vec<State>,
    pub display_state_as: HashMap<State, String>,
    pub transition_function_buidler: TransitionFunctionBuilder,
    pub universe: Universe,
}

impl TryFrom<String> for UniverseMetadata {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "beaver_2" => Ok(two_state_busy_beaver()),
            "beaver_3" => Ok(three_state_busy_beaver()),
            "beaver_4" => Ok(four_state_busy_beaver()),
            "beaver_5" => Ok(five_state_busy_beaver()),
            "counter_2" => Ok(counter_binary()),
            other => Err(format!("unknown preset: {other}")),
        }
    }
}
