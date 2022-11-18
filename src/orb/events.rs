use bevy::prelude::Vec2;

use super::model::OrbGroup;

#[derive(Debug)]
pub struct SpawnOrbEvent {
    pub group: OrbGroup,
    pub starting_position: Option<Vec2>,
}

impl SpawnOrbEvent {
    pub fn new(group: OrbGroup, starting_position: Option<Vec2>) -> SpawnOrbEvent {
        Self {
            group,
            starting_position,
        }
    }
}
