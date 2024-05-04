use bevy::{app::ScheduleRunnerPlugin, prelude::*};
use bevy_rat::{ratatui_error_handler, ratatui_plugin, RatatuiResource};
use chronology::{chronology_setup, chronology_update, Chronology};
use clap::Parser;
use keys::handle_keys;
use std::io;
use std::time::Duration;
use widgets::root::RootWidget;

mod chronology;
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
        .add_plugins(ratatui_plugin)
        .add_systems(Startup, chronology_setup)
        .add_systems(Update, chronology_update)
        .add_systems(Update, ratatui_update.pipe(ratatui_error_handler))
        .add_systems(Update, handle_keys.pipe(ratatui_error_handler))
        .insert_resource(Flags::default())
        .insert_resource(Args::parse())
        .run();
}

fn ratatui_update(
    mut rat: ResMut<RatatuiResource>,
    args: Res<Args>,
    flags: Res<Flags>,
    chronology: Res<Chronology>,
) -> io::Result<()> {
    rat.terminal.draw(|frame| {
        frame.render_widget(
            RootWidget {
                title: &args.title,
                flags: &flags,
                chronology: &chronology,
            },
            frame.size(),
        );
    })?;

    Ok(())
}
