use ratatui::{
    prelude::*,
    widgets::canvas::{Canvas, Line},
};

pub struct BushWidget {
    pub count: u32,
}

impl BushWidget {
    pub fn _new(count: u32) -> Self {
        BushWidget { count }
    }
}

impl Widget for BushWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Canvas::default()
            .x_bounds([0.0, 120.0])
            .y_bounds([0.0, 90.0])
            .paint(|ctx| {
                ctx.draw(&Line {
                    x1: 60.0,
                    y1: 0.0,
                    x2: 60.0,
                    y2: (self.count.min(90)).into(),
                    color: Color::Green,
                });
                ctx.draw(&Line {
                    x1: 61.0,
                    y1: 0.0,
                    x2: 61.0,
                    y2: (self.count.min(90)).into(),
                    color: Color::Green,
                });
                for i in 0..(self.count.min(90) / 8) {
                    ctx.draw(&Line {
                        x1: 61.0,
                        y1: (i * 8).into(),
                        x2: (61 + (self.count.min(90) - i * 8) / 4).min(120).into(),
                        y2: (i * 8 + (self.count.min(90) - 3 - i * 4) / 4)
                            .min(90)
                            .into(),
                        color: Color::Green,
                    });
                    ctx.draw(&Line {
                        x1: 60.0,
                        y1: (1 + i * 8).into(),
                        x2: (60 - (self.count.min(90) - i * 8) / 4).max(0).into(),
                        y2: (i * 8 + (self.count.min(90) - 2 - i * 4) / 4)
                            .min(90)
                            .into(),
                        color: Color::Green,
                    });
                }
            })
            .render(area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render() {
        let widget = BushWidget { count: 44 };
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
