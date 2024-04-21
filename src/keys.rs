use std::io;

use bevy::{app::AppExit, prelude::*};
use bevy_rat::RatatuiEvent;
use crossterm::event;

use crate::{chronology::Chronology, Flags};

pub fn q_to_quit(
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

pub fn p_to_pause(
    mut rat_events: EventReader<RatatuiEvent>,
    mut chronology: ResMut<Chronology>,
) -> io::Result<()> {
    for ev in rat_events.read() {
        if let RatatuiEvent(event::Event::Key(key_event)) = ev {
            if key_event.kind == event::KeyEventKind::Press
                && key_event.code == event::KeyCode::Char('p')
            {
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
        }
    }

    Ok(())
}

pub fn d_to_debug(
    mut rat_events: EventReader<RatatuiEvent>,
    mut flags: ResMut<Flags>,
) -> io::Result<()> {
    for ev in rat_events.read() {
        if let RatatuiEvent(event::Event::Key(key_event)) = ev {
            if key_event.kind == event::KeyEventKind::Press
                && key_event.code == event::KeyCode::Char('d')
            {
                flags.debug = !flags.debug;
            }
        }
    }

    Ok(())
}
