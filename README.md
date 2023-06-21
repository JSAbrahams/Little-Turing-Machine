# Turing Machine

Rediscovering my love of computer science.
A fun little interactive Turing machine (with finite tape).

## Requirements

<TODO: list cargo requirements here>

## Example Program

Example program of a 3-state, 2-symbol busy beaver.

```
machine
symbols: 0, 1
states: A, B, C
initial tape: 0000000000
initial state: A
transition function:
  (current state, scanned symbol) -> (print symbol, move tape, next state)
  (A, 0) -> (W(1), R, B)
  (A, 1) -> (W(1), L, C)
  (B, 0) -> (W(1), L, A)
  (B, 1) -> (W(1), R, B)
  (C, 0) -> (W(1), L, B)
  (C, 1) -> (W(1), N, H)

computation
sequence :: instruction :: tape
                       HEAD -> $
       0 ::      A      :: 0000000000
       1 ::      B      :: 0000100000
       2 ::      A      :: 0000110000
       3 ::      C      :: 0000110000
       4 ::      B      :: 0001110000
       5 ::      A      :: 0011110000
       6 ::      B      :: 0111110000
       7 ::      B      :: 0111110000
       8 ::      B      :: 0111110000
       9 ::      B      :: 0111110000
      10 ::      B      :: 0111110000
      11 ::      A      :: 0111111000
      12 ::      C      :: 0111111000
```

To get this output, run `cargo run`
