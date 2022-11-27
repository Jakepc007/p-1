use bevy::prelude::PluginGroup;

use crate::{camera::bevy::CameraPlugin, boid::bevy::BoidPlugin};

pub struct CorePlugins;

impl PluginGroup for CorePlugins {
    fn build(&mut self, group: &mut bevy::app::PluginGroupBuilder) {
        group.add(CameraPlugin);
        group.add(BoidPlugin);
    }
}
