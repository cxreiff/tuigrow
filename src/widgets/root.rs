use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{
        block::{Position, Title},
        Block, Borders, Padding,
    },
};

use super::{clouds::CloudWidget, grass::GrassWidget, tree::TreeWidget};

pub const MIN_X: f64 = -64.0;
pub const MAX_X: f64 = 64.0;
pub const MIN_Y: f64 = 0.0;
pub const MAX_Y: f64 = 128.0;

pub trait Grows {
    fn grew(elapsed: f64, percent: f64) -> Self;
}

pub struct RootWidget {
    pub title: String,
    pub elapsed: f64,
    pub remaining: f64,
    pub percent: f64,
    pub paused: bool,
}

impl Widget for RootWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(format!(
            " {} [ {:0>2}:{:0>2} ] ",
            self.title,
            self.remaining as u32 / 60,
            self.remaining as u32 % 60
        ));
        let subtitle = Title::from(Line::from(vec![
            Span::from(" [q] quit "),
            Span::styled("───", Style::default().fg(Color::Green)),
            Span::from(format!(
                " [p] {} ",
                if self.paused { "unpause" } else { "pause" }
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
            .padding(Padding::new(2, 2, 1, 0));

        let inner_area = block.inner(area);

        block.render(area, buf);

        CloudWidget::grew(self.elapsed, self.percent).render(inner_area, buf);
        GrassWidget::grew(self.elapsed, self.percent).render(inner_area, buf);
        TreeWidget::grew(self.elapsed, self.percent).render(inner_area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render() {
        let widget = RootWidget {
            title: "test".into(),
            elapsed: 48.0,
            remaining: 80.0,
            percent: 0.375,
            paused: false,
        };

        let mut buf = Buffer::empty(Rect::new(0, 0, 50, 5));

        widget.render(buf.area, &mut buf);

        let expected = Buffer::with_lines(vec![
            "┏ test title ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓",
            "┃                                                ┃",
            "┃  count: 44 ('q' to quit)                       ┃",
            "┃                                                ┃",
            "┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛",
        ]);

        assert_eq!(buf, expected);
    }
}
