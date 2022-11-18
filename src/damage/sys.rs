use bevy::prelude::{EventReader, EventWriter, Query};

use crate::{death::comp::DeathEvent, health::comp::Health};

use super::event::TakeDamageEvent;

pub fn take_damage_event_listener(
    mut reader: EventReader<TakeDamageEvent>,
    mut entities_with_health: Query<&Health>,
    mut death_event: EventWriter<DeathEvent>,
) {
    for TakeDamageEvent {
        quantity,
        damage_type: _,
        source,
    } in reader.iter()
    {
        for Health { mut value, .. } in entities_with_health.iter_mut() {
            if value < *quantity {
                death_event.send(DeathEvent {
                    source: source.to_death_source(),
                    overkill: *quantity - value,
                })
            } else {
                value -= quantity
            }
        }
    }
}
