use std::collections::HashMap;

use super::machine::{Action, State, Write};
use super::Symbol;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Input {
    pub state: State,
    pub symbol: Symbol,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Output {
    pub write: Write,
    pub action: Action,
    pub state: State,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TransitionFunction(HashMap<Input, Output>);

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct TransitionFunctionBuilder(Vec<(Input, Output)>);

impl TransitionFunction {
    pub fn act(&self, current_state: State, scanned_symbol: Symbol) -> Result<Output, String> {
        self.0
            .get(&Input {
                state: current_state,
                symbol: scanned_symbol,
            })
            .cloned()
            .ok_or_else(|| format!("{current_state}, {scanned_symbol} -> ?"))
    }
}

impl TransitionFunctionBuilder {
    pub fn add(
        &mut self,
        input_state: State,
        input_symbol: Symbol,
        write: Write,
        action: Action,
        next_state: State,
    ) {
        self.0.push((
            Input {
                state: input_state,
                symbol: input_symbol,
            },
            Output {
                write,
                action,
                state: next_state,
            },
        ));
    }

    pub fn added(&self) -> Vec<(Input, Output)> {
        self.0.clone()
    }

    pub fn build(&self) -> TransitionFunction {
        TransitionFunction(HashMap::from_iter(self.0.clone().into_iter()))
    }
}
