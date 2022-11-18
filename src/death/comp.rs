use bevy::prelude::Entity;

pub struct DeathEvent {
    pub source: DeathSource,
    pub overkill: i8,
}

pub enum DeathSource {
    Orb { entity: Entity },
}
