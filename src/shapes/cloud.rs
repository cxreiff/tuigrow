use ratatui::{
    style::Color,
    widgets::canvas::{Painter, Shape},
};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Cloud {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
    pub color: Color,
}

impl Shape for Cloud {
    fn draw(&self, painter: &mut Painter<'_, '_>) {
        for angle in 0..140 {
            let radians = f64::from(angle).to_radians();
            let circle_x = (self.radius).mul_add(radians.cos(), self.x + self.radius * 2.0);
            let circle_y = self.radius.mul_add(radians.sin(), self.y);
            if let Some((x, y)) = painter.get_point(circle_x, circle_y) {
                painter.paint(x, y, self.color);
            }
        }
        for angle in 0..180 {
            let radians = f64::from(angle).to_radians();
            let circle_x = (self.radius * 1.5).mul_add(radians.cos(), self.x);
            let circle_y = self.radius.mul_add(radians.sin(), self.y + self.radius);
            if let Some((x, y)) = painter.get_point(circle_x, circle_y) {
                painter.paint(x, y, self.color);
            }
        }
        for angle in 40..180 {
            let radians = f64::from(angle).to_radians();
            let circle_x = (self.radius).mul_add(radians.cos(), self.x - self.radius * 2.0);
            let circle_y = self.radius.mul_add(radians.sin(), self.y);
            if let Some((x, y)) = painter.get_point(circle_x, circle_y) {
                painter.paint(x, y, self.color);
            }
        }
        for angle in 180..360 {
            let radians = f64::from(angle).to_radians();
            let circle_x = (self.radius * 3.0).mul_add(radians.cos(), self.x);
            let circle_y = (self.radius * 0.6).mul_add(radians.sin(), self.y);
            if let Some((x, y)) = painter.get_point(circle_x, circle_y) {
                painter.paint(x, y, self.color);
            }
        }
    }
}
