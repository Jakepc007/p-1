mod boid;
mod camera;
mod plugin_groups;
mod combinable_draggable;

use bevy::{
    prelude::{App, ClearColor, Color, Commands, Transform, Vec3},
    sprite::{Sprite, SpriteBundle},
    DefaultPlugins,
};
use plugin_groups::CorePlugins;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.8, 0.8, 1.0)))
        // .add_startup_system(spawn_blue_square)
        .add_plugins(DefaultPlugins)
        .add_plugins(CorePlugins)
        .run();
}

fn spawn_blue_square(mut commands: Commands) {
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.0, 0.0, 1.0),
            ..Default::default()
        },
        transform: Transform {
            scale: Vec3::new(100.0, 100.0, 1.0),
            translation: Vec3::new(0.0, 0.0, 0.0),
            ..Default::default()
        },
        ..Default::default()
    });
}
