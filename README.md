# Little Turing Machine

A fun little interactive (eventually) Turing machine (with finite tape, of course).

For now, we only allow loading of presets.
These presets consider of a 2, 3, 4, and 5-state 2-symbol busy beavers.

## Requirements

Rust, may be installed using [rustup](https://www.rust-lang.org/tools/install).
The above is the reocmmended approach for installing Rust.

## Running Program

### Print to CLI

```
Usage: cargo run cli [OPTIONS]

Options:
  -p, --preset <PRESET>
  -h, --help             Print help
```

### Animate

```
Usage: cargo run animate [OPTIONS]

Options:
  -p, --preset <PRESET>
  -f, --full-screen
      --animate-moving <ANIMATE_MOVING>  [default: tape] [possible values: machine, tape]
      --show-tick-count
  -h, --help                             Print help
  ```

The current preset are:

Preset | Description
---|---
"beaver_2" | A 2-state 2-symbol busy beaver
"beaver_3" | A 3-state 2-symbol busy beaver
"beaver_4" | A 4-state 2-symbol busy beaver
"beaver_5" | A 5-state 2-symbol busy beaver

## Example Output

To the the following: `cargo run cli -p beaver_3`

Example program of a 3-state, 2-symbol busy beaver.
`!` means halt.

```
machine: 3-state, 2-symbol busy beaver
symbols: _ 1
states: A B C
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
sequence :: instr :: HEAD :: tape
       0 ::   A   ::  1   ::
       1 ::   B   ::  2   :: _1
       2 ::   C   ::  3   :: _1_
       3 ::   C   ::  2   :: _1_1
       4 ::   C   ::  1   :: _111
       5 ::   A   ::  0   :: _111
       6 ::   B   ::  1   :: 1111
       7 ::   B   ::  2   :: 1111
       8 ::   B   ::  3   :: 1111
       9 ::   B   ::  4   :: 1111
      10 ::   C   ::  5   :: 1111_
      11 ::   C   ::  4   :: 1111_1
      12 ::   C   ::  3   :: 111111
      13 ::   A   ::  2   :: 111111
      14 ::   !   ::  3   :: 111111
```
