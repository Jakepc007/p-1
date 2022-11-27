mod move_to;

use bevy::{
    prelude::{
        App, Camera2dBundle, Commands, Component, Plugin, Res, ResMut, SystemSet, Transform, Vec2,
        Vec3,
    },
    time::{FixedTimestep, Time},
};

use self::move_to::{update_camera_position, CameraTarget};

pub struct CameraPlugin;

#[derive(Component)]
pub struct MainCamera;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        // resources
        app.insert_resource(CameraTarget {
            position: Vec2::ZERO,
            start_time: 0.0,
        });

        // systems
        app.add_startup_system(spawn_camera);
        app.add_system(update_camera_position);
        app.add_system_set(
            SystemSet::new().with_run_criteria(FixedTimestep::step(0.5)), // .with_system(debug_move_on_interval),
        );
    }
}

fn spawn_camera(mut commands: Commands) {
    commands
        .spawn_bundle(Camera2dBundle {
            transform: Transform {
                scale: Vec3::new(1.0, 1.0, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(MainCamera);
}

fn debug_move_on_interval(mut camera_target: ResMut<CameraTarget>, time: Res<Time>) {
    let rand_x = (rand::random::<f32>() * 200.0) - 100.0;
    let rand_y = (rand::random::<f32>() * 200.0) - 100.0;

    camera_target.position = Vec2::new(rand_x, rand_y);
    camera_target.start_time = time.seconds_since_startup();
}
