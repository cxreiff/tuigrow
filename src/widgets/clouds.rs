use ratatui::{
    style::Color,
    symbols::Marker,
    widgets::{canvas::Canvas, Widget},
};

use crate::shapes::cloud::Cloud;

use super::root::{Grows, MAX_X, MAX_Y, MIN_X, MIN_Y};

pub struct CloudWidget {
    clouds: Vec<Cloud>,
}

impl Grows for CloudWidget {
    fn grew(elapsed: f64, _percent: f64) -> Self {
        let clouds: Vec<Cloud> = vec![
            Cloud {
                x: elapsed % 192.0 - 96.0,
                y: 104.0,
                radius: 7.0,
                color: Color::DarkGray,
            },
            Cloud {
                x: (elapsed + 56.0) % 192.0 - 96.0,
                y: 86.0,
                radius: 9.0,
                color: Color::DarkGray,
            },
            Cloud {
                x: (elapsed + 114.0) % 192.0 - 96.0,
                y: 100.0,
                radius: 4.0,
                color: Color::DarkGray,
            },
            Cloud {
                x: (elapsed + 136.0) % 192.0 - 96.0,
                y: 80.0,
                radius: 6.0,
                color: Color::DarkGray,
            },
        ];

        CloudWidget { clouds }
    }
}

impl Widget for CloudWidget {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let canvas_braille = Canvas::default()
            .x_bounds([MIN_X, MAX_X])
            .y_bounds([MIN_Y, MAX_Y])
            .marker(Marker::Braille)
            .paint(|ctx| {
                self.clouds.iter().for_each(|cloud| {
                    ctx.draw(cloud);
                })
            });

        canvas_braille.render(area, buf);
    }
}
