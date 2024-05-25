use bevy::prelude::*;
use ratatui::style::Color;

use crate::{chronology::Chronology, shapes::cloud::Cloud};

#[derive(Resource, Deref, DerefMut)]
pub struct Clouds(Vec<Cloud>);

pub fn setup(mut commands: Commands) {
    commands.insert_resource(Clouds(vec![]));
}

pub fn update(chronology: Res<Chronology>, mut clouds: ResMut<Clouds>) {
    let horizontal =
        |offset: f64| (chronology.weather_timer.elapsed_secs() as f64 + offset) % 192.0 - 96.0;

    *clouds = Clouds(
        [
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
        .collect(),
    );
}
