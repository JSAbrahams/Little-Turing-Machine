use std::collections::HashMap;

use super::machine::{Action, State, Write};
use super::Symbol;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Input(pub State, pub Symbol);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Output(pub Write, pub Action, pub State);

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TransitionFunction(HashMap<Input, Output>);

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct TransitionFunctionBuilder(Vec<(Input, Output)>);

impl TransitionFunction {
    pub fn act(&self, current_state: State, scanned_symbol: Symbol) -> Result<Output, String> {
        self.0
            .get(&Input(current_state, scanned_symbol))
            .cloned()
            .ok_or_else(|| format!("{current_state}, {scanned_symbol} -> ?"))
    }
}

impl TransitionFunctionBuilder {
    pub fn add(
        &mut self,
        current_state: State,
        input_symbol: Symbol,
        write_action: Write,
        move_head: Action,
        next_state: State,
    ) {
        self.0.push((
            Input(current_state, input_symbol),
            Output(write_action, move_head, next_state),
        ));
    }

    pub fn added(&self) -> Vec<(Input, Output)> {
        self.0.clone()
    }

    pub fn build(&self) -> TransitionFunction {
        TransitionFunction(HashMap::from_iter(self.0.clone().into_iter()))
    }
}

impl Input {
    pub fn current_state(&self) -> State {
        self.0
    }

    pub fn scanned_symbol(&self) -> Symbol {
        self.1
    }
}

impl Output {
    pub fn print_symbol(&self) -> Write {
        self.0
    }

    pub fn move_head(&self) -> Action {
        self.1
    }

    pub fn next_state(&self) -> State {
        self.2
    }
}
