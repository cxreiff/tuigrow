use bevy::{app::ScheduleRunnerPlugin, prelude::*};
use bevy_rat::{ratatui_error_handler, ratatui_plugin, RatatuiResource};
use chronology::{chronology_setup, chronology_update, Chronology};
use keys::{d_to_debug, p_to_pause, q_to_quit, w_to_weather};
use std::io;
use std::time::Duration;
use widgets::root::RootWidget;

mod chronology;
mod keys;
mod shapes;
mod utils;
mod widgets;

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
        .add_systems(Update, q_to_quit.pipe(ratatui_error_handler))
        .add_systems(Update, p_to_pause.pipe(ratatui_error_handler))
        .add_systems(Update, d_to_debug.pipe(ratatui_error_handler))
        .add_systems(Update, w_to_weather.pipe(ratatui_error_handler))
        .insert_resource(Flags::default())
        .run();
}

fn ratatui_update(
    mut rat: ResMut<RatatuiResource>,
    flags: ResMut<Flags>,
    chronology: Res<Chronology>,
) -> io::Result<()> {
    rat.terminal.draw(|frame| {
        frame.render_widget(
            RootWidget {
                title: "test".into(),
                flags: &flags,
                chronology: &chronology,
            },
            frame.size(),
        );
    })?;

    Ok(())
}
