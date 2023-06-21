# Turing Machine

Rediscovering my love of computer science.
A fun little interactive Turing machine (with finite tape).

## Requirements

Rust, may be installed using [rustup](https://www.rust-lang.org/tools/install).

## Example Program

Example program of a 3-state, 2-symbol busy beaver.

```
machine
symbols: _, 1
states: A, B, C
initial tape:
initial state: A
transition function:
  (current state, scanned symbol) -> (print symbol, move tape, next state)
  (A, _) -> (W(1), R, B)
  (A, 1) -> (W(1), L, C)
  (B, _) -> (W(1), L, A)
  (B, 1) -> (W(1), R, B)
  (C, _) -> (W(1), L, B)
  (C, 1) -> (W(1), N, !)

computation
sequence :: instruction :: tape
                       HEAD -> $
       0 ::      A      ::
       1 ::      B      :: ____1
       2 ::      A      :: ____11
       3 ::      C      :: ____11
       4 ::      B      :: ___111
       5 ::      A      :: __1111
       6 ::      B      :: _11111
       7 ::      B      :: _11111
       8 ::      B      :: _11111
       9 ::      B      :: _11111
      10 ::      B      :: _11111
      11 ::      A      :: _111111
      12 ::      C      :: _111111
```

To get this output, run `cargo run`
