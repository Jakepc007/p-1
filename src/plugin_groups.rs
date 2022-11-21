use bevy::prelude::PluginGroup;

use crate::{damage::plugin::DamagePlugin, orb::plugin::OrbPlugin};

pub struct CorePlugins;

impl PluginGroup for CorePlugins {
    fn build(&mut self, group: &mut bevy::app::PluginGroupBuilder) {
        group.add(OrbPlugin);
        group.add(DamagePlugin);
        group.add(DeathPlugin)
    }
}
