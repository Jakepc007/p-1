use bevy::prelude::Plugin;

use super::{event::TakeDamageEvent, sys::take_damage_event_listener};

pub struct DamagePlugin;

impl Plugin for DamagePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<TakeDamageEvent>()
            .add_system(take_damage_event_listener);
    }
}
