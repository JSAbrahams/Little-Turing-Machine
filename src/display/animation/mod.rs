use std::sync::Mutex;
use std::{collections::VecDeque, time::Duration};

use crate::presets::UniverseMetadata;
use crate::{Action, Machine, Symbol, Tape, Universe};

use nannou::prelude::*;
use once_cell::sync::OnceCell;

use super::{display_state, DisplayStateAs};

const DEFAULT_TICK_SPEED: Duration = Duration::from_secs(1);
const DISPLAY_TAPE_HALF_WIDTH: usize = 25;

const CELL_WIDTH: f32 = 20_f32;
const CELL_HEIGHT: f32 = 20_f32;
const CELL_X_OFFSET: f32 = 0_f32;
const CELL_STROKE_WIDTH: f32 = 1_f32;

const TURING_MACHINE_HEIGHT: f32 = 40_f32;
const TURING_MACHINE_WIDTH: f32 = 40_f32;
const TURING_MACHINE_X_OFFSET: f32 = CELL_WIDTH * DISPLAY_TAPE_HALF_WIDTH as f32 + CELL_X_OFFSET;
const TURING_MACHINE_Y_OFFSET: f32 = 0_f32;

const CELL_Y_OFFSET: f32 = TURING_MACHINE_Y_OFFSET + TURING_MACHINE_HEIGHT + 10_f32;

const CELL_OUTLINE_COLOR: nannou::prelude::rgb::Srgb<u8> = STEELBLUE;

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
    animation_queue: VecDeque<State>,
    state_as: DisplayStateAs,
    universe: Universe,
}

impl From<UniverseMetadata> for Model {
    fn from(value: UniverseMetadata) -> Self {
        Model {
            universe: value.universe,
            state_as: value.display_state_as,
            ..Default::default()
        }
    }
}

pub fn animate(metadata: UniverseMetadata, tick_speed: Option<Duration>) {
    set_model(metadata);

    nannou::app(model)
        .update(update)
        .loop_mode(LoopMode::Rate {
            update_interval: tick_speed.unwrap_or(DEFAULT_TICK_SPEED),
        })
        .simple_window(view)
        .run();
}

fn set_model(metadata: UniverseMetadata) {
    let model = MODEL.get_or_init(|| Mutex::new(Model::default()));
    *model.lock().unwrap() = Model::from(metadata);
}

/// Panics if model was never set.
///
/// Workaround because we cannot pass to api as argument, and we cannot capture variables in closures.
fn model(_app: &App) -> Model {
    let model = MODEL.get_or_init(|| Mutex::new(Model::default()));
    model.lock().unwrap().clone()
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let front = model.animation_queue.pop_front();
    if !matches!(front, Some(State::Reading) | None) {
        return;
    }

    let queue = &mut model.animation_queue;
    let (print, action) = model.universe.tick().unwrap();
    queue.push_back(match print {
        crate::Write::Print(x) => State::Writing(format!("{x}")),
        crate::Write::Erase => State::Erasing,
        crate::Write::None => State::Halted,
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
    // reset background
    draw.background().color(BLACK);

    draw_tape(&model.universe.tape, model.universe.head, &draw);
    draw_machine(
        &model.universe.machine,
        model.universe.head,
        &model.state_as,
        &draw,
    );

    draw.to_frame(app, &frame).unwrap();
}

#[allow(dead_code)]
fn draw_debug_info(draw: &Draw, model: &Model) {
    // print debug info
    let head_symbol = format!(
        "{}{} [{}]",
        "_".repeat(6 - 1 + model.universe.head),
        "$",
        display_state(model.universe.machine.state, &model.state_as)
    );
    draw.text(
        format!(
            "head position: {}\n{}\ntape: {}\nmachine state: {}\ndraw state: {:?}",
            model.universe.head,
            head_symbol,
            model.universe.tape,
            display_state(model.universe.machine.state, &model.state_as),
            model.animation_queue.back()
        )
        .as_str(),
    )
    .left_justify();
}

fn draw_tape(tape: &Tape, head_position: usize, draw: &Draw) {
    let symbol_range = head_position.saturating_sub(DISPLAY_TAPE_HALF_WIDTH)
        ..head_position.saturating_add(DISPLAY_TAPE_HALF_WIDTH);

    let symbols = tape.symbols(symbol_range);
    for (i, symbol) in symbols.iter().enumerate() {
        draw_cell(symbol, i, draw);
    }
}

#[allow(dead_code)]
fn draw_cell(content: &Symbol, position: usize, draw: &Draw) {
    draw.rect()
        .stroke_color(CELL_OUTLINE_COLOR)
        .stroke_weight(CELL_STROKE_WIDTH)
        .no_fill()
        .w(CELL_WIDTH)
        .h(CELL_HEIGHT)
        .x_y(CELL_X_OFFSET + CELL_WIDTH * position as f32, CELL_Y_OFFSET);

    let symbol_text = if content.is_empty() {
        String::default()
    } else {
        content.to_string()
    };

    draw.text(&symbol_text)
        .x_y(CELL_X_OFFSET + CELL_WIDTH * position as f32, CELL_Y_OFFSET)
        .center_justify();
}

fn draw_machine(machine: &Machine, position: usize, state_as: &DisplayStateAs, draw: &Draw) {
    let position = CELL_WIDTH * position as f32;
    let tape_start = TURING_MACHINE_X_OFFSET - DISPLAY_TAPE_HALF_WIDTH as f32 * CELL_WIDTH;

    // whole machine
    draw.rect()
        .stroke_color(CELL_OUTLINE_COLOR)
        .stroke_weight(CELL_STROKE_WIDTH)
        .no_fill()
        .w(TURING_MACHINE_WIDTH)
        .h(TURING_MACHINE_HEIGHT)
        .x_y(tape_start + position, -TURING_MACHINE_Y_OFFSET);

    // pointer
    let pointer_height = TURING_MACHINE_HEIGHT / 4.0;
    draw.rect()
        .stroke_color(CELL_OUTLINE_COLOR)
        .stroke_weight(CELL_STROKE_WIDTH)
        .no_fill()
        .w(CELL_WIDTH)
        .h(TURING_MACHINE_HEIGHT / 4.0)
        .x_y(
            tape_start + position,
            -TURING_MACHINE_Y_OFFSET + TURING_MACHINE_HEIGHT - pointer_height,
        );

    // state
    let state = display_state(machine.state, state_as);
    draw.text(state.as_str())
        .x_y(tape_start + position, -TURING_MACHINE_Y_OFFSET)
        .center_justify();
}
