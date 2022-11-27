use bevy::prelude::PluginGroup;

pub struct CorePlugins;

impl PluginGroup for CorePlugins {
    fn build(&mut self, group: &mut bevy::app::PluginGroupBuilder) {}
}
