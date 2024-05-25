use std::array;

use bevy::prelude::*;
use ratatui::{style::Color, widgets::canvas::Line};

use crate::chronology::Chronology;

#[derive(Resource, Deref, DerefMut)]
pub struct Grass([Line; 128]);

pub fn setup(mut commands: Commands) {
    commands.insert_resource(Grass(array::from_fn(|i| {
        let i = i as i32;

        let color = if i % 2 == 0 {
            Color::LightGreen
        } else {
            Color::Green
        };

        Line {
            x1: (i - 64) as f64,
            y1: 0.0,
            x2: (i - 64) as f64 + (i as f64 / 10.0).sin(),
            y2: 2.0 + (i % 3) as f64 - ((i - 64) as f64).abs() / 16.0 * 2.0,
            color,
        }
    })));
}

pub fn update(chronology: Res<Chronology>, mut grass: ResMut<Grass>) {
    let elapsed = chronology.global_time.elapsed_secs_f64();
    let percent = chronology.growth_timer.fraction() as f64;

    grass.iter_mut().enumerate().for_each(|(i, line)| {
        line.x2 = (i as i32 - 64) as f64 + (elapsed + i as f64 / 10.0).sin();
        line.y2 = 2.0 + (i % 3) as f64 - ((i as i32 - 64) as f64).abs() / 16.0 + percent * 2.0;
    })
}
