mod combat;
mod damage;
mod death;
mod health;
mod orb;
mod plugin_groups;
mod effect;
use bevy::{prelude::App, DefaultPlugins};
use plugin_groups::CorePlugins;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CorePlugins)
        .run();
}
