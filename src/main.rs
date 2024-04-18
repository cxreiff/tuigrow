use bevy::{
    app::{AppExit, ScheduleRunnerPlugin},
    prelude::*,
    time::Stopwatch,
};
use bevy_rat::{ratatui_error_handler, ratatui_plugin, RatatuiEvent, RatatuiResource};
use crossterm::event;
use std::io;
use std::time::Duration;
use widgets::root::RootWidget;

mod shapes;
mod utils;
mod widgets;

fn main() {
    App::new()
        .add_plugins(
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
                1.0 / 10.0,
            ))),
        )
        .add_plugins(ratatui_plugin)
        .add_systems(Startup, growth_setup)
        .add_systems(Update, growth_update)
        .add_systems(Update, ratatui_update.pipe(ratatui_error_handler))
        .add_systems(Update, q_to_quit.pipe(ratatui_error_handler))
        .add_systems(Update, p_to_pause.pipe(ratatui_error_handler))
        .run();
}

fn ratatui_update(mut rat: ResMut<RatatuiResource>, growth: Res<Growth>) -> io::Result<()> {
    rat.terminal.draw(|frame| {
        frame.render_widget(
            RootWidget {
                title: "test".into(),
                elapsed: growth.stopwatch.elapsed_secs() as f64,
                remaining: growth.timer.remaining_secs() as f64,
                percent: growth.timer.fraction() as f64,
                paused: growth.timer.paused(),
            },
            frame.size(),
        );
    })?;

    Ok(())
}

fn q_to_quit(
    mut rat_events: EventReader<RatatuiEvent>,
    mut exit: EventWriter<AppExit>,
) -> io::Result<()> {
    for ev in rat_events.read() {
        if let RatatuiEvent(event::Event::Key(key_event)) = ev {
            if key_event.kind == event::KeyEventKind::Press
                && key_event.code == event::KeyCode::Char('q')
            {
                exit.send(AppExit);
            }
        }
    }

    Ok(())
}

fn p_to_pause(
    mut rat_events: EventReader<RatatuiEvent>,
    mut growth: ResMut<Growth>,
) -> io::Result<()> {
    for ev in rat_events.read() {
        if let RatatuiEvent(event::Event::Key(key_event)) = ev {
            if key_event.kind == event::KeyEventKind::Press
                && key_event.code == event::KeyCode::Char('p')
            {
                if growth.timer.paused() {
                    growth.timer.unpause();
                    growth.stopwatch.unpause();
                } else {
                    growth.timer.pause();
                    growth.stopwatch.pause();
                }
            }
        }
    }

    Ok(())
}

#[derive(Resource)]
struct Growth {
    timer: Timer,
    stopwatch: Stopwatch,
}

fn growth_setup(mut commands: Commands) {
    commands.insert_resource(Growth {
        timer: Timer::new(Duration::from_secs(30), TimerMode::Once),
        stopwatch: Stopwatch::new(),
    });
}

fn growth_update(mut growth: ResMut<Growth>, time: Res<Time>) {
    growth.timer.tick(time.delta());
    growth.stopwatch.tick(time.delta());
}
