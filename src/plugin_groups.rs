use bevy::prelude::PluginGroup;

use crate::orb::plugin::OrbPlugin;

pub struct CorePlugins;

impl PluginGroup for CorePlugins {
    fn build(&mut self, group: &mut bevy::app::PluginGroupBuilder) {
        group.add(OrbPlugin);
    }
}
