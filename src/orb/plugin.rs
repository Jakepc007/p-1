use bevy::prelude::Plugin;

use super::{
    events::SpawnOrbEvent,
    sys::{spawn_orb_event_listener, spawn_player_orb_on_space_bar_pressed},
};

pub struct OrbPlugin;

impl Plugin for OrbPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<SpawnOrbEvent>()
            .add_system(spawn_orb_event_listener)
            .add_system(spawn_player_orb_on_space_bar_pressed);
    }
}
