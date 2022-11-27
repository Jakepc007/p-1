use bevy::{
    prelude::{Component, Plugin, Query, Res, Transform},
    window::Window,
};

pub struct CombinableDraggablePlugin;

#[derive(Component)]
pub struct CombinableDraggable {
    hovered: bool,
    selected: bool,
}

impl Plugin for CombinableDraggablePlugin {
    fn build(&self, app: &mut bevy::app::App) {}
}

fn highlight_hovered(
    mut query: Query<(&Transform, &mut CombinableDraggable)>,
    windows: Res<Window>,
) {
}
