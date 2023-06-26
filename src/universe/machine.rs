use std::fmt::Display;

use super::function::{Output, TransitionFunction};
use super::Symbol;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct State(Option<usize>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Action {
    L,
    R,
    N,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Write {
    Print(Symbol),
    Erase,
    None,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Machine {
    pub state: State,
    transition_function: TransitionFunction,
}

impl Machine {
    pub fn new(initial_state: State, transition_function: TransitionFunction) -> Self {
        Machine {
            state: initial_state,
            transition_function,
        }
    }

    pub fn tick(&mut self, scanned_symbol: Symbol) -> Result<(Write, Action), String> {
        if self.state.is_halted() {
            return Ok((Write::None, Action::N));
        }

        let Output(print, action, state) =
            self.transition_function.act(self.state, scanned_symbol)?;

        self.state = state;
        Ok((print, action))
    }
}

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Action::L => write!(f, "L"),
            Action::R => write!(f, "R"),
            Action::N => write!(f, "N"),
        }
    }
}

impl Display for Write {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Write::Print(s) => write!(f, "W({s})"),
            Write::Erase => write!(f, "E"),
            Write::None => write!(f, "N"),
        }
    }
}

impl From<Symbol> for Write {
    fn from(symbol: Symbol) -> Self {
        Write::Print(symbol)
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            None => write!(f, "!"),
            Some(symbol) => write!(f, "{symbol}"),
        }
    }
}

impl From<usize> for State {
    fn from(value: usize) -> Self {
        State(Some(value))
    }
}

impl State {
    pub fn halt() -> Self {
        State(None)
    }

    pub fn is_halted(&self) -> bool {
        self.0.is_none()
    }
}

#[cfg(test)]
mod tests {
    use crate::universe::machine::{State, Write};
    use crate::universe::Symbol;

    #[test]
    fn halt_state_is_exclamation() {
        assert_eq!(State::halt().to_string(), String::from("!"));
    }

    #[test]
    fn default_state_is_halt() {
        assert_eq!(State::default(), State::halt());
    }

    #[test]
    fn halt_state_is_halted() {
        assert!(State::halt().is_halted());
    }

    #[test]
    fn print_state() {
        assert_eq!(State::from(42).to_string(), String::from("42"));
    }

    #[test]
    fn print_write_write() {
        assert_eq!(Write::Erase.to_string(), String::from("E"));
        assert_eq!(Write::None.to_string(), String::from("N"));
        assert_eq!(
            Write::Print(Symbol::from(10)).to_string(),
            String::from("W(10)")
        );
    }
}
