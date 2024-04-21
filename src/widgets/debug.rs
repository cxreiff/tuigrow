use ratatui::{prelude::*, widgets::Paragraph};

use crate::chronology::Chronology;

pub struct DebugWidget<'a> {
    chronology: &'a Chronology,
}

impl<'a> DebugWidget<'a> {
    pub fn new(chronology: &'a Chronology) -> Self {
        DebugWidget { chronology }
    }
}

impl<'a> Widget for DebugWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new(vec![
            Line::from(vec![
                "weather_variant: ".light_red(),
                self.chronology.weather_variant.to_string().red(),
            ]),
            Line::from(vec![
                "weather_timer: ".light_red(),
                self.chronology
                    .weather_timer
                    .remaining_secs()
                    .to_string()
                    .red(),
            ]),
        ])
        .render(area, buf);
    }
}
