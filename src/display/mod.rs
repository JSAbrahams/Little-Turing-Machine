use std::collections::HashMap;

use crate::State;

pub mod animation;
pub mod cli;

pub type DisplayStateAs = HashMap<State, String>;

pub fn display_state(state: State, display_state_as: &DisplayStateAs) -> String {
    match state {
        x if x.is_halted() => x.to_string(),
        x => {
            if let Some(display) = display_state_as.get(&x) {
                display.to_owned()
            } else {
                format!("{state}")
            }
        }
    }
}
