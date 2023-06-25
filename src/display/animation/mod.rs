use std::{collections::VecDeque, time::Duration};

use crate::presets::busy_beaver::three_state_busy_beaver;
use crate::presets::UniverseMetaData;
use crate::{Action, Tape, Universe};

use nannou::prelude::*;

use super::{display_state, DisplayStateAs};

const DEFAULT_TICK_SPEED: Duration = Duration::from_secs(1);

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

/// State used for drawing purposes
#[derive(Debug)]
enum State {
    Moving(Direction),
    Reading,
    Writing(String),
    Erasing,
    Halted,
}

#[allow(dead_code)]
struct Model {
    animation_queue: VecDeque<State>,
    state_as: DisplayStateAs,
    universe: Universe,
}

pub fn animate(_packed_universe: UniverseMetaData, tick_speed: Option<Duration>) {
    nannou::app(model)
        .update(update)
        .loop_mode(LoopMode::Rate {
            update_interval: tick_speed.unwrap_or(DEFAULT_TICK_SPEED),
        })
        .simple_window(view)
        .run();
}

fn model(_app: &App) -> Model {
    // TODO: figure out how to pass universe metadata using retrictive interface
    let universe_metadata = three_state_busy_beaver();

    Model {
        animation_queue: VecDeque::new(),
        state_as: universe_metadata.display_state_as,
        universe: universe_metadata.universe,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    if !matches!(
        model.animation_queue.pop_front(),
        Some(State::Reading) | None
    ) {
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
    draw.text(
        format!(
            "head position: {}\ntape: {}\nmachine state: {}\ndraw state: {:?}",
            model.universe.head,
            model.universe.tape,
            display_state(model.universe.machine.state, &model.state_as),
            model.animation_queue.back()
        )
        .as_str(),
    );

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
