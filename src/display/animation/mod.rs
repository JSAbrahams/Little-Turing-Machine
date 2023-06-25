use std::sync::Mutex;
use std::{collections::VecDeque, time::Duration};

use crate::presets::UniverseMetadata;
use crate::{Action, Tape, Universe};

use nannou::prelude::*;
use once_cell::sync::OnceCell;

use super::{display_state, DisplayStateAs};

const DEFAULT_TICK_SPEED: Duration = Duration::from_secs(1);

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

    draw.to_frame(app, &frame).unwrap();
}

#[allow(dead_code)]
fn draw_tape(_tape: &Tape, _head_position: usize, _draw: &Draw, _frame: &Frame) {
    todo!()
}

#[allow(dead_code)]
fn draw_cell(_content: &str, _position: usize, _draw: &Draw, _frame: &Frame) {
    todo!()
}

#[allow(dead_code)]
fn draw_machine(_state: &State, _draw: &Draw, _frame: &Frame) {
    todo!()
}
