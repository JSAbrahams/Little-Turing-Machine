use std::collections::HashMap;
use std::fmt::Display;
use std::iter::FromIterator;

pub mod presets;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct State(Option<usize>);

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            None => write!(f, "!"),
            Some(symbol) => write!(f, "{}", symbol),
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Symbol(Option<usize>);

impl From<usize> for Symbol {
    fn from(value: usize) -> Self {
        Symbol(Some(value))
    }
}

impl Symbol {
    pub fn empty() -> Self {
        Symbol(None)
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            None => write!(f, "_"),
            Some(symbol) => write!(f, "{}", symbol),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Action {
    L,
    R,
    N,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Write {
    Print(Symbol),
    Erase,
    None,
}

impl Display for Write {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Write::Print(s) => write!(f, "W({})", s),
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

pub struct TransitionFunction(HashMap<Input, Output>);

impl TransitionFunction {
    pub fn act(&self, current_state: State, scanned_symbol: Symbol) -> Result<Output, String> {
        self.0
            .get(&Input(current_state, scanned_symbol))
            .cloned()
            .ok_or_else(|| format!("{current_state}, {scanned_symbol} -> ?"))
    }
}

#[derive(Default)]
pub struct TransitionFunctionBuilder(Vec<(Input, Output)>);

impl TransitionFunctionBuilder {
    pub fn add(
        &mut self,
        current_state: State,
        input_symbol: Symbol,
        write_action: Write,
        move_tape: Action,
        next_state: State,
    ) {
        self.0.push((
            Input(current_state, input_symbol),
            Output(write_action, move_tape, next_state),
        ));
    }

    pub fn added(&self) -> Vec<(Input, Output)> {
        self.0.clone()
    }

    pub fn build(&self) -> TransitionFunction {
        TransitionFunction(HashMap::from_iter(self.0.clone().into_iter()))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Input(State, Symbol);

impl Input {
    pub fn current_state(&self) -> State {
        self.0
    }

    pub fn scanned_symbol(&self) -> Symbol {
        self.1
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Output(Write, Action, State);

impl Output {
    pub fn print_symbol(&self) -> Write {
        self.0
    }

    pub fn move_tape(&self) -> Action {
        self.1
    }

    pub fn next_state(&self) -> State {
        self.2
    }
}

pub struct Tape(Vec<Symbol>);

impl Display for Tape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0.iter().map(|i| format!("{i}")).collect::<String>()
        )
    }
}

impl FromIterator<Symbol> for Tape {
    fn from_iter<T: IntoIterator<Item = Symbol>>(symbols: T) -> Self {
        Tape(Vec::from_iter(symbols))
    }
}

impl Tape {
    pub fn read(&self, position: usize) -> Symbol {
        self.0.get(position).cloned().unwrap_or_else(Symbol::empty)
    }

    pub fn write(&mut self, write: Write, position: usize) {
        match write {
            Write::Print(symbol) if position >= self.0.len() => {
                let pad_length = position - self.0.len() + 1;
                self.0.append(&mut vec![Symbol::empty(); pad_length]);
                self.0[position] = symbol;
            }
            Write::Print(symbol) => self.0[position] = symbol,
            Write::Erase if position < self.0.len() => self.0[position] = Symbol::empty(),
            _ => {}
        }
    }

    // Shift everything to the right by one and prepend with empty symbol.
    pub fn shift(&mut self) {
        self.0.push(Symbol::empty());
        for i in (1..self.0.len()).rev() {
            self.0[i] = self.0[i - 1];
        }
        self.0[0] = Symbol::empty();
    }

    pub fn read_to(&self, len: usize) -> Vec<Symbol> {
        let pad_length = len.saturating_sub(self.0.len());

        let mut values = self.0.clone();
        values.append(&mut vec![Symbol::empty(); pad_length]);
        values
    }

    pub fn print_to(&self, len: usize) -> String {
        let symbols = self.read_to(len);
        symbols.iter().map(|i| format!("{i}")).collect::<String>()
    }
}

pub struct Machine {
    pub state: State,
    transition_function: TransitionFunction,
}

impl Machine {
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

impl Machine {
    pub fn new(initial_state: State, transition_function: TransitionFunction) -> Self {
        Machine {
            state: initial_state,
            transition_function,
        }
    }
}

pub struct Universe {
    pub tape: Tape,
    pub head: usize,
    pub machine: Machine,
}

impl Universe {
    pub fn new<T: IntoIterator<Item = Symbol>>(
        initial_tape: T,
        initial_head: usize,
        initial_state: State,
        transition_function: TransitionFunction,
    ) -> Self {
        Universe {
            tape: Tape::from_iter(initial_tape),
            head: initial_head,
            machine: Machine::new(initial_state, transition_function),
        }
    }

    fn shift(&mut self, action: Action) {
        match action {
            Action::L => self.head -= 1,
            Action::R => self.head += 1,
            Action::N => {}
        }
    }

    pub fn tick(&mut self) -> Result<(), String> {
        let scanned_symbol = self.tape.read(self.head);

        let (print, action) = self.machine.tick(scanned_symbol)?;

        self.tape.write(print, self.head);

        if self.head == 0 && action == Action::L {
            self.tape.shift();
        } else {
            self.shift(action);
        }

        Ok(())
    }
}
