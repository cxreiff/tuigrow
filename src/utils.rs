use ratatui::widgets::canvas::{Context, Line};

use crate::widgets::root::{MAX_X, MAX_Y, MIN_X, MIN_Y};

pub fn draw_lines_bounded(ctx: &mut Context, lines: &[Line]) {
    lines
        .iter()
        .map(|line| Line {
            x1: line.x1.max(MIN_X).min(MAX_X),
            y1: line.y1.max(MIN_Y).min(MAX_Y),
            x2: line.x2.max(MIN_X).min(MAX_X),
            y2: line.y2.max(MIN_Y).min(MAX_Y),
            color: line.color,
        })
        .for_each(|line| {
            ctx.draw(&line);
        });
}
