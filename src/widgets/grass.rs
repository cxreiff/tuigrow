use ratatui::{
    style::Color,
    symbols::Marker,
    widgets::{
        canvas::{Canvas, Line},
        Widget,
    },
};

use crate::{
    chronology::{Chronological, Chronology},
    utils::draw_lines_bounded,
};

use super::root::{MAX_X, MAX_Y, MIN_X, MIN_Y};

pub struct GrassWidget {
    lines: Vec<Line>,
}

impl Chronological for GrassWidget {
    fn frame(chronology: &Chronology) -> Self {
        let elapsed = chronology.global_time.elapsed_secs_f64();
        let percent = chronology.growth_timer.fraction() as f64;

        let lines: Vec<Line> = (1..128)
            .map(|i| {
                let color = if i % 2 == 0 {
                    Color::LightGreen
                } else {
                    Color::Green
                };

                Line {
                    x1: (i - 64) as f64,
                    y1: 0.0,
                    x2: (i - 64) as f64 + (elapsed + i as f64 / 10.0).sin(),
                    y2: 2.0 + (i % 3) as f64 - ((i - 64) as f64).abs() / 16.0 + percent * 2.0,
                    color,
                }
            })
            .collect();

        GrassWidget { lines }
    }
}

impl Widget for GrassWidget {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let canvas_braille = Canvas::default()
            .x_bounds([MIN_X, MAX_X])
            .y_bounds([MIN_Y, MAX_Y])
            .marker(Marker::Braille)
            .paint(|ctx| {
                draw_lines_bounded(ctx, &self.lines);
            });

        canvas_braille.render(area, buf);
    }
}
