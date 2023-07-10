use nannou::Draw;

use crate::display::{display_state, DisplayStateAs};
use crate::universe::function::{Input, Output, TransitionFunctionBuilder};
use crate::universe::machine::Machine;

use super::{
    CELL_HEIGHT, CELL_OUTLINE_COLOR, CELL_STROKE_WIDTH, CELL_WIDTH, CELL_Y_OFFSET, STEPS_Y_OFFSET,
    TRANSITION_FUNCTION_LINE_HEIGHT, TRANSITION_FUNCTION_Y_OFFSET, TURING_MACHINE_HEIGHT,
    TURING_MACHINE_WIDTH, TURING_MACHINE_Y_OFFSET,
};

pub fn draw_machine(machine: &Machine, pos: isize, state_as: &DisplayStateAs, draw: &Draw) {
    let position = CELL_WIDTH * (pos - 1) as f32;

    // whole machine
    draw.rect()
        .stroke_color(CELL_OUTLINE_COLOR)
        .stroke_weight(CELL_STROKE_WIDTH)
        .no_fill()
        .w(TURING_MACHINE_WIDTH)
        .h(TURING_MACHINE_HEIGHT)
        .x_y(position, TURING_MACHINE_Y_OFFSET);

    // pointer
    draw.rect()
        .stroke_color(CELL_OUTLINE_COLOR)
        .stroke_weight(CELL_STROKE_WIDTH)
        .no_fill()
        .w(CELL_WIDTH + CELL_WIDTH / 4.0)
        .h(CELL_HEIGHT + CELL_HEIGHT / 4.0)
        .x_y(position, CELL_Y_OFFSET);

    // state
    let state = display_state(machine.state, state_as);
    draw.text(state.as_str())
        .x_y(position, TURING_MACHINE_Y_OFFSET)
        .center_justify();
}

pub fn draw_steps(steps: usize, pos: isize, draw: &Draw) {
    let position = CELL_WIDTH * (pos - 1) as f32;

    draw.rect()
        .stroke_color(CELL_OUTLINE_COLOR)
        .stroke_weight(CELL_STROKE_WIDTH)
        .no_fill()
        .w(TURING_MACHINE_WIDTH)
        .h(TURING_MACHINE_HEIGHT / 2.0)
        .x_y(position, STEPS_Y_OFFSET);

    draw.text(steps.to_string().as_str())
        .center_justify()
        .align_text_middle_y()
        .w(TURING_MACHINE_WIDTH)
        .h(TURING_MACHINE_HEIGHT / 2.0)
        .x_y(position, STEPS_Y_OFFSET);
}

pub fn draw_transition_function(
    builder: &TransitionFunctionBuilder,
    state_as: &DisplayStateAs,
    draw: &Draw,
) {
    for (pos, (input, output)) in builder.added().iter().rev().enumerate() {
        draw_function_line(*input, *output, pos, state_as, draw)
    }
}

pub fn draw_function_line(
    input: Input,
    output: Output,
    pos: usize,
    state_as: &DisplayStateAs,
    draw: &Draw,
) {
    let (state, symbol) = (display_state(input.state, state_as), input.symbol);
    let (write, action, o_state) = (
        output.write,
        output.action,
        display_state(output.state, state_as),
    );

    draw.text(format!("{state}, {symbol} -> {write}, {action}, {o_state}").as_str())
        .x_y(
            0_f32,
            TRANSITION_FUNCTION_Y_OFFSET + (TRANSITION_FUNCTION_LINE_HEIGHT * pos as f32),
        )
        .left_justify();
}
