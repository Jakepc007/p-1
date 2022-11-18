use bevy::prelude::{Commands, EventReader, EventWriter, Input, KeyCode, Res};

use super::{events::SpawnOrbEvent, model::OrbGroup};

pub fn spawn_orb_event_listener(mut reader: EventReader<SpawnOrbEvent>, mut commands: Commands) {
    for SpawnOrbEvent {
        group,
        starting_position,
    } in reader.iter()
    {
        println!("Spawning Orb");

        match group {
            OrbGroup::Player => {
                println!("Spawn player within the orb holder if no position given")
            }
            OrbGroup::Enemy => {
                println!("Spawn enemy in a row by default at the enemy combat position")
            }
        }
    }
}

pub fn spawn_player_orb_on_space_bar_pressed(
    input: Res<Input<KeyCode>>,
    mut spawn_orb_writer: EventWriter<SpawnOrbEvent>,
) {
    if input.just_pressed(KeyCode::Space) {
        spawn_orb_writer.send(SpawnOrbEvent::new(OrbGroup::Player, None));
    }
}
