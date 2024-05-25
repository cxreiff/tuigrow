use std::{io, time::Duration};

use bevy::{app::AppExit, prelude::*};
use bevy_rat::RatatuiEvent;
use crossterm::event;

use crate::{chronology::Chronology, Flags};

pub fn handle_keys(
    mut rat_events: EventReader<RatatuiEvent>,
    mut exit: EventWriter<AppExit>,
    mut flags: ResMut<Flags>,
    mut chronology: ResMut<Chronology>,
) -> io::Result<()> {
    for ev in rat_events.read() {
        if let RatatuiEvent(event::Event::Key(key_event)) = ev {
            if key_event.kind == event::KeyEventKind::Press {
                match key_event.code {
                    event::KeyCode::Char('q') => {
                        exit.send(AppExit::Success);
                    }
                    event::KeyCode::Char('p') => {
                        if chronology.global_time.paused() {
                            chronology.global_time.unpause();
                            chronology.growth_timer.unpause();
                            chronology.weather_timer.unpause();
                        } else {
                            chronology.global_time.pause();
                            chronology.growth_timer.pause();
                            chronology.weather_timer.pause();
                        }
                    }
                    event::KeyCode::Char('d') => {
                        flags.debug = !flags.debug;
                    }
                    event::KeyCode::Char('w') => {
                        let duration =
                            chronology.weather_timer.duration() - Duration::from_millis(10);
                        chronology.weather_timer.set_elapsed(duration);
                    }
                    _ => {}
                }
            }
        }
    }

    Ok(())
}
