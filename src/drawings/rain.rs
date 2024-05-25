use std::array;

use bevy::prelude::*;
use itertools::Itertools;
use ratatui::{style::Color, widgets::canvas::Line};

use crate::chronology::Chronology;

#[derive(Resource, Deref, DerefMut)]
pub struct Rain([Line; 384]);

pub fn setup(mut commands: Commands) {
    let mut lines = (0..24).cartesian_product(0..16);
    commands.insert_resource(Rain(array::from_fn(|_| {
        let (x, y) = lines.next().unwrap();
        get_line(x, y, 0.0)
    })));
}

pub fn update(chronology: Res<Chronology>, mut rain: ResMut<Rain>) {
    let elapsed = chronology.global_time.elapsed_secs_f64();

    let mut lines = (0..24).cartesian_product(0..16);
    *rain = Rain(array::from_fn(|_| {
        let (x, y) = lines.next().unwrap();
        get_line(x, y, elapsed)
    }));
}

fn get_line(x: u32, y: u32, elapsed: f64) -> Line {
    let x_jitter = 2.0 * (y % 3 + y % 5) as f64;
    let y_jitter = 2.0 * (x % 4 + x % 2) as f64;
    let x1 = (x as f64 * 8.0 + elapsed * 12.8 + x_jitter) % 192.0 - 96.0;
    let y1 = 128.0 - ((elapsed * 64.0 + y as f64 * 8.0 + y_jitter) % 128.0);

    Line {
        x1,
        y1,
        x2: x1 + 0.1,
        y2: y1 - 0.5,
        color: Color::Cyan,
    }
}
