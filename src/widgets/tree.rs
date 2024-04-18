use ratatui::{
    style::Color,
    symbols::Marker,
    widgets::{
        canvas::{Canvas, Line},
        Widget,
    },
};

use crate::utils::draw_lines_bounded;

use super::root::{Grows, MAX_X, MAX_Y, MIN_X, MIN_Y};

pub struct TreeWidget {
    lines: Vec<Line>,
}

impl Grows for TreeWidget {
    fn grew(elapsed: f64, percent: f64) -> Self {
        let lines: Vec<Line> = (1..((percent * MAX_Y) as u32 / 4))
            .flat_map(|i| {
                let growth = ((percent * MAX_Y) as u32 - i * 4) as f64 / 4.0;
                let groundedness = (12 / i).pow(3) as f64;
                let variance = (i % 3 * 2) as f64;
                let altitude = (i * 4) as f64;
                let lift = (elapsed / 10.0) % 1.0 * 0.5;
                let color = if i % 2 == 0 {
                    Color::LightGreen
                } else {
                    Color::Green
                };

                let line1 = Line {
                    x1: 0.0,
                    y1: altitude,
                    x2: (growth - variance - groundedness).max(0.0),
                    y2: altitude + growth + variance - groundedness + lift,
                    color,
                };

                let line2 = Line {
                    x1: 0.0,
                    y1: altitude,
                    x2: ((growth * 0.8 - 4.0).max(0.0) - variance - groundedness).max(0.0),
                    y2: altitude + (growth * 0.8).max(0.0) + variance - groundedness + lift,
                    color,
                };

                vec![
                    Line {
                        x2: -line1.x2,
                        ..line1
                    },
                    Line {
                        x2: -line2.x2,
                        ..line2
                    },
                    line1,
                    line2,
                ]
            })
            .chain(vec![
                Line {
                    x1: 0.0,
                    y1: 0.0,
                    x2: 0.0,
                    y2: 0.0 + (percent * MAX_Y).min(128.0),
                    color: Color::Green,
                },
                Line {
                    x1: -0.5,
                    y1: 0.0,
                    x2: -0.5,
                    y2: (percent * MAX_Y).min(128.0) * 2.0 / 3.0,
                    color: Color::Green,
                },
            ])
            .collect();

        TreeWidget { lines }
    }
}

impl Widget for TreeWidget {
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
