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

pub struct TreeWidget {
    lines: Vec<Line>,
}

impl Chronological for TreeWidget {
    fn frame(chronology: &Chronology) -> Self {
        let elapsed = chronology.global_time.elapsed_secs_f64();
        let percent = chronology.growth_timer.fraction() as f64;

        let lines: Vec<Line> = (5..((percent * MAX_Y) as u32 / 4))
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

                vec![
                    Line {
                        x1: 0.0,
                        y1: altitude,
                        x2: (growth - variance - groundedness).max(0.0),
                        y2: altitude + growth + variance - groundedness + lift,
                        color,
                    },
                    Line {
                        x1: 0.0,
                        y1: altitude,
                        x2: ((growth * 0.8 - 4.0).max(0.0) - variance - groundedness).max(0.0),
                        y2: altitude + (growth * 0.8).max(0.0) + variance - groundedness + lift,
                        color,
                    },
                ]
            })
            .chain(vec![
                Line {
                    x1: 0.25,
                    y1: (percent * MAX_Y).min(128.0) * 2.0 / 3.0,
                    x2: 0.0,
                    y2: (percent * MAX_Y).min(128.0),
                    color: Color::Green,
                },
                Line {
                    x1: 0.75,
                    y1: (percent * MAX_Y).min(128.0) * 1.0 / 12.0,
                    x2: 0.25,
                    y2: (percent * MAX_Y).min(128.0) * 2.0 / 3.0,
                    color: Color::Green,
                },
                Line {
                    x1: 2.0,
                    y1: 0.0,
                    x2: 0.75,
                    y2: (percent * MAX_Y).min(128.0) * 1.0 / 12.0,
                    color: Color::Green,
                },
            ])
            .flat_map(|line| {
                vec![
                    Line {
                        x1: -line.x1,
                        x2: -line.x2,
                        ..line
                    },
                    line,
                ]
            })
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
