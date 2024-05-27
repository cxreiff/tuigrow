use bevy::utils::error;
use bevy::{app::ScheduleRunnerPlugin, prelude::*};
use bevy_rat::{RatPlugin, RatResource};
use chronology::{chronology_setup, chronology_update, Chronology};
use clap::Parser;
use drawings::clouds::{self, Clouds};
use drawings::grass::{self, Grass};
use drawings::rain::{self, Rain};
use drawings::tree::{self, Tree};
use keys::handle_keys;
use std::io;
use std::time::Duration;
use widgets::root::RootWidget;

mod chronology;
mod drawings;
mod keys;
mod shapes;
mod utils;
mod widgets;

#[derive(Parser, Resource)]
struct Args {
    #[arg(default_value = "")]
    title: String,

    #[arg(short, long, default_value_t = 180)]
    time: u64,
}

#[derive(Resource, Default)]
pub struct Flags {
    debug: bool,
}

fn main() {
    App::new()
        .add_plugins(
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
                1.0 / 30.0,
            ))),
        )
        .add_plugins(RatPlugin)
        .add_systems(Startup, chronology_setup)
        .add_systems(Update, chronology_update)
        .add_systems(Startup, grass::setup)
        .add_systems(Update, grass::update)
        .add_systems(Startup, tree::setup)
        .add_systems(Update, tree::update)
        .add_systems(Startup, clouds::setup)
        .add_systems(Update, clouds::update)
        .add_systems(Startup, rain::setup)
        .add_systems(Update, rain::update)
        .add_systems(Update, rat_render.map(error))
        .add_systems(Update, handle_keys.map(error))
        .insert_resource(Flags::default())
        .insert_resource(Args::parse())
        .run();
}

#[allow(clippy::too_many_arguments)]
fn rat_render(
    mut rat: ResMut<RatResource>,
    args: Res<Args>,
    flags: Res<Flags>,
    chronology: Res<Chronology>,
    grass: Res<Grass>,
    tree: Res<Tree>,
    clouds: Res<Clouds>,
    rain: Res<Rain>,
) -> io::Result<()> {
    rat.terminal.draw(|frame| {
        frame.render_widget(
            RootWidget {
                title: &args.title,
                flags: &flags,
                chronology: &chronology,
                grass: &grass,
                tree: &tree,
                clouds: &clouds,
                rain: &rain,
            },
            frame.size(),
        );
    })?;

    Ok(())
}
