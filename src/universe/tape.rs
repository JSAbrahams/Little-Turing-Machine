use std::fmt::Display;
use std::ops::Range;

use super::machine::Write;
use super::Symbol;

#[derive(Debug, Clone, Eq, Default)]
pub struct Tape {
    positive: Vec<Symbol>,
    negative: Vec<Symbol>,
}

impl PartialEq for Tape {
    /// Two tapes are equal iff the order in which the symbols are written is equivalent, regardless of their actual index.
    ///
    /// Empty symbols at the end are trimmed.
    fn eq(&self, other: &Self) -> bool {
        let trim = |symbols: Vec<Symbol>| {
            let from = symbols.iter().position(|s| !s.is_empty()).unwrap_or(0);
            let to = symbols
                .iter()
                .rposition(|s| !s.is_empty())
                .unwrap_or_else(|| symbols.len());
            symbols[from..to].to_vec()
        };

        trim(self.all_symbols()) == trim(other.all_symbols())
    }
}

impl Display for Tape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.all_symbols()
                .iter()
                .map(|i| i.to_string())
                .collect::<String>()
        )
    }
}

impl FromIterator<Symbol> for Tape {
    fn from_iter<T: IntoIterator<Item = Symbol>>(symbols: T) -> Self {
        Tape {
            positive: Vec::from_iter(symbols),
            ..Default::default()
        }
    }
}

impl Tape {
    pub fn read(&self, pos: isize) -> Symbol {
        if pos.is_positive() {
            self.positive.get(pos as usize - 1)
        } else {
            self.negative.get(pos.unsigned_abs())
        }
        .cloned()
        .unwrap_or_else(Symbol::empty)
    }

    pub fn write(&mut self, write: Write, pos: isize) {
        let (tape_half, index) = if pos.is_positive() {
            (&mut self.positive, pos as usize - 1)
        } else {
            (&mut self.negative, pos.unsigned_abs())
        };

        match write {
            Write::Print(symbol) => {
                let append_len = (index + 1).saturating_sub(tape_half.len());
                tape_half.append(&mut vec![Symbol::default(); append_len]);
                tape_half[index] = symbol;
            }
            Write::Erase if index < tape_half.len() => {
                tape_half.remove(index);
            }
            _ => {}
        }
    }

    pub fn all_symbols(&self) -> Vec<Symbol> {
        if self.positive.is_empty() && self.negative.is_empty() {
            return Vec::default();
        }

        let mut negative: Vec<_> = if self.negative.is_empty() {
            vec![Symbol::empty()]
        } else {
            self.negative.iter().rev().cloned().collect()
        };

        negative.append(&mut self.positive.clone());
        negative
    }

    pub fn symbols(&self, range: Range<isize>) -> Vec<Symbol> {
        if self.positive.is_empty() && self.negative.is_empty() {
            return Vec::default();
        }

        let neg_range = 0..range.start.min(0).unsigned_abs().min(self.negative.len());
        let pos_range = 0..range.end.max(0).unsigned_abs().min(self.positive.len());

        let mut symbols: Vec<_> = if self.negative.is_empty() {
            vec![Symbol::empty()]
        } else {
            self.negative[neg_range].iter().rev().cloned().collect()
        };

        symbols.append(&mut self.positive[pos_range].to_vec());
        symbols
    }
}

#[cfg(test)]
mod tests {
    use crate::universe::machine::Write;
    use crate::universe::Symbol;
    use test_case::test_case;

    use super::Tape;

    #[test]
    fn empty_symbol_is_default() {
        assert_eq!(Symbol::empty(), Symbol::default());
    }

    #[test_case(0)]
    #[test_case(1)]
    #[test_case(2)]
    fn read_empty_tape(index: isize) {
        let tape = Tape::default();

        assert_eq!(tape.read(index), Symbol::empty());
    }

    #[test_case(-2)]
    #[test_case(-1)]
    fn read_empty_tape_negative(index: isize) {
        let tape = Tape::default();

        assert_eq!(tape.read(index), Symbol::empty());
    }

    #[test]
    fn test_multiple_symbols() {
        let expected_tape = Tape::from_iter([
            Symbol::empty(),
            Symbol::from(5),
            Symbol::from(6),
            Symbol::from(4),
        ]);
        let mut tape = Tape::default();

        tape.write(Write::Print(Symbol::from(4)), 3);
        tape.write(Write::Print(Symbol::from(5)), 1);
        tape.write(Write::Print(Symbol::from(6)), 2);

        assert_eq!(tape, expected_tape);
    }

    #[test]
    fn multiple_symbol_negative() {
        let mut tape = Tape::default();
        tape.write(Write::Print(Symbol::from(4)), -3);
        tape.write(Write::Print(Symbol::from(5)), -1);
        tape.write(Write::Print(Symbol::from(6)), -2);

        assert_eq!(tape.to_string(), String::from("465_"));
    }

    #[test]
    fn multiple_symbols_edge() {
        let mut tape = Tape::default();
        tape.write(Write::Print(Symbol::from(4)), 1);
        tape.write(Write::Print(Symbol::from(5)), -1);
        tape.write(Write::Print(Symbol::from(6)), 0);

        assert_eq!(tape.to_string(), String::from("564"));
    }

    #[test]
    fn multiple_symbol_positive() {
        let mut tape = Tape::default();
        tape.write(Write::Print(Symbol::from(4)), 3);
        tape.write(Write::Print(Symbol::from(5)), 1);
        tape.write(Write::Print(Symbol::from(6)), 2);

        assert_eq!(tape.to_string(), String::from("_564"));
    }

    #[test]
    fn test_different_offsets_equal() {
        let mut tape1 = Tape::default();
        tape1.write(Write::Print(Symbol::from(4)), 3);
        tape1.write(Write::Print(Symbol::from(5)), 1);
        tape1.write(Write::Print(Symbol::from(6)), 2);

        let mut tape2 = Tape::default();
        tape2.write(Write::Print(Symbol::from(4)), 1);
        tape2.write(Write::Print(Symbol::from(5)), -1);
        tape2.write(Write::Print(Symbol::from(6)), 0);

        assert_eq!(tape1, tape2);
    }

    #[test_case(0, "3")]
    #[test_case(1, "_3")]
    #[test_case(8, "________3")]
    fn write_to_empty_tape_positive(index: isize, expected: &str) {
        let mut tape = Tape::default();
        tape.write(Write::Print(Symbol::from(3)), index);

        assert_eq!(tape.to_string(), expected.to_string());
    }

    #[test_case(-1, "3_")]
    fn write_to_empty_tape_negative(index: isize, expected: &str) {
        let mut tape = Tape::default();
        tape.write(Write::Print(Symbol::from(3)), index);

        assert_eq!(tape.to_string(), expected.to_string());
    }

    #[test_case(0)]
    #[test_case(-1)]
    #[test_case(-2)]
    #[test_case(3)]
    fn read_write_to_empty_tape(index: isize) {
        let mut tape = Tape::default();
        tape.write(Write::Print(Symbol::from(3)), index);

        assert_eq!(tape.read(index), Symbol::from(3));
    }
}
