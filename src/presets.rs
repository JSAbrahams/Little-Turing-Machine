#[cfg(test)]
mod tests {
    use crate::Action::*;
    use crate::{State, Symbol, TransitionFunctionBuilder, Universe, Write};

    #[test]
    #[ignore = "beaver does not behave"]
    pub fn verify_state_busy_beaver() {
        let tape_len = 10_usize;
        let initial_head = 4_usize;

        let s0 = Symbol::from(0);
        let s1 = Symbol::from(1);

        let sA = State::from(0);
        let sB = State::from(1);
        let sC = State::from(2);
        let sH = State::halt();

        let mut builder = TransitionFunctionBuilder::default();
        builder.add(sA, s0, Write::from(s1), R, sB);
        builder.add(sA, s1, Write::from(s1), L, sC);
        builder.add(sB, s0, Write::from(s1), L, sA);
        builder.add(sB, s1, Write::from(s1), R, sB);
        builder.add(sC, s0, Write::from(s1), L, sB);
        builder.add(sC, s1, Write::from(s1), N, sH);

        let transition_function = builder.build();
        let mut u = Universe::new(vec![s0; tape_len], initial_head, sA, transition_function);

        let mut sequence = 1;
        macro_rules! step_verify {
            ($state:expr, $($symbols:tt),*) => {
                let a = u.machine.state == $state;

                let u_tape = u.tape.read_to(tape_len);
                let exp_tape = vec![$($symbols),*];
                let b = u_tape == exp_tape;

                if !a || !b {
                    let left = format!("{sequence} : {} : {}", u.machine.state, u_tape.iter().map(|i| format!("{i}")).collect::<String>());
                    let right = format!("{sequence} : {} : {}", $state, exp_tape.iter().map(|i| format!("{i}")).collect::<String>());
                    assert_eq!(left, right);
                }

                u.tick().unwrap();
                sequence += 1;
            };
        }

        //////////////////////////////// HEAD
        step_verify!(sA, s0, s0, s0, s0, s0, s0, s0, s0, s0, s0);
        step_verify!(sB, s0, s0, s0, s0, s0, s1, s0, s0, s0, s0);
        step_verify!(sA, s0, s0, s0, s1, s1, s0, s0, s0, s0, s0);
        step_verify!(sC, s0, s0, s1, s1, s0, s0, s0, s0, s0, s0);
        step_verify!(sB, s0, s1, s1, s1, s0, s0, s0, s0, s0, s0);
        step_verify!(sA, s1, s1, s1, s1, s0, s0, s0, s0, s0, s0);
        step_verify!(sB, s0, s1, s1, s1, s1, s1, s0, s0, s0, s0);
        step_verify!(sB, s0, s0, s1, s1, s1, s1, s1, s0, s0, s0);
        step_verify!(sB, s0, s0, s0, s1, s1, s1, s1, s1, s0, s0);
        step_verify!(sB, s0, s0, s0, s0, s1, s1, s1, s1, s1, s0);
        step_verify!(sB, s0, s0, s0, s0, s0, s1, s1, s1, s1, s1);
        step_verify!(sA, s0, s0, s0, s1, s1, s1, s1, s1, s1, s0);
        step_verify!(sC, s0, s0, s1, s1, s1, s1, s1, s1, s0, s0);
        step_verify!(sH, s0, s0, s0, s1, s1, s1, s1, s1, s1, s0);
    }
}
