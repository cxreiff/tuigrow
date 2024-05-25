use bevy::prelude::Resource;
use ratatui::{
    prelude::*,
    text,
    widgets::{
        canvas::{Canvas, Line, Shape},
        Block, BorderType, Borders, Padding,
    },
};

use crate::{
    chronology::{Chronology, WeatherVariant},
    drawings::{clouds::Clouds, grass::Grass, rain::Rain, tree::Tree},
    utils::bound_lines,
    Flags,
};

use super::debug::DebugWidget;

pub const MIN_X: f64 = -64.0;
pub const MAX_X: f64 = 64.0;
pub const MIN_Y: f64 = 0.0;
pub const MAX_Y: f64 = 128.0;

#[derive(Resource)]
pub struct RootWidget<'a> {
    pub flags: &'a Flags,
    pub chronology: &'a Chronology,
    pub title: &'a str,
    pub grass: &'a Grass,
    pub tree: &'a Tree,
    pub clouds: &'a Clouds,
    pub rain: &'a Rain,
}

impl<'a> Widget for RootWidget<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut Buffer) {
        let timer_block = self.timer_block();

        let inner_area = timer_block.inner(area);

        timer_block.render(area, buf);

        match self.chronology.weather_variant {
            WeatherVariant::Clouds => self.render_braille(self.clouds, inner_area, buf),
            WeatherVariant::Rain => self.render_braille(
                &self.rain.iter().map(bound_lines).collect::<Vec<Line>>(),
                inner_area,
                buf,
            ),
            WeatherVariant::None => {}
        }

        self.render_braille(
            &self.tree.iter().map(bound_lines).collect::<Vec<Line>>(),
            inner_area,
            buf,
        );
        self.render_braille(
            &self.grass.iter().map(bound_lines).collect::<Vec<Line>>(),
            inner_area,
            buf,
        );

        if self.flags.debug {
            DebugWidget::new(self.chronology).render(inner_area, buf);
        }
    }
}

impl<'a> RootWidget<'a> {
    fn timer_block(&self) -> Block {
        let remaining = self.chronology.growth_timer.remaining_secs() as u32;
        let paused = self.chronology.global_time.paused();

        let timer = format!("[ {:0>2}:{:0>2} ]", remaining / 60, remaining % 60);

        let title = text::Line::from(if self.title.is_empty() {
            format!(" {} ", timer)
        } else {
            format!(" {} {} ", self.title, timer)
        });

        Block::default()
            .borders(Borders::all())
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Green))
            .padding(Padding::horizontal(2))
            .title_alignment(Alignment::Center)
            .title_style(Style::default().fg(Color::Reset))
            .title_top(title)
            .title_bottom(" [q] quit ")
            .title_bottom(format!(
                " [p] {} ",
                if paused { "unpause" } else { "pause" }
            ))
    }

    fn render_braille(
        &self,
        shapes: &[impl Shape],
        area: ratatui::prelude::Rect,
        buf: &mut Buffer,
    ) {
        Canvas::default()
            .x_bounds([MIN_X, MAX_X])
            .y_bounds([MIN_Y, MAX_Y])
            .marker(Marker::Braille)
            .paint(|ctx| {
                for shape in shapes {
                    ctx.draw(shape)
                }
            })
            .render(area, buf)
    }
}
