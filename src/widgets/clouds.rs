use ratatui::{
    style::Color,
    symbols::Marker,
    widgets::{canvas::Canvas, Widget},
};

use crate::{
    chronology::{Chronological, Chronology},
    shapes::cloud::Cloud,
};

use super::root::{MAX_X, MAX_Y, MIN_X, MIN_Y};

pub struct CloudWidget {
    clouds: Vec<Cloud>,
}

impl Chronological for CloudWidget {
    fn frame(chronology: &Chronology) -> Self {
        let horizontal =
            |offset: f64| (chronology.weather_timer.elapsed_secs() as f64 + offset) % 192.0 - 96.0;

        let clouds: Vec<Cloud> = vec![
            (horizontal(0.0), 104.0, 7.0),
            (horizontal(56.0), 86.0, 9.0),
            (horizontal(114.0), 100.0, 4.0),
            (horizontal(136.0), 80.0, 6.0),
        ]
        .into_iter()
        .filter(|(x, _, radius)| {
            let center_at_weather_end = x + chronology.weather_timer.remaining_secs() as f64;
            let left_edge_at_weather_end = center_at_weather_end - radius * 3.0;
            let center_at_weather_start = x - chronology.weather_timer.elapsed_secs() as f64;
            let right_edge_at_weather_start = center_at_weather_start + radius * 3.0;
            left_edge_at_weather_end > 64.0 && right_edge_at_weather_start < -64.0
        })
        .map(|(x, y, radius)| Cloud {
            x,
            y,
            radius,
            color: Color::DarkGray,
        })
        .collect();

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
                });
            });

        canvas_braille.render(area, buf);
    }
}
