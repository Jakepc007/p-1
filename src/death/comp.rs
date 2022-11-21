use bevy::prelude::Entity;

pub enum DeathSource {
    Orb { entity: Entity },
}
