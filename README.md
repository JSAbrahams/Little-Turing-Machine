# Turing Machine

Rediscovering my love of computer science.
A fun little interactive Turing machine (with finite tape).

## Requirements

Rust, may be installed using [rustup](https://www.rust-lang.org/tools/install).

## Running Program

`cargo run`

Must then type either 3, 4, or 5 to run either 3, 4, or 5-state 2-symbol busy beaver.
Note that the 5-state busy beaver requires 47.176.870 steps to complete!

## Partial Output

Example program of a 3-state, 2-symbol busy beaver.
`!` means halt.

```
machine: 3-state, 2-symbol busy beaver
symbols: _ 1
states: A B C
initial tape:
initial state: A
transition function:
  (current state, scanned symbol) -> (print symbol, move tape, next state)
  (A, _) -> (W(1), R, B)
  (A, 1) -> (W(1), R, !)
  (B, _) -> (W(_), R, C)
  (B, 1) -> (W(1), R, B)
  (C, _) -> (W(1), L, C)
  (C, 1) -> (W(1), L, A)

computation
sequence :: instr :: tape
              HEAD -> $
       0 ::   A   ::
       1 ::   B   :: _1
       2 ::   C   :: _1_
       3 ::   C   :: _1_1
       4 ::   C   :: _111
       5 ::   A   :: _111
       6 ::   B   :: 1111
       7 ::   B   :: 1111
       8 ::   B   :: 1111
       9 ::   B   :: 1111
      10 ::   C   :: 1111_
      11 ::   C   :: 1111_1
      12 ::   C   :: 111111
      13 ::   A   :: 111111
      14 ::   !   :: 111111
```
