use std::{fmt, time::Duration};

use bevy::{prelude::*, time::Stopwatch};
use rand::distributions::{Distribution, Standard};

use crate::Args;

#[derive(Resource)]
pub struct Chronology {
    pub global_time: Stopwatch,
    pub growth_timer: Timer,
    pub weather_timer: Timer,
    pub weather_variant: WeatherVariant,
}

#[derive(Copy, Clone, Debug)]
pub enum WeatherVariant {
    None,
    Clouds,
    Rain,
}

impl Distribution<WeatherVariant> for Standard {
    fn sample<R: rand::prelude::Rng + ?Sized>(&self, rng: &mut R) -> WeatherVariant {
        match rng.gen_range(0..=1) {
            0 => WeatherVariant::Clouds,
            1 => WeatherVariant::Rain,
            _ => WeatherVariant::None,
        }
    }
}

impl fmt::Display for WeatherVariant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

pub fn chronology_setup(mut commands: Commands, args: Res<Args>) {
    commands.insert_resource(Chronology {
        global_time: Stopwatch::new(),
        growth_timer: Timer::new(Duration::from_secs(args.time), TimerMode::Once),
        weather_timer: Timer::from_seconds(360.0, TimerMode::Repeating),
        weather_variant: WeatherVariant::Clouds,
    });
}

pub fn chronology_update(time: Res<Time>, mut chronology: ResMut<Chronology>) {
    chronology.global_time.tick(time.delta());
    chronology.growth_timer.tick(time.delta());
    chronology.weather_timer.tick(time.delta());

    if chronology.weather_timer.just_finished() {
        chronology.weather_variant = rand::random();
    }
}
