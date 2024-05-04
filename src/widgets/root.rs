use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{
        block::{Position, Title},
        Block, Borders, Padding,
    },
};

use crate::{
    chronology::{Chronological, Chronology, WeatherVariant},
    Flags,
};

use super::{
    clouds::CloudWidget, debug::DebugWidget, grass::GrassWidget, rain::RainWidget, tree::TreeWidget,
};

pub const MIN_X: f64 = -64.0;
pub const MAX_X: f64 = 64.0;
pub const MIN_Y: f64 = 0.0;
pub const MAX_Y: f64 = 128.0;

pub struct RootWidget<'a> {
    pub title: &'a String,
    pub flags: &'a Flags,
    pub chronology: &'a Chronology,
}

impl<'a> Widget for RootWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let remaining = self.chronology.growth_timer.remaining_secs();
        let paused = self.chronology.global_time.paused();

        let timer = format!(
            "[ {:0>2}:{:0>2} ]",
            remaining as u32 / 60,
            remaining as u32 % 60
        );

        let title = Title::from(if self.title.is_empty() {
            format!(" {} ", timer)
        } else {
            format!(" {} {} ", self.title, timer)
        });

        let subtitle = Title::from(Line::from(vec![
            Span::from(" [q] quit "),
            Span::styled("───", Style::default().fg(Color::Green)),
            Span::from(format!(
                " [p] {} ",
                if paused { "unpause" } else { "pause" }
            )),
        ]));

        let block = Block::default()
            .title(title.alignment(Alignment::Center))
            .title(
                subtitle
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
            )
            .title_style(Style::default().fg(Color::Reset))
            .borders(Borders::all())
            .border_set(border::ROUNDED)
            .border_style(Style::default().fg(Color::Green))
            .padding(Padding::horizontal(2));

        let inner_area = block.inner(area);

        block.render(area, buf);

        match self.chronology.weather_variant {
            WeatherVariant::Clouds => CloudWidget::frame(self.chronology).render(inner_area, buf),
            WeatherVariant::Rain => RainWidget::frame(self.chronology).render(inner_area, buf),
            WeatherVariant::None => {}
        }

        GrassWidget::frame(self.chronology).render(inner_area, buf);
        TreeWidget::frame(self.chronology).render(inner_area, buf);

        if self.flags.debug {
            DebugWidget::new(self.chronology).render(inner_area, buf);
        }
    }
}
