use std::fmt::Display;

use self::function::TransitionFunction;
use self::machine::{Action, Machine, State, Write};
use self::tape::Tape;

pub mod function;
pub mod machine;
pub mod tape;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Symbol(Option<usize>);

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Universe {
    pub tape: Tape,
    pub pos: isize,
    pub machine: Machine,
}

impl Universe {
    pub fn new<T: IntoIterator<Item = Symbol>>(
        initial_tape: T,
        initial_pos: usize,
        initial_state: State,
        transition_function: TransitionFunction,
    ) -> Self {
        Universe {
            tape: Tape::from_iter(initial_tape),
            pos: initial_pos as isize,
            machine: Machine::new(initial_state, transition_function),
        }
    }

    fn shift(&mut self, action: Action) {
        match action {
            Action::L => self.pos -= 1,
            Action::R => self.pos += 1,
            Action::N => {}
        }
    }

    pub fn tick(&mut self) -> Result<(Write, Action), String> {
        let scanned_symbol = self.tape.read(self.pos);

        let (print, action) = self.machine.tick(scanned_symbol)?;

        self.tape.write(print, self.pos);
        self.shift(action);

        Ok((print, action))
    }
}

impl From<usize> for Symbol {
    fn from(value: usize) -> Self {
        Symbol(Some(value))
    }
}

impl Symbol {
    pub fn empty() -> Self {
        Symbol(None)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_none()
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            None => write!(f, "_"),
            Some(symbol) => write!(f, "{symbol}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::universe::Symbol;

    #[test]
    fn empty_symbol_is_empty() {
        assert!(Symbol::empty().is_empty())
    }

    #[test]
    fn empty_symbol_is_underscore() {
        assert_eq!(Symbol::empty().to_string(), String::from("_"));
    }

    #[test]
    fn symbol_to_string_is_number() {
        assert_eq!(Symbol::from(4).to_string(), String::from("4"))
    }
}
