use std::collections::{HashMap, VecDeque};

use crate::{presets::UniverseMetaData, Action, Symbol, Universe};

use nannou::prelude::*;

enum Direction {
    Left,
    Right,
}

/// State used for drawing purposes
#[allow(dead_code)]
enum State {
    Moving(Direction),
    Reading,
    Writing(String),
    Erasing,
}

#[allow(dead_code)]
struct Model {
    animation_queue: VecDeque<State>,
    display_symbol_as: HashMap<Symbol, String>,
    universe: Universe,
}

pub fn animate(_packed_universe: UniverseMetaData) {
    nannou::app(model).update(update).simple_window(view).run();
}

fn model(_app: &App) -> Model {
    todo!();
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    if !matches!(
        model.animation_queue.pop_front(),
        Some(State::Reading) | None
    ) {
        return;
    }

    let (print, action) = model.universe.tick().unwrap();
    match print {
        crate::Write::Print(x) => model
            .animation_queue
            .push_back(State::Writing(format!("{x}"))),
        crate::Write::Erase => model.animation_queue.push_back(State::Erasing),
        crate::Write::None => todo!(),
    }

    match action {
        Action::L => model
            .animation_queue
            .push_back(State::Moving(Direction::Left)),
        Action::R => model
            .animation_queue
            .push_back(State::Moving(Direction::Right)),
        _ => {}
    }
}

fn view(_app: &App, model: &Model, _frame: Frame) {
    let Some(_state) = model.animation_queue.front() else {
        return;
    };
}
