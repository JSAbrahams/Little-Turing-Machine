use crate::{presets::UniverseMetaData, Universe};

use nannou::prelude::*;

/// State used for drawing purposes
#[allow(dead_code)]
enum State {
    Moving,
    Reading,
    Writing,
}

#[allow(dead_code)]
struct Model {
    state: State,
    universe: Universe,
}

pub fn animate(_packed_universe: UniverseMetaData) {
    nannou::app(model).update(update).simple_window(view).run();
}

fn model(_app: &App) -> Model {
    todo!();
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    todo!();
}

fn view(_app: &App, _model: &Model, _frame: Frame) {
    todo!();
}
