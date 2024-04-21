use itertools::Itertools;
use ratatui::{
    style::Color,
    symbols::Marker,
    widgets::{
        canvas::{Canvas, Line},
        Widget,
    },
};

use crate::chronology::{Chronological, Chronology};

use super::root::{MAX_X, MAX_Y, MIN_X, MIN_Y};

pub struct RainWidget {
    lines: Vec<Line>,
}

impl Chronological for RainWidget {
    fn frame(chronology: &Chronology) -> Self {
        let elapsed = (chronology.weather_timer.elapsed_secs() * 24.0) as f64;

        let lines: Vec<Line> = (0..24)
            .cartesian_product(0..16)
            .map(|(x, y)| {
                let x_jitter = 2.0 * (y % 3 + y % 5) as f64;
                let y_jitter = 2.0 * (x % 4 + x % 2) as f64;
                let x1 = (x as f64 * 8.0 + elapsed * 0.4 + x_jitter) % 192.0 - 96.0;
                let y1 = 128.0 - ((elapsed * 2.0 + y as f64 * 8.0 + y_jitter) % 128.0);
                Line {
                    x1,
                    y1,
                    x2: x1 + 0.1,
                    y2: y1 - 0.5,
                    color: Color::Cyan,
                }
            })
            .collect();

        RainWidget { lines }
    }
}

impl Widget for RainWidget {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let canvas_braille = Canvas::default()
            .x_bounds([MIN_X, MAX_X])
            .y_bounds([MIN_Y, MAX_Y])
            .marker(Marker::Braille)
            .paint(|ctx| {
                self.lines.iter().for_each(|line| {
                    ctx.draw(line);
                });
            });

        canvas_braille.render(area, buf);
    }
}
