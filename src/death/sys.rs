use bevy::prelude::EventReader;

use super::event::DeathEvent;

pub fn death_event_listener(mut reader: EventReader<DeathEvent>) {
    for DeathEvent { source, overkill } in reader.iter() {}
}
