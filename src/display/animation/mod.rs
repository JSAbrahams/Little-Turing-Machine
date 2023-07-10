use std::default::Default;
use std::sync::Mutex;
use std::{collections::VecDeque, time::Duration};

use crate::presets::UniverseMetadata;
use crate::universe::function::{Input, Output, TransitionFunctionBuilder};
use crate::universe::machine::{Action, Machine, Write};
use crate::universe::tape::Tape;
use crate::universe::{Symbol, Universe};

use clap::ValueEnum;
use nannou::prelude::*;
use once_cell::sync::OnceCell;

use super::{display_state, DisplayStateAs};

const WINDOW_TITLE: &str = "My Little Turing Machine";

const DEFAULT_TICK_SPEED: Duration = Duration::from_secs(1);
const DISPLAY_TAPE_HALF_WIDTH: usize = 50;

const TRANSITION_FUNCTION_LINE_HEIGHT: f32 = 15_f32;

const CELL_WIDTH: f32 = 20_f32;
const CELL_HEIGHT: f32 = 20_f32;
const CELL_STROKE_WIDTH: f32 = 1_f32;

const TURING_MACHINE_HEIGHT: f32 = 40_f32;
const TURING_MACHINE_WIDTH: f32 = 40_f32;

const TURING_MACHINE_Y_OFFSET: f32 = CELL_HEIGHT + CELL_HEIGHT;
const CELL_Y_OFFSET: f32 = 0.0;
const TRANSITION_FUNCTION_Y_OFFSET: f32 = 1.5 * TURING_MACHINE_HEIGHT + CELL_HEIGHT;

const CELL_OUTLINE_COLOR: nannou::prelude::rgb::Srgb<u8> = STEELBLUE;

const STEPS_Y_OFFSET: f32 = -1.5 * CELL_HEIGHT;

// workaround for nannou API so we can pass model
static MODEL: OnceCell<Mutex<Model>> = OnceCell::new();

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

/// State used for drawing purposes
#[derive(Debug, Clone)]
enum State {
    Moving(Direction),
    Reading,
    Writing(String),
    Erasing,
    Halted,
}

#[allow(dead_code)]
#[derive(Debug, Default, Clone)]
struct Model {
    builder: TransitionFunctionBuilder,
    animation_queue: VecDeque<State>,
    state_as: DisplayStateAs,
    universe: Universe,
    full_screen: bool,
    animate_moving: AnimateMoving,
    show_tick_count: bool,
}

/// When animating, decide whether to move the machine or tape
#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
pub enum AnimateMoving {
    Machine,
    Tape,
}

impl Default for AnimateMoving {
    fn default() -> Self {
        AnimateMoving::Tape
    }
}

impl From<UniverseMetadata> for Model {
    fn from(value: UniverseMetadata) -> Self {
        let mut universe = value.universe;
        if universe.tape.is_empty() {
            universe.pos = 0;
        };

        Model {
            builder: value.transition_function_buidler,
            universe,
            state_as: value.display_state_as,
            ..Default::default()
        }
    }
}

// TODO have some `Options` struct
pub fn animate(
    metadata: UniverseMetadata,
    tick_speed: Option<Duration>,
    full_screen: bool,
    move_item: AnimateMoving,
    show_tick_count: bool,
) {
    set_model(metadata, full_screen, move_item, show_tick_count);

    nannou::app(model)
        .update(update)
        .loop_mode(LoopMode::Rate {
            update_interval: tick_speed.unwrap_or(DEFAULT_TICK_SPEED),
        })
        .run();
}

fn set_model(
    metadata: UniverseMetadata,
    full_screen: bool,
    move_item: AnimateMoving,
    show_tick_count: bool,
) {
    let model_static = MODEL.get_or_init(|| Mutex::new(Model::default()));
    let mut model = Model::from(metadata);
    model.full_screen = full_screen;
    model.animate_moving = move_item;
    model.show_tick_count = show_tick_count;

    *model_static.lock().unwrap() = model;
}

/// Panics if model was never set.
///
/// Workaround because we cannot pass to api as argument, and we cannot capture variables in closures.
fn model(app: &App) -> Model {
    let model = MODEL.get_or_init(|| Mutex::new(Model::default()));
    let model = model.lock().unwrap().clone();

    let view_builder = app.new_window().title(WINDOW_TITLE).view(view);
    let view_builder = if model.full_screen {
        view_builder.fullscreen()
    } else {
        view_builder
    };
    view_builder.build().unwrap();

    model
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let front = model.animation_queue.pop_front();
    if !matches!(front, Some(State::Reading) | None) {
        return;
    }

    let queue = &mut model.animation_queue;
    let (print, action) = model.universe.tick().unwrap();
    queue.push_back(match print {
        Write::Print(x) => State::Writing(format!("{x}")),
        Write::Erase => State::Erasing,
        Write::None => State::Halted,
    });

    match action {
        Action::L => queue.push_back(State::Moving(Direction::Left)),
        Action::R => queue.push_back(State::Moving(Direction::Right)),
        _ => {}
    }

    queue.push_back(State::Reading);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let Some(_state) = model.animation_queue.front() else {
        return;
    };

    let draw = app.draw();
    let universe = &model.universe;

    // reset background
    draw.background().color(BLACK);

    draw_transition_function(&model.builder, &model.state_as, &draw);

    let (offset, pos) = match model.animate_moving {
        AnimateMoving::Tape => (universe.pos, 0),
        AnimateMoving::Machine => (0, universe.pos),
    };

    draw_tape(&universe.tape, universe.pos, offset, &draw);
    draw_machine(&universe.machine, pos, &model.state_as, &draw);

    if model.show_tick_count {
        draw_steps(universe.ticks, 0, &draw);
    }

    draw.to_frame(app, &frame).unwrap();
}

#[allow(dead_code)]
fn draw_debug_info(draw: &Draw, model: &Model) {
    // print debug info
    draw.text(
        format!(
            "head position: {}\ntape: {}\nmachine state: {}\ndraw state: {:?}",
            model.universe.pos,
            model.universe.tape,
            display_state(model.universe.machine.state, &model.state_as),
            model.animation_queue.back()
        )
        .as_str(),
    )
    .left_justify();
}

fn draw_tape(tape: &Tape, pos: isize, offset: isize, draw: &Draw) {
    let draw_range =
        (-(DISPLAY_TAPE_HALF_WIDTH as isize)).min(pos)..(DISPLAY_TAPE_HALF_WIDTH as isize).max(pos);

    for pos in draw_range {
        draw_cell(pos, draw)
    }

    let symbols = tape.second_half();
    for (pos, symbol) in symbols.iter().enumerate() {
        draw_symbol(symbol, pos as isize + 1 - offset, draw);
    }

    let symbols = tape.first_half();
    for (pos, symbol) in symbols.iter().enumerate() {
        draw_symbol(symbol, -(pos as isize) - offset, draw);
    }
}

fn draw_cell(pos: isize, draw: &Draw) {
    draw.rect()
        .stroke_color(CELL_OUTLINE_COLOR)
        .stroke_weight(CELL_STROKE_WIDTH)
        .no_fill()
        .w(CELL_WIDTH)
        .h(CELL_HEIGHT)
        .x_y(CELL_WIDTH * pos as f32, CELL_Y_OFFSET);
}

fn draw_symbol(content: &Symbol, pos: isize, draw: &Draw) {
    let symbol_text = if content.is_empty() {
        String::default()
    } else {
        content.to_string()
    };

    draw.text(&symbol_text)
        .x_y(CELL_WIDTH * pos as f32, CELL_Y_OFFSET)
        .center_justify();
}

fn draw_machine(machine: &Machine, pos: isize, state_as: &DisplayStateAs, draw: &Draw) {
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

fn draw_steps(steps: usize, pos: isize, draw: &Draw) {
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

fn draw_transition_function(
    builder: &TransitionFunctionBuilder,
    state_as: &DisplayStateAs,
    draw: &Draw,
) {
    for (pos, (input, output)) in builder.added().iter().rev().enumerate() {
        draw_function_line(*input, *output, pos, state_as, draw)
    }
}

fn draw_function_line(
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

#[cfg(test)]
mod tests {
    use crate::presets::busy_beaver::three_state_busy_beaver;

    use super::Model;

    #[test]
    fn model_from_beaver_3_universe_meta() {
        let beaver = three_state_busy_beaver();
        let model = Model::from(three_state_busy_beaver());

        assert!(model.animation_queue.is_empty());
        assert_eq!(
            model.builder.build(),
            beaver.transition_function_buidler.build()
        );

        assert_eq!(model.universe.tape, beaver.universe.tape);
        assert_eq!(model.universe.machine, beaver.universe.machine);
    }
}
