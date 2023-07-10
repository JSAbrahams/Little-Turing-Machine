use nannou::Draw;

use crate::universe::{tape::Tape, Symbol};

use super::{
    CELL_HEIGHT, CELL_OUTLINE_COLOR, CELL_STROKE_WIDTH, CELL_WIDTH, CELL_Y_OFFSET,
    DISPLAY_TAPE_HALF_WIDTH,
};

pub fn draw_tape(tape: &Tape, pos: isize, offset: isize, draw: &Draw) {
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

pub fn draw_cell(pos: isize, draw: &Draw) {
    draw.rect()
        .stroke_color(CELL_OUTLINE_COLOR)
        .stroke_weight(CELL_STROKE_WIDTH)
        .no_fill()
        .w(CELL_WIDTH)
        .h(CELL_HEIGHT)
        .x_y(CELL_WIDTH * pos as f32, CELL_Y_OFFSET);
}

pub fn draw_symbol(content: &Symbol, pos: isize, draw: &Draw) {
    let symbol_text = if content.is_empty() {
        String::default()
    } else {
        content.to_string()
    };

    draw.text(&symbol_text)
        .x_y(CELL_WIDTH * pos as f32, CELL_Y_OFFSET)
        .center_justify();
}
