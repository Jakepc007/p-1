use bevy::prelude::Entity;

use crate::death::comp::DeathSource;

pub enum DamageType {
    Normal,
}

pub enum DamageSource {
    Orb { entity: Entity },
}

impl DamageSource {
    pub(crate) fn to_death_source(&self) -> crate::death::comp::DeathSource {
        match self {
            DamageSource::Orb { entity } => DeathSource::Orb { entity: *entity },
        }
    }
}
